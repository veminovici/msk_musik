//! Chord formulas using bit-packed degree representations.
//!
//! This module provides the `ChordFormula` struct for representing chord formulas
//! using a compact bit-packed format. Each degree in the major scale uses 2 bits
//! to encode whether it's absent, natural, flat, or sharp in the chord.

use crate::degree_alteration::DegreeAlteration;
use std::fmt;

/// Represents a chord formula using bit-packed degree information.
///
/// Uses a `u32` to store chord degree information with 2 bits per degree:
/// - 0: Degree not in chord
/// - 1: Natural degree in chord
/// - 2: Flat degree in chord  
/// - 3: Sharp degree in chord
///
/// The bits are arranged as follows (LSB to MSB):
/// - Bits 0-1: Root (1st degree)
/// - Bits 2-3: 2nd degree
/// - Bits 4-5: 3rd degree
/// - Bits 6-7: 4th degree
/// - Bits 8-9: 5th degree
/// - Bits 10-11: 6th degree
/// - Bits 12-13: 7th degree
/// - Bits 14-15: 8th degree (octave)
/// - Bits 16-17: 9th degree
/// - Bits 18-19: 10th degree
/// - Bits 20-21: 11th degree
/// - Bits 22-23: 12th degree
/// - Bits 24-25: 13th degree
/// - Bits 26-27: 14th degree
/// - Bits 28-29: 15th degree
/// - Bits 30-31: Reserved for future use
///
/// # Examples
/// ```
/// use musik_std::{ChordFormula, DegreeAlteration};
///
/// // C major triad: Root + Major 3rd + Perfect 5th
/// let c_major = ChordFormula::major_triad();
/// assert!(c_major.has_degree(1, DegreeAlteration::None));
/// assert!(c_major.has_degree(3, DegreeAlteration::None));
/// assert!(c_major.has_degree(5, DegreeAlteration::None));
///
/// // C minor triad: Root + Minor 3rd + Perfect 5th
/// let c_minor = ChordFormula::minor_triad();
/// assert!(c_minor.has_degree(1, DegreeAlteration::None));
/// assert!(c_minor.has_degree(3, DegreeAlteration::Flat));
/// assert!(c_minor.has_degree(5, DegreeAlteration::None));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChordFormula(pub u32);

impl ChordFormula {
    /// Creates an empty chord formula with no degrees.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ChordFormula;
    ///
    /// let empty = ChordFormula::empty();
    /// assert!(empty.is_empty());
    /// ```
    pub const fn empty() -> Self {
        ChordFormula(0)
    }

    /// Creates a new chord formula from a raw u32 value.
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let formula = ChordFormula::new(0b01_00_01_00_01); // Root + 3rd + 5th
    /// assert!(formula.has_degree(1, DegreeAlteration::None));
    /// assert!(formula.has_degree(3, DegreeAlteration::None));
    /// assert!(formula.has_degree(5, DegreeAlteration::None));
    /// ```
    pub const fn new(bits: u32) -> Self {
        ChordFormula(bits)
    }

    /// Returns the raw u32 value of the chord formula.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ChordFormula;
    ///
    /// let formula = ChordFormula::major_triad();
    /// let bits = formula.bits();
    /// assert_ne!(bits, 0);
    /// ```
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Checks if the chord formula is empty (no degrees).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ChordFormula;
    ///
    /// assert!(ChordFormula::empty().is_empty());
    /// assert!(!ChordFormula::major_triad().is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Adds a degree to the chord formula.
    ///
    /// # Arguments
    /// * `degree` - The scale degree (1-15)
    /// * `alteration` - The alteration (None for natural, Flat, or Sharp)
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let mut formula = ChordFormula::empty();
    /// formula = formula.with_degree(1, DegreeAlteration::None); // Root
    /// formula = formula.with_degree(3, DegreeAlteration::Flat); // Minor 3rd
    /// formula = formula.with_degree(5, DegreeAlteration::None); // Perfect 5th
    ///
    /// assert!(formula.has_degree(1, DegreeAlteration::None));
    /// assert!(formula.has_degree(3, DegreeAlteration::Flat));
    /// assert!(formula.has_degree(5, DegreeAlteration::None));
    /// ```
    pub const fn with_degree(self, degree: u8, alteration: DegreeAlteration) -> Self {
        if degree == 0 || degree > 15 {
            return self;
        }

        let value = match alteration {
            DegreeAlteration::None => 1,  // Natural
            DegreeAlteration::Flat => 2,  // Flat
            DegreeAlteration::Sharp => 3, // Sharp
        };

        let shift = (degree - 1) * 2;
        let mask = !(0b11 << shift);
        let new_bits = (self.0 & mask) | (value << shift);

        ChordFormula(new_bits)
    }

