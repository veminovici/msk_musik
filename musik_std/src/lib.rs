//! # musik_std
//!
//! A comprehensive music processing library providing fundamental music theory
//! concepts, audio processing capabilities, and utilities for music applications.
//!
//! ## Features
//!
//! - Music theory primitives (notes, scales, chords)
//! - Audio processing utilities
//! - MIDI support
//! - Digital signal processing
//!
//! ## Examples
//!
//! ```rust
//! use musik_std::Note;
//!
//! let note = Note::C;
//! println!("Note: {}", note);
//! ```

pub mod audio;
pub mod midi;
pub mod theory;

pub use theory::Note;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}