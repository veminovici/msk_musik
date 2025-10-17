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
//! - **FormulaDegree**: Extended harmony degrees for chord construction (9ths, 11ths, 13ths, alterations)
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
//! use musik_std::{Semitone, FormulaDegree};
//!
//! let semitone = Semitone::from(5u8);
//! assert_eq!(u8::from(semitone), 5);
//!
//! // Jazz chord extensions
//! let sharp_eleven = FormulaDegree::sharp(11);
//! assert_eq!(sharp_eleven.symbol(), "♯11");
//! assert!(sharp_eleven.is_tension());
//! ```

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Module declarations
mod formula_degree;
mod note;
mod octave;
pub mod prelude;
mod scale_formula;
mod semitone;

// Re-exports
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
