use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tokio::sync::oneshot;

use crate::audio::error::{AudioError, AudioValidationError};
use crate::audio::player::{AudioPlayer, PlaybackContext};
use crate::audio::validator::{AudioFileMetadata, AudioValidator};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub is_playing: bool,
    pub current: Option<PlaybackContext>,
}

impl PlaybackState {
    fn idle() -> Self {
        Self {
            is_playing: false,
            current: None,
        }
    }
}

enum AudioCommand {
    Play {
        path: PathBuf,
        metadata: AudioFileMetadata,
        volume: f32,
        fade_duration: Duration,
        respond_to: oneshot::Sender<Result<PlaybackState, AudioError>>,
    },
    Stop {
        respond_to: oneshot::Sender<Result<PlaybackState, AudioError>>,
    },
}

#[derive(Clone)]
pub struct AudioService {
    validator: AudioValidator,
    commands: Sender<AudioCommand>,
    state: Arc<RwLock<PlaybackState>>,
}

impl AudioService {
    pub fn new() -> Result<Self, AudioError> {
        Self::with_validator(AudioValidator::default())
    }

    pub fn with_validator(validator: AudioValidator) -> Result<Self, AudioError> {
        let (command_tx, command_rx) = mpsc::channel();
        let state = Arc::new(RwLock::new(PlaybackState::idle()));
        let (init_tx, init_rx) = mpsc::sync_channel(1);
        let state_for_thread = Arc::clone(&state);

        thread::Builder::new()
            .name("audio-service".into())
            .spawn(move || {
                let mut player = match AudioPlayer::new() {
                    Ok(player) => {
                        let _ = init_tx.send(Ok(()));
                        player
                    }
                    Err(err) => {
                        let _ = init_tx.send(Err(err));
                        return;
                    }
                };

                const FADE_OUT: Duration = Duration::from_millis(300);

                while let Ok(command) = command_rx.recv() {
                    match command {
                        AudioCommand::Play {
                            path,
                            metadata,
                            volume,
                            fade_duration,
                            respond_to,
                        } => {
                            let result = player.play(path, metadata, volume, fade_duration).map(|context| {
                                let new_state = PlaybackState {
                                    is_playing: true,
                                    current: Some(context.clone()),
                                };

                                if let Ok(mut guard) = state_for_thread.write() {
                                    *guard = new_state.clone();
                                }

                                new_state
                            });

                            let _ = respond_to.send(result);
                        }
                        AudioCommand::Stop { respond_to } => {
                            player.stop(FADE_OUT);

                            let new_state = PlaybackState::idle();
                            if let Ok(mut guard) = state_for_thread.write() {
                                *guard = new_state.clone();
                            }

                            let _ = respond_to.send(Ok(new_state));
                        }
                    }
                }
            })
            .map_err(AudioError::Io)?;

        match init_rx.recv() {
            Ok(Ok(())) => Ok(Self {
                validator,
                commands: command_tx,
                state,
            }),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(AudioError::EngineUnavailable),
        }
    }

    pub async fn validate<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<AudioFileMetadata, AudioValidationError> {
        self.validator.validate(path)
    }

    pub async fn play<P: Into<PathBuf>>(
        &self,
        path: P,
        volume_percent: u8,
    ) -> Result<PlaybackState, AudioError> {
        self.play_with_fade(path, volume_percent, Duration::from_millis(400)).await
    }

    pub async fn play_with_fade<P: Into<PathBuf>>(
        &self,
        path: P,
        volume_percent: u8,
        fade_duration: Duration,
    ) -> Result<PlaybackState, AudioError> {
        let path = path.into();
        let metadata = self.validator.validate(&path)?;
        let volume = Self::volume_from_percent(volume_percent);
        let (tx, rx) = oneshot::channel();

        self.commands
            .send(AudioCommand::Play {
                path,
                metadata,
                volume,
                fade_duration,
                respond_to: tx,
            })
            .map_err(|_| AudioError::EngineUnavailable)?;

        rx.await.map_err(|_| AudioError::EngineUnavailable)?
    }

    pub async fn stop(&self) -> Result<PlaybackState, AudioError> {
        let (tx, rx) = oneshot::channel();
        self.commands
            .send(AudioCommand::Stop { respond_to: tx })
            .map_err(|_| AudioError::EngineUnavailable)?;

        rx.await.map_err(|_| AudioError::EngineUnavailable)?
    }

    pub async fn status(&self) -> PlaybackState {
        self.state
            .read()
            .map(|state| state.clone())
            .unwrap_or_else(|_| PlaybackState::idle())
    }

    fn volume_from_percent(volume_percent: u8) -> f32 {
        (volume_percent as f32 / 100.0).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_service() -> Result<AudioService, AudioError> {
        AudioService::with_validator(AudioValidator::new(20 * 1024 * 1024))
    }

    fn write_sample_wav(path: &Path) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44_100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec).unwrap();
        for _ in 0..(spec.sample_rate / 10) {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();
    }

    #[tokio::test]
    async fn play_and_stop_updates_state() {
        let service = match create_service() {
            Ok(service) => service,
            Err(AudioError::NoOutputDevice) => return,
            Err(other) => panic!("unexpected error initialising audio service: {other:?}"),
        };

        let temp_dir = tempfile::TempDir::new().unwrap();
        let file_path = temp_dir.path().join("service.wav");
        write_sample_wav(&file_path);

        let state = service.play(file_path.clone(), 80).await.unwrap();
        assert!(state.is_playing);
        assert!(state.current.is_some());

        let stopped = service.stop().await.unwrap();
        assert!(!stopped.is_playing);
    }

    #[tokio::test]
    async fn status_returns_idle_when_not_playing() {
        let service = match create_service() {
            Ok(service) => service,
            Err(AudioError::NoOutputDevice) => return,
            Err(other) => panic!("unexpected error initialising audio service: {other:?}"),
        };

        let status = service.status().await;
        assert!(!status.is_playing);
        assert!(status.current.is_none());
    }
}
