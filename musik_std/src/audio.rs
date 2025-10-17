//! Audio processing utilities and types

/// Sample rate type for audio processing
pub type SampleRate = u32;

/// Audio sample type
pub type Sample = f32;

/// Common sample rates
pub mod sample_rates {
    use super::SampleRate;
    
    pub const CD_QUALITY: SampleRate = 44100;
    pub const DVD_QUALITY: SampleRate = 48000;
    pub const HIGH_DEFINITION: SampleRate = 96000;
    pub const ULTRA_HIGH: SampleRate = 192000;
}

/// Audio buffer for processing samples
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    samples: Vec<Sample>,
    sample_rate: SampleRate,
    channels: usize,
}

impl AudioBuffer {
    /// Create a new audio buffer
    pub fn new(sample_rate: SampleRate, channels: usize) -> Self {
        Self {
            samples: Vec::new(),
            sample_rate,
            channels,
        }
    }

    /// Create an audio buffer with pre-allocated capacity
    pub fn with_capacity(sample_rate: SampleRate, channels: usize, capacity: usize) -> Self {
        Self {
            samples: Vec::with_capacity(capacity),
            sample_rate,
            channels,
        }
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// Get the number of channels
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Get the number of frames (samples per channel)
    pub fn frames(&self) -> usize {
        self.samples.len() / self.channels
    }

    /// Get a reference to the sample data
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }

    /// Get a mutable reference to the sample data
    pub fn samples_mut(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }

    /// Add samples to the buffer
    pub fn push_samples(&mut self, samples: &[Sample]) {
        self.samples.extend_from_slice(samples);
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.samples.clear();
    }

    /// Get the duration in seconds
    pub fn duration_secs(&self) -> f64 {
        self.frames() as f64 / self.sample_rate as f64
    }
}

/// Generate a sine wave
pub fn generate_sine_wave(
    frequency: f64,
    duration_secs: f64,
    sample_rate: SampleRate,
    amplitude: Sample,
) -> Vec<Sample> {
    let num_samples = (duration_secs * sample_rate as f64) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let sample = amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin() as Sample;
        samples.push(sample);
    }
    
    samples
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_buffer_creation() {
        let buffer = AudioBuffer::new(44100, 2);
        assert_eq!(buffer.sample_rate(), 44100);
        assert_eq!(buffer.channels(), 2);
        assert_eq!(buffer.frames(), 0);
    }

    #[test]
    fn test_audio_buffer_samples() {
        let mut buffer = AudioBuffer::new(44100, 1);
        let samples = vec![0.1, 0.2, 0.3];
        buffer.push_samples(&samples);
        
        assert_eq!(buffer.frames(), 3);
        assert_eq!(buffer.samples(), &[0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_sine_wave_generation() {
        let samples = generate_sine_wave(440.0, 1.0, 44100, 1.0);
        assert_eq!(samples.len(), 44100);
        // Check that the wave oscillates
        assert!(samples[0].abs() < 0.1); // Should be near 0 at t=0
    }

    #[test]
    fn test_duration_calculation() {
        let mut buffer = AudioBuffer::new(44100, 1);
        buffer.push_samples(&vec![0.0; 44100]); // 1 second of samples
        
        assert!((buffer.duration_secs() - 1.0).abs() < 0.001);
    }
}