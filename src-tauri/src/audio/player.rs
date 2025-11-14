use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chrono::Utc;
use rodio::{decoder::DecoderBuilder, Decoder, OutputStream, OutputStreamBuilder, Sink};
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
    sink: Option<Arc<Sink>>,
    current: Option<PlaybackContext>,
    fade_task: Option<JoinHandle<()>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioError> {
        let stream = OutputStreamBuilder::open_default_stream().map_err(|err| match err {
            rodio::StreamError::NoDevice => AudioError::NoOutputDevice,
            other => AudioError::Stream(other),
        })?;

        Ok(Self {
            _stream: stream,
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

        let sink = Arc::new(Sink::connect_new(self._stream.mixer()));
        let decoder = Self::create_decoder(&path)?;

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

impl AudioPlayer {
    fn create_decoder(path: &Path) -> Result<Decoder<BufReader<File>>, AudioError> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let byte_len = metadata.len();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());
        let reader = BufReader::new(file);

        Self::build_decoder(reader, extension.as_deref(), Some(byte_len))
    }

    fn build_decoder<R>(
        reader: R,
        extension_hint: Option<&str>,
        byte_len: Option<u64>,
    ) -> Result<Decoder<R>, AudioError>
    where
        R: Read + Seek + Send + Sync + 'static,
    {
        let mut builder = DecoderBuilder::new().with_data(reader);

        if let Some(len) = byte_len {
            builder = builder.with_byte_len(len).with_seekable(true);
        }

        if let Some(hint) = extension_hint {
            builder = builder.with_hint(hint);
        }

        panic::catch_unwind(AssertUnwindSafe(|| builder.build()))
            .map_err(|_| AudioError::Decoder("failed to decode audio file".into()))?
            .map_err(|err| AudioError::Decoder(err.to_string()))
    }
}
