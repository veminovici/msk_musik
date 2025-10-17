//! # musik_std
//!
//! A comprehensive music processing library providing fundamental music theory
//! concepts, audio processing capabilities, and utilities for music applications.
//!
//! ## Features
//!
//! - **Music Theory Primitives**: Notes, intervals, scales, and chords
//! - **Musical Traits**: Comprehensive trait system for musical operations
//! - **Audio Processing**: Sample buffers, waveform generation, and audio utilities
//! - **MIDI Support**: MIDI message handling, note/frequency conversion, and protocol utilities
//! - **Digital Signal Processing**: Audio processing foundations
//!
//! ## Musical Traits
//!
//! The library provides several powerful traits for working with musical notes:
//!
//! - [`ChromaticNote`]: Core chromatic note functionality
//! - [`TransposableNote`]: Transposition and interval operations
//! - [`EnharmonicNote`]: Enharmonic equivalent handling
//! - [`ScaleDegree`]: Scale degree relationships
//! - [`FrequencyNote`]: Frequency and MIDI conversions
//! - [`CircleOfFifths`]: Circle of fifths relationships
//! - [`MusicalScale`]: Scale operations and note enumeration
//!
//! ## Examples
//!
//! ### Basic Note Operations
//! ```rust
//! use musik_std::{Note, ChromaticNote, TransposableNote};
//!
//! let note = Note::C;
//! println!("Note: {} (semitone: {})", note.name(), note.semitone());
//!
//! // Transpose up a perfect fifth
//! let fifth = note.transpose(7);
//! assert_eq!(fifth, Note::G);
//! ```
//!
//! ### Scale Analysis
//! ```rust
//! use musik_std::{Note, ScaleDegree, EnharmonicNote};
//!
//! let note = Note::E;
//! let tonic = Note::C;
//!
//! // Check if note is in C major scale
//! if note.is_in_major_scale(&tonic) {
//!     let degree = note.degree_in_major(&tonic).unwrap();
//!     println!("E is degree {} in C major", degree); // degree 3
//! }
//!
//! // Get enharmonic equivalents
//! let equivalents = Note::CSharp.enharmonic_equivalents();
//! println!("C# equivalents: {:?}", equivalents); // ["C#", "Db"]
//! ```
//!
//! ### Frequency and MIDI
//! ```rust
//! use musik_std::{Note, FrequencyNote};
//!
//! let a4 = Note::A;
//! let frequency = a4.frequency(4); // A4 = 440 Hz
//! let midi_note = a4.midi_number(4); // A4 = MIDI 69
//!
//! println!("A4: {:.2} Hz, MIDI {}", frequency, midi_note.unwrap());
//! ```
//!
//! ### Musical Scales
//! ```rust
//! use musik_std::{Note, Scale, ScaleType, MusicalScale};
//!
//! // Create a C major scale
//! let c_major = Scale::new(Note::C, ScaleType::Major);
//! println!("Scale: {}", c_major.name()); // "C Major"
//!
//! // Get all notes in the scale
//! let notes = c_major.notes();
//! println!("Notes: {:?}", notes); // [C, D, E, F, G, A, B]
//!
//! // Check scale degrees
//! assert_eq!(c_major.degree_of(&Note::E), Some(3)); // E is the 3rd degree
//! assert_eq!(c_major.note_at_degree(5), Some(Note::G)); // 5th degree is G
//!
//! // Check if notes are in the scale
//! assert!(c_major.contains(&Note::F)); // F is in C major
//! assert!(!c_major.contains(&Note::FSharp)); // F# is not in C major
//!
//! // Work with different scale types
//! let a_minor = Scale::new(Note::A, ScaleType::NaturalMinor);
//! let blues_scale = Scale::new(Note::E, ScaleType::BluesScale);
//! let pentatonic = Scale::new(Note::G, ScaleType::PentatonicMajor);
//! ```

pub mod audio;
pub mod midi;
pub mod theory;

pub use theory::{
    ChromaticNote, CircleOfFifths, EnharmonicNote, FrequencyNote, Interval, MusicalScale, Note,
    Scale, ScaleDegree, ScaleType, TransposableNote,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // Test that version is a valid semver format (e.g., "0.1.0")
        assert!(VERSION.len() >= 5); // At least "0.0.0"
        assert!(VERSION.contains('.')); // Should contain dots

        // Test that it parses as valid semver parts
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            parts.len() >= 3,
            "Version should have at least major.minor.patch"
        );

        // Each part should be numeric
        for part in parts.iter().take(3) {
            assert!(
                part.parse::<u32>().is_ok(),
                "Version part '{}' should be numeric",
                part
            );
        }
    }
}
