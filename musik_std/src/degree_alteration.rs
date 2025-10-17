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
    /// assert_eq!(DegreeAlteration::from_u8(1), DegreeAlteration::None);
    /// assert_eq!(DegreeAlteration::from_u8(2), DegreeAlteration::Flat);
    /// assert_eq!(DegreeAlteration::from_u8(3), DegreeAlteration::Sharp);
    /// ```
    ///
    /// # Panics
    /// Panics if the value is not 1, 2, or 3.
    pub const fn from_u8(value: u8) -> Self {
        match value {
            1 => DegreeAlteration::None,
            2 => DegreeAlteration::Flat,
            3 => DegreeAlteration::Sharp,
            _ => panic!("Invalid u8 value for DegreeAlteration"),
        }
    }

    /// Convert this `DegreeAlteration` to a u8 value.
    ///
    /// - `None` converts to `1`
    /// - `Flat` converts to `2`
    /// - `Sharp` converts to `3`
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAlteration;
    ///
    /// assert_eq!(DegreeAlteration::None.to_u8(), 1);
    /// assert_eq!(DegreeAlteration::Flat.to_u8(), 2);
    /// assert_eq!(DegreeAlteration::Sharp.to_u8(), 3);
    /// ```
    pub const fn to_u8(&self) -> u8 {
        match self {
            DegreeAlteration::None => 1,
            DegreeAlteration::Flat => 2,
            DegreeAlteration::Sharp => 3,
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
    fn test_const_from_to_u8_methods() {
        // Test const from_u8 method
        const NONE_ALT: DegreeAlteration = DegreeAlteration::from_u8(1);
        const FLAT_ALT: DegreeAlteration = DegreeAlteration::from_u8(2);
        const SHARP_ALT: DegreeAlteration = DegreeAlteration::from_u8(3);

        assert_eq!(NONE_ALT, DegreeAlteration::None);
        assert_eq!(FLAT_ALT, DegreeAlteration::Flat);
        assert_eq!(SHARP_ALT, DegreeAlteration::Sharp);

        // Test const to_u8 method
        const NONE_VALUE: u8 = DegreeAlteration::None.to_u8();
        const FLAT_VALUE: u8 = DegreeAlteration::Flat.to_u8();
        const SHARP_VALUE: u8 = DegreeAlteration::Sharp.to_u8();

        assert_eq!(NONE_VALUE, 1);
        assert_eq!(FLAT_VALUE, 2);
        assert_eq!(SHARP_VALUE, 3);

        // Test round trip with const methods
        assert_eq!(
            DegreeAlteration::from_u8(DegreeAlteration::None.to_u8()),
            DegreeAlteration::None
        );
        assert_eq!(
            DegreeAlteration::from_u8(DegreeAlteration::Flat.to_u8()),
            DegreeAlteration::Flat
        );
        assert_eq!(
            DegreeAlteration::from_u8(DegreeAlteration::Sharp.to_u8()),
            DegreeAlteration::Sharp
        );
    }
}
