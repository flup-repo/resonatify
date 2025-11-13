use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chrono::Utc;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::Serialize;

use crate::audio::error::AudioError;
use crate::audio::validator::AudioFileMetadata;

#[derive(Debug, Clone, Serialize)]
pub struct PlaybackContext {
    pub file_path: String,
    pub metadata: AudioFileMetadata,
    pub started_at: String,
    pub volume: f32,
}

pub struct AudioPlayer {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    sink: Option<Arc<Sink>>,
    current: Option<PlaybackContext>,
    fade_task: Option<JoinHandle<()>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        let (stream, handle) = OutputStream::try_default().map_err(|err| match err {
            rodio::StreamError::NoDevice => AudioError::NoOutputDevice,
            other => AudioError::Stream(other),
        })?;

        Ok(Self {
            _stream: stream,
            handle,
            sink: None,
            current: None,
            fade_task: None,
        })
    }

    pub fn play(
        &mut self,
        path: PathBuf,
        metadata: AudioFileMetadata,
        volume: f32,
    ) -> Result<PlaybackContext, AudioError> {
        self.abort_fade();
        self.stop_immediate();

        let sink = Arc::new(Sink::try_new(&self.handle).map_err(AudioError::Sink)?);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).map_err(|err| AudioError::Decoder(err.to_string()))?;

        sink.set_volume(0.0);
        sink.append(decoder);
        sink.play();

        self.spawn_fade(
            Arc::clone(&sink),
            0.0,
            volume,
            Duration::from_millis(400),
            false,
        );

        let context = PlaybackContext {
            file_path: path.display().to_string(),
            metadata,
            started_at: Utc::now().to_rfc3339(),
            volume,
        };

        self.sink = Some(sink);
        self.current = Some(context.clone());

        Ok(context)
    }

    pub fn stop(&mut self, fade_duration: Duration) -> Option<PlaybackContext> {
        self.abort_fade();

        let previous = self.current.clone();

        if let Some(sink) = self.sink.take() {
            let current_volume = sink.volume();
            self.spawn_fade(sink, current_volume, 0.0, fade_duration, true);
        }

        self.current = None;
        previous
    }

    fn stop_immediate(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self.current = None;
    }

    fn abort_fade(&mut self) {
        if let Some(handle) = self.fade_task.take() {
            let _ = handle.join();
        }
    }

    fn spawn_fade(
        &mut self,
        sink: Arc<Sink>,
        start_volume: f32,
        end_volume: f32,
        duration: Duration,
        stop_after: bool,
    ) {
        let steps = 12u32;
        let step_duration = if steps == 0 {
            Duration::from_millis(0)
        } else {
            duration
                .checked_div(steps)
                .unwrap_or_else(|| Duration::from_millis(0))
        };

        let task = thread::spawn(move || {
            if steps == 0 {
                sink.set_volume(end_volume.clamp(0.0, 1.0));
            } else {
                for step in 0..=steps {
                    let t = step as f32 / steps as f32;
                    let volume = start_volume + (end_volume - start_volume) * t;
                    sink.set_volume(volume.clamp(0.0, 1.0));

                    if step < steps {
                        thread::sleep(step_duration);
                    }
                }
            }

            if stop_after {
                sink.stop();
            }
        });

        self.fade_task = Some(task);
    }
}