    /// Checks if a specific degree with alteration is present in the chord formula.
    ///
    /// # Arguments
    /// * `degree` - The scale degree (1-15)
    /// * `alteration` - The alteration (None for natural, Flat, or Sharp)
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let major = ChordFormula::major_triad();
    /// assert!(major.has_degree(1, DegreeAlteration::None)); // Root
    /// assert!(major.has_degree(3, DegreeAlteration::None)); // Major 3rd
    /// assert!(major.has_degree(5, DegreeAlteration::None)); // Perfect 5th
    /// assert!(!major.has_degree(3, DegreeAlteration::Flat)); // No minor 3rd
    /// ```
    pub const fn has_degree(self, degree: u8, alteration: DegreeAlteration) -> bool {
        if degree == 0 || degree > 15 {
            return false;
        }

        let expected = match alteration {
            DegreeAlteration::None => 1,  // Natural
            DegreeAlteration::Flat => 2,  // Flat
            DegreeAlteration::Sharp => 3, // Sharp
        };

        let shift = (degree - 1) * 2;
        let actual_value = (self.0 >> shift) & 0b11;

        actual_value == expected
    }

    /// Checks if any form of a degree (natural, flat, or sharp) is in the chord.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ChordFormula;
    ///
    /// let minor_triad = ChordFormula::minor_triad();
    /// assert!(minor_triad.has_any_degree(3)); // Has flat 3rd
    /// assert!(!minor_triad.has_any_degree(7)); // No 7th
    /// ```
    pub const fn has_any_degree(&self, degree: u8) -> bool {
        if degree == 0 || degree > 15 {
            return false;
        }

        let shift = (degree - 1) * 2;
        let value = (self.0 >> shift) & 0b11;

        value != 0
    }

    /// Gets the alteration of a degree if it exists in the chord.
    ///
    /// Returns `None` if the degree is not in the chord, or `Some(Option<DegreeAlteration>)`
    /// where the inner `Option` is `None` for natural and `Some` for altered degrees.
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let minor_triad = ChordFormula::minor_triad();
    /// assert_eq!(minor_triad.get_degree_alteration(1), Some(DegreeAlteration::None)); // Natural root
    /// assert_eq!(minor_triad.get_degree_alteration(3), Some(DegreeAlteration::Flat)); // Flat 3rd
    /// assert_eq!(minor_triad.get_degree_alteration(7), None); // No 7th
    /// ```
    pub const fn get_degree_alteration(&self, degree: u8) -> Option<DegreeAlteration> {
        if degree == 0 || degree > 15 {
            return None;
        }

        let shift = (degree - 1) * 2;
        let value = (self.0 >> shift) & 0b11;

        match value {
            0 => None,                          // Not in chord
            1 => Some(DegreeAlteration::None),  // Natural
            2 => Some(DegreeAlteration::Flat),  // Flat
            3 => Some(DegreeAlteration::Sharp), // Sharp
            _ => None,                          // Should never happen
        }
    }

    /// Returns a list of all degrees present in the chord with their alterations.
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let minor_seventh = ChordFormula::minor_seventh();
    /// let degrees = minor_seventh.degrees();
    ///
    /// assert_eq!(degrees.len(), 4);
    /// assert!(degrees.contains(&(1, DegreeAlteration::None)));
    /// assert!(degrees.contains(&(3, DegreeAlteration::Flat)));
    /// assert!(degrees.contains(&(5, DegreeAlteration::None)));
    /// assert!(degrees.contains(&(7, DegreeAlteration::Flat)));
    /// ```
    pub fn degrees(&self) -> Vec<(u8, DegreeAlteration)> {
        let mut result = Vec::new();

        for degree in 1..=15 {
            if let Some(alteration) = self.get_degree_alteration(degree) {
                result.push((degree, alteration));
            }
        }

        result
    }

