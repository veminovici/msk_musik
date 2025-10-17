//! Degree accidentals for representing sharp and flat alterations.
//!
//! This module provides the `DegreeAccident` enum for representing musical accidentals
//! that modify scale degrees, chord tones, and other musical intervals.

use std::fmt;

/// Represents a musical accidental that modifies a degree.
///
/// Used to indicate whether a musical degree is sharpened or flattened
/// from its natural position. This enum is useful for chord construction,
/// scale analysis, and harmonic notation.
///
/// # Examples
/// ```
/// use musik_std::DegreeAccident;
///
/// let sharp = DegreeAccident::Sharp;
/// let flat = DegreeAccident::Flat;
///
/// assert_eq!(sharp.symbol(), "♯");
/// assert_eq!(flat.symbol(), "♭");
/// assert_eq!(sharp.semitone_offset(), 1);
/// assert_eq!(flat.semitone_offset(), -1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DegreeAccident {
    /// Sharp accidental (♯) - raises the degree by a semitone
    Sharp,
    /// Flat accidental (♭) - lowers the degree by a semitone
    Flat,
}

impl DegreeAccident {
    /// Get the symbol representation of the accidental.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert_eq!(DegreeAccident::Sharp.symbol(), "♯");
    /// assert_eq!(DegreeAccident::Flat.symbol(), "♭");
    /// ```
    pub const fn symbol(&self) -> &'static str {
        match self {
            DegreeAccident::Sharp => "♯",
            DegreeAccident::Flat => "♭",
        }
    }

    /// Get the semitone offset applied by this accidental.
    ///
    /// Returns the number of semitones to add to a natural degree.
    /// Sharp adds +1 semitone, flat adds -1 semitone.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert_eq!(DegreeAccident::Sharp.semitone_offset(), 1);
    /// assert_eq!(DegreeAccident::Flat.semitone_offset(), -1);
    ///
    /// // Usage in calculations
    /// let natural_third = 4; // E in C major (4 semitones)
    /// let flat_third = natural_third + DegreeAccident::Flat.semitone_offset();
    /// assert_eq!(flat_third, 3); // Eb (3 semitones)
    /// ```
    pub const fn semitone_offset(&self) -> i8 {
        match self {
            DegreeAccident::Sharp => 1,
            DegreeAccident::Flat => -1,
        }
    }

    /// Check if this is a sharp accidental.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert!(DegreeAccident::Sharp.is_sharp());
    /// assert!(!DegreeAccident::Flat.is_sharp());
    /// ```
    pub const fn is_sharp(&self) -> bool {
        matches!(self, DegreeAccident::Sharp)
    }

    /// Check if this is a flat accidental.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert!(!DegreeAccident::Sharp.is_flat());
    /// assert!(DegreeAccident::Flat.is_flat());
    /// ```
    pub const fn is_flat(&self) -> bool {
        matches!(self, DegreeAccident::Flat)
    }

    /// Get the opposite accidental.
    ///
    /// Sharp becomes flat, flat becomes sharp. Useful for enharmonic
    /// equivalence and musical transformations.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert_eq!(DegreeAccident::Sharp.opposite(), DegreeAccident::Flat);
    /// assert_eq!(DegreeAccident::Flat.opposite(), DegreeAccident::Sharp);
    /// ```
    pub const fn opposite(&self) -> DegreeAccident {
        match self {
            DegreeAccident::Sharp => DegreeAccident::Flat,
            DegreeAccident::Flat => DegreeAccident::Sharp,
        }
    }
}

impl fmt::Display for DegreeAccident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl Default for DegreeAccident {
    /// Returns `DegreeAccident::Sharp` as the default.
    ///
    /// # Examples
    /// ```
    /// use musik_std::DegreeAccident;
    ///
    /// assert_eq!(DegreeAccident::default(), DegreeAccident::Sharp);
    /// ```
    fn default() -> Self {
        DegreeAccident::Sharp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_accident_creation() {
        let sharp = DegreeAccident::Sharp;
        let flat = DegreeAccident::Flat;

        assert!(sharp.is_sharp());
        assert!(!sharp.is_flat());
        assert!(!flat.is_sharp());
        assert!(flat.is_flat());
    }

    #[test]
    fn test_symbols() {
        assert_eq!(DegreeAccident::Sharp.symbol(), "♯");
        assert_eq!(DegreeAccident::Flat.symbol(), "♭");
    }

    #[test]
    fn test_semitone_offsets() {
        assert_eq!(DegreeAccident::Sharp.semitone_offset(), 1);
        assert_eq!(DegreeAccident::Flat.semitone_offset(), -1);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(DegreeAccident::Sharp.opposite(), DegreeAccident::Flat);
        assert_eq!(DegreeAccident::Flat.opposite(), DegreeAccident::Sharp);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DegreeAccident::Sharp), "♯");
        assert_eq!(format!("{}", DegreeAccident::Flat), "♭");
    }

    #[test]
    fn test_equality_and_ordering() {
        let sharp1 = DegreeAccident::Sharp;
        let sharp2 = DegreeAccident::Sharp;
        let flat = DegreeAccident::Flat;

        assert_eq!(sharp1, sharp2);
        assert_ne!(sharp1, flat);

        // Test ordering (implementation dependent, but should be consistent)
        assert!(sharp1 <= sharp2);
        assert!(sharp1 >= sharp2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DegreeAccident::Sharp);
        set.insert(DegreeAccident::Flat);
        set.insert(DegreeAccident::Sharp); // Duplicate

        assert_eq!(set.len(), 2); // Only two unique values
        assert!(set.contains(&DegreeAccident::Sharp));
        assert!(set.contains(&DegreeAccident::Flat));
    }

    #[test]
    fn test_default() {
        assert_eq!(DegreeAccident::default(), DegreeAccident::Sharp);
    }

    #[test]
    fn test_musical_calculations() {
        // Test practical usage in musical calculations
        let natural_third = 4; // E in C major
        let flat_third = natural_third + DegreeAccident::Flat.semitone_offset();
        let sharp_third = natural_third + DegreeAccident::Sharp.semitone_offset();

        assert_eq!(flat_third, 3); // Eb
        assert_eq!(sharp_third, 5); // E# (same as F)

        // Test with seventh
        let natural_seventh = 11; // B in C major
        let flat_seventh = natural_seventh + DegreeAccident::Flat.semitone_offset();

        assert_eq!(flat_seventh, 10); // Bb
    }

    #[test]
    fn test_const_functions() {
        // Test that const functions work at compile time
        const SHARP: DegreeAccident = DegreeAccident::Sharp;
        const FLAT: DegreeAccident = DegreeAccident::Flat;

        assert_eq!(SHARP.symbol(), "♯");
        assert_eq!(FLAT.symbol(), "♭");
        assert_eq!(SHARP.semitone_offset(), 1);
        assert_eq!(FLAT.semitone_offset(), -1);
        assert!(SHARP.is_sharp());
        assert!(FLAT.is_flat());
        assert_eq!(SHARP.opposite(), FLAT);
    }
}
