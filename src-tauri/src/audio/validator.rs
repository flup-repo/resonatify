use std::fs;
use std::io::{BufReader, Read, Seek};
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

use rodio::{decoder::DecoderBuilder, Decoder, Source};
use serde::Serialize;

use crate::audio::error::AudioValidationError;

const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "wav", "flac", "ogg", "oga", "m4a", "aac"];
const DEFAULT_MAX_FILE_SIZE_BYTES: u64 = 512 * 1024 * 1024; // 512MB

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioFileMetadata {
    pub path: String,
    pub file_name: String,
    pub extension: String,
    pub duration_ms: Option<u64>,
    pub sample_rate: u32,
    pub channels: u16,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct AudioValidator {
    max_file_size_bytes: u64,
}

impl Default for AudioValidator {
    fn default() -> Self {
        Self {
            max_file_size_bytes: DEFAULT_MAX_FILE_SIZE_BYTES,
        }
    }
}

impl AudioValidator {
    pub fn new(max_file_size_bytes: u64) -> Self {
        Self {
            max_file_size_bytes,
        }
    }

    pub fn validate<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<AudioFileMetadata, AudioValidationError> {
        let path = path.as_ref();
        let canonical_path = path.canonicalize().map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => {
                AudioValidationError::NotFound(path.display().to_string())
            }
            _ => AudioValidationError::Io(err),
        })?;

        let metadata = fs::metadata(&canonical_path).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => {
                AudioValidationError::NotFound(path.display().to_string())
            }
            _ => AudioValidationError::Io(err),
        })?;

        if !metadata.is_file() {
            return Err(AudioValidationError::Decode(
                "path does not point to a file".into(),
            ));
        }

        let size_bytes = metadata.len();
        if size_bytes > self.max_file_size_bytes {
            return Err(AudioValidationError::FileTooLarge {
                path: canonical_path.display().to_string(),
                file_bytes: size_bytes,
                max_bytes: self.max_file_size_bytes,
            });
        }

        let extension = canonical_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .ok_or_else(|| AudioValidationError::UnsupportedFormat {
                extension: String::new(),
            })?;

        if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(AudioValidationError::UnsupportedFormat { extension });
        }

        let file = std::fs::File::open(&canonical_path)?;
        let reader = BufReader::new(file);
        let decoder = Self::build_decoder(reader)?;

        let duration_ms = decoder
            .total_duration()
            .map(|duration| duration.as_millis() as u64);

        let sample_rate = decoder.sample_rate();
        let channels = decoder.channels();

        let file_name = canonical_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();

        Ok(AudioFileMetadata {
            path: canonical_path.display().to_string(),
            file_name,
            extension,
            duration_ms,
            sample_rate,
            channels,
            size_bytes,
        })
    }
}

impl AudioValidator {
    fn build_decoder<R>(reader: R) -> Result<Decoder<R>, AudioValidationError>
    where
        R: Read + Seek + Send + Sync + 'static,
    {
        let builder = DecoderBuilder::new().with_data(reader);

        panic::catch_unwind(AssertUnwindSafe(|| builder.build()))
            .map_err(|_| AudioValidationError::Decode("failed to decode audio file".into()))?
            .map_err(|err| AudioValidationError::Decode(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_silent_wav(dir: &tempfile::TempDir, name: &str, duration_ms: u64) -> PathBuf {
        let file_path = dir.path().join(name);
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44_100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let total_samples = (spec.sample_rate as u64 * duration_ms) / 1000;

        let mut writer = hound::WavWriter::create(&file_path, spec).unwrap();
        for _ in 0..total_samples {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();

        file_path
    }

    #[test]
    fn validates_supported_file() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let wav_path = create_silent_wav(&temp_dir, "test.wav", 500);

        let validator = AudioValidator::default();
        let metadata = validator.validate(&wav_path).unwrap();

        assert_eq!(metadata.file_name, "test.wav");
        assert_eq!(metadata.extension, "wav");
        assert!(metadata.duration_ms.unwrap_or(0) > 0);
        assert_eq!(metadata.sample_rate, 44_100);
        assert_eq!(metadata.channels, 1);
    }

    #[test]
    fn rejects_unsupported_extension() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let txt_path = temp_dir.path().join("notes.txt");
        std::fs::write(&txt_path, b"hello").unwrap();

        let validator = AudioValidator::default();
        let err = validator.validate(&txt_path).unwrap_err();

        match err {
            AudioValidationError::UnsupportedFormat { .. } => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_too_large_file() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let wav_path = create_silent_wav(&temp_dir, "big.wav", 100);

        let validator = AudioValidator::new(1);
        let err = validator.validate(&wav_path).unwrap_err();

        match err {
            AudioValidationError::FileTooLarge { .. } => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_missing_file() {
        let validator = AudioValidator::default();
        let err = validator
            .validate(Path::new("/tmp/non-existent-file.wav"))
            .unwrap_err();

        match err {
            AudioValidationError::NotFound(_) => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