    /// Combines two chord formulas using bitwise OR.
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ChordFormula, DegreeAlteration};
    ///
    /// let triad = ChordFormula::major_triad();
    /// let seventh = ChordFormula::empty().with_degree(7, DegreeAlteration::None);
    /// let major_seventh = triad.union(seventh);
    ///
    /// assert!(major_seventh.has_degree(1, DegreeAlteration::None));
    /// assert!(major_seventh.has_degree(3, DegreeAlteration::None));
    /// assert!(major_seventh.has_degree(5, DegreeAlteration::None));
    /// assert!(major_seventh.has_degree(7, DegreeAlteration::None));
    /// ```
    pub const fn union(self, other: ChordFormula) -> Self {
        ChordFormula(self.0 | other.0)
    }

    // ============================================================================
    // Common Chord Formulas
    // ============================================================================

    /// Major triad: 1 - 3 - 5
    pub const fn major_triad() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
    }

    /// Minor triad: 1 - ♭3 - 5
    pub const fn minor_triad() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
    }

    /// Diminished triad: 1 - ♭3 - ♭5
    pub const fn diminished_triad() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::Flat) // Diminished 5th
    }

    /// Augmented triad: 1 - 3 - ♯5
    pub const fn augmented_triad() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::Sharp) // Augmented 5th
    }

    /// Suspended 2nd: 1 - 2 - 5
    pub const fn sus2() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(2, DegreeAlteration::None) // Major 2nd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
    }

    /// Suspended 4th: 1 - 4 - 5
    pub const fn sus4() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(4, DegreeAlteration::None) // Perfect 4th
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
    }

    // ============================================================================
    // Seventh Chords
    // ============================================================================

    /// Major 7th: 1 - 3 - 5 - 7
    pub const fn major_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
    }

    /// Minor 7th: 1 - ♭3 - 5 - ♭7
    pub const fn minor_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }

    /// Dominant 7th: 1 - 3 - 5 - ♭7
    pub const fn dominant_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }

    /// Minor major 7th: 1 - ♭3 - 5 - 7
    pub const fn minor_major_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
    }

    /// Half-diminished 7th: 1 - ♭3 - ♭5 - ♭7
    pub const fn half_diminished_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::Flat) // Diminished 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }

    /// Fully diminished 7th: 1 - ♭3 - ♭5 - ♭♭7 (double flat 7th = 6th)
    pub const fn fully_diminished_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::Flat) // Diminished 5th
            .with_degree(6, DegreeAlteration::None) // Major 6th (enharmonic to ♭♭7)
    }

    /// Augmented major 7th: 1 - 3 - ♯5 - 7
    pub const fn augmented_major_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::Sharp) // Augmented 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
    }

    /// Augmented 7th: 1 - 3 - ♯5 - ♭7
    pub const fn augmented_seventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::Sharp) // Augmented 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }

    // ============================================================================
    // Extended Jazz Chords (9ths)
    // ============================================================================

    /// Major 9th: 1 - 3 - 5 - 7 - 9
    pub const fn major_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// Minor 9th: 1 - ♭3 - 5 - ♭7 - 9
    pub const fn minor_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// Dominant 9th: 1 - 3 - 5 - ♭7 - 9
    pub const fn dominant_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// Dominant 7♭9: 1 - 3 - 5 - ♭7 - ♭9
    pub const fn dominant_seventh_flat_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::Flat) // Flat 9th
    }

    /// Dominant 7♯9: 1 - 3 - 5 - ♭7 - ♯9
    pub const fn dominant_seventh_sharp_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::Sharp) // Sharp 9th
    }

    // ============================================================================
    // Extended Jazz Chords (11ths)
    // ============================================================================

    /// Major 11th: 1 - 3 - 5 - 7 - 9 - 11
    pub const fn major_eleventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
    }

    /// Minor 11th: 1 - ♭3 - 5 - ♭7 - 9 - 11
    pub const fn minor_eleventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
    }

    /// Dominant 11th: 1 - 3 - 5 - ♭7 - 9 - 11
    pub const fn dominant_eleventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
    }

    /// Dominant 7♯11: 1 - 3 - 5 - ♭7 - ♯11
    pub const fn dominant_seventh_sharp_eleventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(11, DegreeAlteration::Sharp) // Sharp 11th
    }

    // ============================================================================
    // Extended Jazz Chords (13ths)
    // ============================================================================

    /// Major 13th: 1 - 3 - 5 - 7 - 9 - 11 - 13
    pub const fn major_thirteenth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::None) // Major 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
            .with_degree(13, DegreeAlteration::None) // Major 13th
    }

    /// Minor 13th: 1 - ♭3 - 5 - ♭7 - 9 - 11 - 13
    pub const fn minor_thirteenth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
            .with_degree(13, DegreeAlteration::None) // Major 13th
    }

    /// Dominant 13th: 1 - 3 - 5 - ♭7 - 9 - 11 - 13
    pub const fn dominant_thirteenth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::None) // Perfect 11th
            .with_degree(13, DegreeAlteration::None) // Major 13th
    }

    /// Dominant 13♭9: 1 - 3 - 5 - ♭7 - ♭9 - 13
    pub const fn dominant_thirteenth_flat_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::Flat) // Flat 9th
            .with_degree(13, DegreeAlteration::None) // Major 13th
    }

    /// Dominant 13♯11: 1 - 3 - 5 - ♭7 - 9 - ♯11 - 13
    pub const fn dominant_thirteenth_sharp_eleventh() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::None) // Major 9th
            .with_degree(11, DegreeAlteration::Sharp) // Sharp 11th
            .with_degree(13, DegreeAlteration::None) // Major 13th
    }

    // ============================================================================
    // Add Chords
    // ============================================================================

    /// Add 9: 1 - 3 - 5 - 9 (no 7th)
    pub const fn add_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// Minor add 9: 1 - ♭3 - 5 - 9 (no 7th)
    pub const fn minor_add_ninth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// 6th chord: 1 - 3 - 5 - 6
    pub const fn sixth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(6, DegreeAlteration::None) // Major 6th
    }

    /// Minor 6th: 1 - ♭3 - 5 - 6
    pub const fn minor_sixth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(6, DegreeAlteration::None) // Major 6th
    }

    /// 6/9 chord: 1 - 3 - 5 - 6 - 9
    pub const fn six_nine() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(6, DegreeAlteration::None) // Major 6th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    /// Minor 6/9: 1 - ♭3 - 5 - 6 - 9
    pub const fn minor_six_nine() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::Flat) // Minor 3rd
            .with_degree(5, DegreeAlteration::None) // Perfect 5th
            .with_degree(6, DegreeAlteration::None) // Major 6th
            .with_degree(9, DegreeAlteration::None) // Major 9th
    }

    // ============================================================================
    // Altered Dominant Chords
    // ============================================================================

    /// Altered dominant: 1 - 3 - ♭7 - ♭9 - ♯9 - ♯11 - ♭13
    pub const fn altered_dominant() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
            .with_degree(9, DegreeAlteration::Flat) // Flat 9th
            .with_degree(9, DegreeAlteration::Sharp) // Sharp 9th (both alterations)
            .with_degree(11, DegreeAlteration::Sharp) // Sharp 11th
            .with_degree(13, DegreeAlteration::Flat) // Flat 13th
    }

    /// Dominant 7♯5: 1 - 3 - ♯5 - ♭7
    pub const fn dominant_seventh_sharp_fifth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::Sharp) // Sharp 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }

    /// Dominant 7♭5: 1 - 3 - ♭5 - ♭7
    pub const fn dominant_seventh_flat_fifth() -> Self {
        ChordFormula(0)
            .with_degree(1, DegreeAlteration::None) // Root
            .with_degree(3, DegreeAlteration::None) // Major 3rd
            .with_degree(5, DegreeAlteration::Flat) // Flat 5th
            .with_degree(7, DegreeAlteration::Flat) // Minor 7th
    }
}

