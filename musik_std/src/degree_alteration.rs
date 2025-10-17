//! Degree alterations for representing sharp and flat modifications.
//!
//! This module provides the `DegreeAlteration` enum for representing musical alterations
//! that modify scale degrees, chord tones, and other musical intervals.

use std::fmt;

/// Represents a musical alteration that modifies a degree.
///
/// Used to indicate whether a musical degree is sharpened or flattened
/// from its natural position. This enum is useful for chord construction,
/// scale analysis, and harmonic notation.
///
/// # Examples
/// ```
/// use musik_std::DegreeAlteration;
///
/// let sharp = DegreeAlteration::Sharp;
/// let flat = DegreeAlteration::Flat;
///
/// assert_eq!(sharp.symbol(), "♯");
/// assert_eq!(flat.symbol(), "♭");
/// assert_eq!(sharp.semitone_offset(), 1);
/// assert_eq!(flat.semitone_offset(), -1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DegreeAlteration {
    /// No alteration - natural degree
    None,
    /// Flat alteration (♭) - lowers the degree by a semitone
    Flat,
    /// Sharp alteration (♯) - raises the degree by a semitone
    Sharp,
}

impl DegreeAlteration {
    /// Get the symbol representation of the alteration.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::Sharp.symbol(), "♯");
    /// assert_eq!(DegreeAlteration::Flat.symbol(), "♭");
    /// ```
    pub const fn symbol(&self) -> &'static str {
        match self {
            DegreeAlteration::None => "",
            DegreeAlteration::Sharp => "♯",
            DegreeAlteration::Flat => "♭",
        }
    }

    /// Get the semitone offset applied by this alteration.
    ///
    /// Returns the number of semitones to add to a natural degree.
    /// Sharp adds +1 semitone, flat adds -1 semitone.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::Sharp.semitone_offset(), 1);
    /// assert_eq!(DegreeAlteration::Flat.semitone_offset(), -1);
    ///
    /// // Usage in calculations
    /// let natural_third = 4; // E in C major (4 semitones)
    /// let flat_third = natural_third + DegreeAlteration::Flat.semitone_offset();
    /// assert_eq!(flat_third, 3); // Eb (3 semitones)
    /// ```
    pub const fn semitone_offset(&self) -> i8 {
        match self {
            DegreeAlteration::None => 0,
            DegreeAlteration::Sharp => 1,
            DegreeAlteration::Flat => -1,
        }
    }

    /// Check if this is a sharp alteration.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert!(DegreeAlteration::Sharp.is_sharp());
    /// assert!(!DegreeAlteration::Flat.is_sharp());
    /// ```
    pub const fn is_sharp(&self) -> bool {
        matches!(self, DegreeAlteration::Sharp)
    }

    /// Check if this is a flat alteration.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert!(!DegreeAlteration::Sharp.is_flat());
    /// assert!(DegreeAlteration::Flat.is_flat());
    /// ```
    pub const fn is_flat(&self) -> bool {
        matches!(self, DegreeAlteration::Flat)
    }

    /// Check if this is no alteration (natural).
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert!(DegreeAlteration::None.is_none());
    /// assert!(!DegreeAlteration::Sharp.is_none());
    /// assert!(!DegreeAlteration::Flat.is_none());
    /// ```
    pub const fn is_none(&self) -> bool {
        matches!(self, DegreeAlteration::None)
    }

    /// Get the opposite alteration.
    ///
    /// Sharp becomes flat, flat becomes sharp. Natural stays natural.
    /// Useful for enharmonic equivalence and musical transformations.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::Sharp.opposite(), DegreeAlteration::Flat);
    /// assert_eq!(DegreeAlteration::Flat.opposite(), DegreeAlteration::Sharp);
    /// assert_eq!(DegreeAlteration::None.opposite(), DegreeAlteration::None);
    /// ```
    pub const fn opposite(&self) -> DegreeAlteration {
        match self {
            DegreeAlteration::None => DegreeAlteration::None,
            DegreeAlteration::Sharp => DegreeAlteration::Flat,
            DegreeAlteration::Flat => DegreeAlteration::Sharp,
        }
    }
}

