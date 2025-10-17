//! # musik_std
//!
//! A standard music library providing common utilities and algorithms for music applications.
//!
//! This library is designed to complement the core music theory functionality with
//! practical utilities, algorithms, and common patterns used in music software development.
//!
//! ## Features
//!
//! Current features include:
//!
//! - **Semitone**: Basic musical interval representation
//! - **Note**: Musical note abstraction containing semitone information
//! - **Octave**: Musical octave position representation
//! - **DegreeAlteration**: Musical alterations (sharp/flat) for degree modifications
//! - **FormulaDegree**: Extended harmony degrees for chord construction (9ths, 11ths, 13ths, alterations)
//! - **ChordFormula**: Bit-packed chord degree representation for efficient chord storage and analysis
//! - **ScaleFormula**: Bit-flag representation of scale patterns and formulas
//! - **Prelude**: Convenient imports for commonly used types and traits
//!
//! Future features may include:
//!
//! - **Music Algorithms**: Pattern recognition, harmony analysis, rhythm processing
//! - **Utilities**: File I/O helpers, format conversion, data structures
//! - **Common Patterns**: Builder patterns, iterators, and convenience functions
//! - **Integration Helpers**: Bridge functions between different music representations
//!
//! ## Usage
//!
//! ### Using the prelude (recommended)
//!
//! ```rust
//! use musik_std::prelude::*;
//!
//! let semitone = Semitone::from(7u8);
//! let value: u8 = semitone.into();
//! assert_eq!(value, 7);
//!
//! // Extended harmony with FormulaDegree
//! let ninth = FormulaDegree::natural(9);
//! let flat_ninth = FormulaDegree::flat(9);
//! assert_eq!(ninth.to_semitone_offset(), Some(2)); // 9th = 2nd
//! assert_eq!(flat_ninth.to_semitone_offset(), Some(1)); // ♭9th = ♭2nd
//!
//! // Degree alterations for harmonic analysis
//! let sharp = DegreeAlteration::Sharp;
//! let flat = DegreeAlteration::Flat;
//! assert_eq!(sharp.semitone_offset(), 1);
//! assert_eq!(flat.symbol(), "♭");
//!
//! // Chord formulas with bit-packed degrees
//! let minor_seventh = ChordFormula::minor_seventh();
//! assert!(minor_seventh.has_degree(1, DegreeAlteration::None)); // Root
//! assert!(minor_seventh.has_degree(3, DegreeAlteration::Flat)); // Minor 3rd
//! assert!(minor_seventh.has_degree(5, DegreeAlteration::None)); // Perfect 5th
//! assert!(minor_seventh.has_degree(7, DegreeAlteration::Flat)); // Minor 7th
//!
//! // Jazz chord construction
//! let dom7_sharp9 = ChordFormula::dominant_seventh_sharp_ninth();
//! let degrees = dom7_sharp9.degrees();
//! println!("Dom7♯9 contains: {}", dom7_sharp9); // "1 3 5 ♭7 ♯9"
//!
//! // Scale formulas with bit flags
//! let major = ScaleFormula::major();
//! let minor = ScaleFormula::minor();
//! assert_eq!(major.note_count(), 7);
//! assert!(major.contains_semitone(4)); // Major 3rd
//! assert!(!minor.contains_semitone(4)); // No major 3rd in minor
//! ```
//!
//! ### Using specific imports
//!
//! ```rust
//! use musik_std::{Semitone, FormulaDegree, DegreeAlteration};
//!
//! let semitone = Semitone::from(5u8);
//! assert_eq!(u8::from(semitone), 5);
//!
//! // Jazz chord extensions
//! let sharp_eleven = FormulaDegree::sharp(11);
//! assert_eq!(sharp_eleven.symbol(), "♯11");
//! assert!(sharp_eleven.is_tension());
//!
//! // Alteration calculations
//! let flat_alteration = DegreeAlteration::Flat;
//! let natural_third = 4; // E in C major
//! let flat_third = natural_third + flat_alteration.semitone_offset();
//! assert_eq!(flat_third, 3); // Eb
//! ```

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Module declarations
mod chord_formula;
mod degree_alteration;
mod formula_degree;
mod note;
mod octave;
pub mod prelude;
mod scale_formula;
mod semitone;

// Re-exports
pub use chord_formula::ChordFormula;
pub use degree_alteration::DegreeAlteration;
pub use formula_degree::FormulaDegree;
pub use note::Note;
pub use octave::Octave;
pub use scale_formula::ScaleFormula;
pub use semitone::{Semitone, SEMITONES_IN_OCTAVE};

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