impl fmt::Display for ChordFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "∅");
        }

        let degrees = self.degrees();
        let degree_strings: Vec<String> = degrees
            .iter()
            .map(|(degree, alteration)| match alteration {
                DegreeAlteration::None => degree.to_string(),
                DegreeAlteration::Flat => format!("♭{}", degree),
                DegreeAlteration::Sharp => format!("♯{}", degree),
            })
            .collect();

        write!(f, "{}", degree_strings.join(" "))
    }
}

impl Default for ChordFormula {
    /// Returns an empty chord formula by default.
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_chord() {
        let empty = ChordFormula::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.bits(), 0);
        assert_eq!(empty.degrees().len(), 0);
    }

    #[test]
    fn test_major_triad() {
        let major = ChordFormula::major_triad();

        assert!(major.has_degree(1, DegreeAlteration::None));
        assert!(major.has_degree(3, DegreeAlteration::None));
        assert!(major.has_degree(5, DegreeAlteration::None));
        assert!(!major.has_degree(7, DegreeAlteration::None));

        let degrees = major.degrees();
        assert_eq!(degrees.len(), 3);
        assert!(degrees.contains(&(1, DegreeAlteration::None)));
        assert!(degrees.contains(&(3, DegreeAlteration::None)));
        assert!(degrees.contains(&(5, DegreeAlteration::None)));
    }

    #[test]
    fn test_minor_triad() {
        let minor = ChordFormula::minor_triad();

        assert!(minor.has_degree(1, DegreeAlteration::None));
        assert!(minor.has_degree(3, DegreeAlteration::Flat));
        assert!(minor.has_degree(5, DegreeAlteration::None));
        assert!(!minor.has_degree(3, DegreeAlteration::None)); // Natural 3rd not present

        let degrees = minor.degrees();
        assert_eq!(degrees.len(), 3);
        assert!(degrees.contains(&(1, DegreeAlteration::None)));
        assert!(degrees.contains(&(3, DegreeAlteration::Flat)));
        assert!(degrees.contains(&(5, DegreeAlteration::None)));
    }

    #[test]
    fn test_dominant_seventh() {
        let dom7 = ChordFormula::dominant_seventh();

        assert!(dom7.has_degree(1, DegreeAlteration::None));
        assert!(dom7.has_degree(3, DegreeAlteration::None));
        assert!(dom7.has_degree(5, DegreeAlteration::None));
        assert!(dom7.has_degree(7, DegreeAlteration::Flat));
        assert!(!dom7.has_degree(7, DegreeAlteration::None)); // Natural 7th not present
    }

    #[test]
    fn test_extended_jazz_chords() {
        let maj13 = ChordFormula::major_thirteenth();

        assert!(maj13.has_degree(1, DegreeAlteration::None));
        assert!(maj13.has_degree(3, DegreeAlteration::None));
        assert!(maj13.has_degree(5, DegreeAlteration::None));
        assert!(maj13.has_degree(7, DegreeAlteration::None));
        assert!(maj13.has_degree(9, DegreeAlteration::None));
        assert!(maj13.has_degree(11, DegreeAlteration::None));
        assert!(maj13.has_degree(13, DegreeAlteration::None));

        let degrees = maj13.degrees();
        assert_eq!(degrees.len(), 7);
    }

    #[test]
    fn test_altered_chords() {
        let dom7_sharp9 = ChordFormula::dominant_seventh_sharp_ninth();

        assert!(dom7_sharp9.has_degree(1, DegreeAlteration::None));
        assert!(dom7_sharp9.has_degree(3, DegreeAlteration::None));
        assert!(dom7_sharp9.has_degree(5, DegreeAlteration::None));
        assert!(dom7_sharp9.has_degree(7, DegreeAlteration::Flat));
        assert!(dom7_sharp9.has_degree(9, DegreeAlteration::Sharp));
        assert!(!dom7_sharp9.has_degree(9, DegreeAlteration::None)); // Natural 9th not present
    }

    #[test]
    fn test_has_any_degree() {
        let minor = ChordFormula::minor_triad();

        assert!(minor.has_any_degree(1));
        assert!(minor.has_any_degree(3)); // Has flat 3rd
        assert!(minor.has_any_degree(5));
        assert!(!minor.has_any_degree(7));
    }

    #[test]
    fn test_get_degree_alteration() {
        let minor = ChordFormula::minor_triad();

        assert_eq!(minor.get_degree_alteration(1), Some(DegreeAlteration::None));
        assert_eq!(minor.get_degree_alteration(3), Some(DegreeAlteration::Flat));
        assert_eq!(minor.get_degree_alteration(5), Some(DegreeAlteration::None));
        assert_eq!(minor.get_degree_alteration(7), None);
    }

    #[test]
    fn test_union() {
        let triad = ChordFormula::major_triad();
        let seventh = ChordFormula::empty().with_degree(7, DegreeAlteration::None);
        let maj7 = triad.union(seventh);

        assert!(maj7.has_degree(1, DegreeAlteration::None));
        assert!(maj7.has_degree(3, DegreeAlteration::None));
        assert!(maj7.has_degree(5, DegreeAlteration::None));
        assert!(maj7.has_degree(7, DegreeAlteration::None));
    }

    #[test]
    fn test_display() {
        let major = ChordFormula::major_triad();
        let display = format!("{}", major);
        assert!(display.contains("1"));
        assert!(display.contains("3"));
        assert!(display.contains("5"));

        let minor = ChordFormula::minor_triad();
        let display = format!("{}", minor);
        assert!(display.contains("♭3"));

        let empty = ChordFormula::empty();
        assert_eq!(format!("{}", empty), "∅");
    }

    #[test]
    fn test_add_chords() {
        let add9 = ChordFormula::add_ninth();

        assert!(add9.has_degree(1, DegreeAlteration::None));
        assert!(add9.has_degree(3, DegreeAlteration::None));
        assert!(add9.has_degree(5, DegreeAlteration::None));
        assert!(add9.has_degree(9, DegreeAlteration::None));
        assert!(!add9.has_degree(7, DegreeAlteration::None)); // No 7th in add chords
        assert!(!add9.has_degree(7, DegreeAlteration::Flat));
    }

    #[test]
    fn test_suspended_chords() {
        let sus2 = ChordFormula::sus2();
        assert!(sus2.has_degree(1, DegreeAlteration::None));
        assert!(sus2.has_degree(2, DegreeAlteration::None));
        assert!(sus2.has_degree(5, DegreeAlteration::None));
        assert!(!sus2.has_degree(3, DegreeAlteration::None)); // No 3rd in sus chords

        let sus4 = ChordFormula::sus4();
        assert!(sus4.has_degree(1, DegreeAlteration::None));
        assert!(sus4.has_degree(4, DegreeAlteration::None));
        assert!(sus4.has_degree(5, DegreeAlteration::None));
        assert!(!sus4.has_degree(3, DegreeAlteration::None)); // No 3rd in sus chords
    }

    #[test]
    fn test_diminished_chords() {
        let dim = ChordFormula::diminished_triad();
        assert!(dim.has_degree(1, DegreeAlteration::None));
        assert!(dim.has_degree(3, DegreeAlteration::Flat));
        assert!(dim.has_degree(5, DegreeAlteration::Flat));

        let dim7 = ChordFormula::fully_diminished_seventh();
        assert!(dim7.has_degree(1, DegreeAlteration::None));
        assert!(dim7.has_degree(3, DegreeAlteration::Flat));
        assert!(dim7.has_degree(5, DegreeAlteration::Flat));
        assert!(dim7.has_degree(6, DegreeAlteration::None)); // ♭♭7 = 6
    }

    #[test]
    fn test_bit_manipulation() {
        let formula = ChordFormula::empty()
            .with_degree(1, DegreeAlteration::None)
            .with_degree(3, DegreeAlteration::Flat)
            .with_degree(5, DegreeAlteration::None);

        // Check specific bit patterns
        let bits = formula.bits();
        assert_eq!(bits & 0b11, 1); // Root = 1 (natural)
        assert_eq!((bits >> 4) & 0b11, 2); // 3rd = 2 (flat)
        assert_eq!((bits >> 8) & 0b11, 1); // 5th = 1 (natural)
    }

    #[test]
    fn test_const_functions() {
        const MAJOR: ChordFormula = ChordFormula::major_triad();
        const MINOR: ChordFormula = ChordFormula::minor_triad();

        assert!(MAJOR.has_degree(1, DegreeAlteration::None));
        assert!(MAJOR.has_degree(3, DegreeAlteration::None));
        assert!(MAJOR.has_degree(5, DegreeAlteration::None));

        assert!(MINOR.has_degree(1, DegreeAlteration::None));
        assert!(MINOR.has_degree(3, DegreeAlteration::Flat));
        assert!(MINOR.has_degree(5, DegreeAlteration::None));
    }
}