impl fmt::Display for DegreeAlteration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl Default for DegreeAlteration {
    /// Returns `DegreeAlteration::None` as the default (natural/unaltered).
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::default(), DegreeAlteration::None);
    /// ```
    fn default() -> Self {
        DegreeAlteration::None
    }
}

impl From<u8> for DegreeAlteration {
    /// Convert a u8 value to a `DegreeAlteration`.
    ///
    /// - Value `1` converts to `None` (natural)
    /// - Value `2` converts to `Flat`
    /// - Value `3` converts to `Sharp`
    /// - Any other value panics
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::from(1), DegreeAlteration::None);
    /// assert_eq!(DegreeAlteration::from(2), DegreeAlteration::Flat);
    /// assert_eq!(DegreeAlteration::from(3), DegreeAlteration::Sharp);
    /// ```
    ///
    /// # Panics
    /// Panics if the value is not 1, 2, or 3.
    fn from(value: u8) -> Self {
        match value {
            1 => DegreeAlteration::None,
            2 => DegreeAlteration::Flat,
            3 => DegreeAlteration::Sharp,
            _ => panic!(
                "Invalid u8 value for DegreeAlteration: {}. Expected 1 (None), 2 (Flat), or 3 (Sharp)",
                value
            ),
        }
    }
}

impl From<DegreeAlteration> for u8 {
    /// Convert a `DegreeAlteration` to a u8 value.
    ///
    /// - `None` converts to `1`
    /// - `Flat` converts to `2`
    /// - `Sharp` converts to `3`
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(u8::from(DegreeAlteration::None), 1);
    /// assert_eq!(u8::from(DegreeAlteration::Flat), 2);
    /// assert_eq!(u8::from(DegreeAlteration::Sharp), 3);
    /// ```
    fn from(alteration: DegreeAlteration) -> Self {
        match alteration {
            DegreeAlteration::None => 1,
            DegreeAlteration::Flat => 2,
            DegreeAlteration::Sharp => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_alteration_creation() {
        let sharp = DegreeAlteration::Sharp;
        let flat = DegreeAlteration::Flat;

        assert!(sharp.is_sharp());
        assert!(!sharp.is_flat());
        assert!(!flat.is_sharp());
        assert!(flat.is_flat());
    }

    #[test]
    fn test_symbols() {
        assert_eq!(DegreeAlteration::Sharp.symbol(), "♯");
        assert_eq!(DegreeAlteration::Flat.symbol(), "♭");
    }

    #[test]
    fn test_semitone_offsets() {
        assert_eq!(DegreeAlteration::Sharp.semitone_offset(), 1);
        assert_eq!(DegreeAlteration::Flat.semitone_offset(), -1);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(DegreeAlteration::Sharp.opposite(), DegreeAlteration::Flat);
        assert_eq!(DegreeAlteration::Flat.opposite(), DegreeAlteration::Sharp);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DegreeAlteration::Sharp), "♯");
        assert_eq!(format!("{}", DegreeAlteration::Flat), "♭");
    }

    #[test]
    fn test_equality_and_ordering() {
        let sharp1 = DegreeAlteration::Sharp;
        let sharp2 = DegreeAlteration::Sharp;
        let flat = DegreeAlteration::Flat;

        assert_eq!(sharp1, sharp2);
        assert_ne!(sharp1, flat);

        // Test ordering (Flat < Sharp based on enum order)
        assert!(flat < sharp1);
        assert!(sharp1 > flat);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DegreeAlteration::Sharp);
        set.insert(DegreeAlteration::Flat);
        set.insert(DegreeAlteration::Sharp); // Duplicate

        assert_eq!(set.len(), 2); // Only two unique values
        assert!(set.contains(&DegreeAlteration::Sharp));
        assert!(set.contains(&DegreeAlteration::Flat));
    }

    #[test]
    fn test_default() {
        assert_eq!(DegreeAlteration::default(), DegreeAlteration::None);
    }

    #[test]
    fn test_musical_calculations() {
        // Test practical usage in musical calculations
        let natural_third = 4; // E in C major
        let flat_third = natural_third + DegreeAlteration::Flat.semitone_offset();
        let sharp_third = natural_third + DegreeAlteration::Sharp.semitone_offset();

        assert_eq!(flat_third, 3); // Eb
        assert_eq!(sharp_third, 5); // E# (same as F)

        // Test with seventh
        let natural_seventh = 11; // B in C major
        let flat_seventh = natural_seventh + DegreeAlteration::Flat.semitone_offset();

        assert_eq!(flat_seventh, 10); // Bb
    }

    #[test]
    fn test_const_functions() {
        // Test that const functions work at compile time
        const SHARP: DegreeAlteration = DegreeAlteration::Sharp;
        const FLAT: DegreeAlteration = DegreeAlteration::Flat;

        assert_eq!(SHARP.symbol(), "♯");
        assert_eq!(FLAT.symbol(), "♭");
        assert_eq!(SHARP.semitone_offset(), 1);
        assert_eq!(FLAT.semitone_offset(), -1);
        assert!(SHARP.is_sharp());
        assert!(FLAT.is_flat());
        assert_eq!(SHARP.opposite(), FLAT);
    }

    #[test]
    fn test_from_u8() {
        // Test valid conversions
        assert_eq!(DegreeAlteration::from(1), DegreeAlteration::None);
        assert_eq!(DegreeAlteration::from(2), DegreeAlteration::Flat);
        assert_eq!(DegreeAlteration::from(3), DegreeAlteration::Sharp);

        // Test using the Into trait (automatic conversion)
        let none: DegreeAlteration = 1.into();
        let flat: DegreeAlteration = 2.into();
        let sharp: DegreeAlteration = 3.into();
        assert_eq!(none, DegreeAlteration::None);
        assert_eq!(flat, DegreeAlteration::Flat);
        assert_eq!(sharp, DegreeAlteration::Sharp);
    }

    #[test]
    #[should_panic(
        expected = "Invalid u8 value for DegreeAlteration: 0. Expected 1 (None), 2 (Flat), or 3 (Sharp)"
    )]
    fn test_from_u8_invalid_low() {
        let _ = DegreeAlteration::from(0);
    }

    #[test]
    #[should_panic(
        expected = "Invalid u8 value for DegreeAlteration: 4. Expected 1 (None), 2 (Flat), or 3 (Sharp)"
    )]
    fn test_from_u8_invalid_high() {
        let _ = DegreeAlteration::from(4);
    }

    #[test]
    fn test_from_degree_alteration_to_u8() {
        // Test direct conversion
        assert_eq!(u8::from(DegreeAlteration::None), 1);
        assert_eq!(u8::from(DegreeAlteration::Flat), 2);
        assert_eq!(u8::from(DegreeAlteration::Sharp), 3);

        // Test using the Into trait (automatic conversion)
        let none_value: u8 = DegreeAlteration::None.into();
        let flat_value: u8 = DegreeAlteration::Flat.into();
        let sharp_value: u8 = DegreeAlteration::Sharp.into();
        assert_eq!(none_value, 1);
        assert_eq!(flat_value, 2);
        assert_eq!(sharp_value, 3);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting u8 -> DegreeAlteration -> u8 is identity
        let none_original = 1u8;
        let flat_original = 2u8;
        let sharp_original = 3u8;

        let none_round_trip: u8 = DegreeAlteration::from(none_original).into();
        let flat_round_trip: u8 = DegreeAlteration::from(flat_original).into();
        let sharp_round_trip: u8 = DegreeAlteration::from(sharp_original).into();

        assert_eq!(none_original, none_round_trip);
        assert_eq!(flat_original, flat_round_trip);
        assert_eq!(sharp_original, sharp_round_trip);

        // Test that converting DegreeAlteration -> u8 -> DegreeAlteration is identity
        let none_alt = DegreeAlteration::None;
        let flat_alt = DegreeAlteration::Flat;
        let sharp_alt = DegreeAlteration::Sharp;

        let none_alt_round_trip = DegreeAlteration::from(u8::from(none_alt));
        let flat_alt_round_trip = DegreeAlteration::from(u8::from(flat_alt));
        let sharp_alt_round_trip = DegreeAlteration::from(u8::from(sharp_alt));

        assert_eq!(none_alt, none_alt_round_trip);
        assert_eq!(flat_alt, flat_alt_round_trip);
        assert_eq!(sharp_alt, sharp_alt_round_trip);
    }
}
