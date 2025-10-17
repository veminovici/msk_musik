//! Scale formulas for representing musical scales using bit flags.
//!
//! This module provides the `ScaleFormula` type for representing scale patterns
//! using bit flags, where each bit position indicates whether a semitone is
//! present in the scale.

use crate::note::Note;
use crate::semitone::{Semitone, SEMITONES_IN_OCTAVE};
use std::fmt;

/// Represents a musical scale formula using bit flags.
///
/// Each bit position corresponds to a semitone offset from the root note.
/// A set bit (1) indicates that the semitone is present in the scale,
/// while an unset bit (0) indicates it's not part of the scale.
///
/// Supports two octaves (24 semitones) to accommodate extended harmony:
///
/// **First Octave (bits 0-11):**
/// - Bit 0: Root (1)
/// - Bit 1: Minor 2nd (♭2)
/// - Bit 2: Major 2nd (2)
/// - Bit 3: Minor 3rd (♭3)
/// - Bit 4: Major 3rd (3)
/// - Bit 5: Perfect 4th (4)
/// - Bit 6: Tritone (♭5/♯4)
/// - Bit 7: Perfect 5th (5)
/// - Bit 8: Minor 6th (♭6)
/// - Bit 9: Major 6th (6)
/// - Bit 10: Minor 7th (♭7)
/// - Bit 11: Major 7th (7)
///
/// **Second Octave (bits 12-23):**
/// - Bit 12: Octave (8)
/// - Bit 13: Minor 9th (♭9)
/// - Bit 14: Major 9th (9)
/// - Bit 15: Minor 10th (♭10)
/// - Bit 16: Major 10th (10)
/// - Bit 17: 11th (11)
/// - Bit 18: Augmented 11th (♯11)
/// - Bit 19: Perfect 12th (12)
/// - Bit 20: Minor 13th (♭13)
/// - Bit 21: Major 13th (13)
/// - Bit 22: Minor 14th (♭14)
/// - Bit 23: Major 14th (14)
///
/// # Examples
/// ```
/// use musik_std::ScaleFormula;
///
/// // Major scale: 1, 2, 3, 4, 5, 6, 7
/// let major = ScaleFormula::major();
/// assert!(major.contains_semitone(0)); // Root
/// assert!(major.contains_semitone(2)); // Major 2nd
/// assert!(major.contains_semitone(4)); // Major 3rd
/// assert!(!major.contains_semitone(1)); // No minor 2nd
///
/// // Minor scale: 1, 2, ♭3, 4, 5, ♭6, ♭7
/// let minor = ScaleFormula::minor();
/// assert!(minor.contains_semitone(3)); // Minor 3rd
/// assert!(!minor.contains_semitone(4)); // No major 3rd
///
/// // Extended harmony with 9ths, 11ths, 13ths
/// let major_extended = ScaleFormula::major_extended();
/// assert!(major_extended.contains_semitone(14)); // 9th
/// assert!(major_extended.contains_semitone(17)); // 11th
/// assert!(major_extended.contains_semitone(21)); // 13th
///
/// // Custom scale formula
/// let custom = ScaleFormula::from_semitones(&[0, 2, 4, 7, 9]);
/// assert_eq!(custom.note_count(), 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleFormula(pub u32);

impl ScaleFormula {
    /// Create a new scale formula from a bit pattern.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// // Binary: 101010110101 = Major scale (first octave)
    /// let formula = ScaleFormula::new(0b101010110101);
    /// assert_eq!(formula.note_count(), 7);
    /// ```
    pub const fn new(bits: u32) -> Self {
        ScaleFormula(bits)
    }

    /// Create an empty scale formula (no notes).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let empty = ScaleFormula::empty();
    /// assert_eq!(empty.note_count(), 0);
    /// assert!(!empty.contains_semitone(0));
    /// ```
    pub const fn empty() -> Self {
        ScaleFormula(0)
    }

    /// Create a chromatic scale formula (all 12 semitones in first octave).
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ScaleFormula, SEMITONES_IN_OCTAVE};
    ///
    /// let chromatic = ScaleFormula::chromatic();
    /// assert_eq!(chromatic.note_count(), 12);
    /// for i in 0..SEMITONES_IN_OCTAVE {
    ///     assert!(chromatic.contains_semitone(i));
    /// }
    /// ```
    pub const fn chromatic() -> Self {
        ScaleFormula((1u32 << SEMITONES_IN_OCTAVE) - 1) // All 12 bits set in first octave
    }

    /// Create a full chromatic scale formula (all 24 semitones across two octaves).
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ScaleFormula, SEMITONES_IN_OCTAVE};
    ///
    /// let chromatic_extended = ScaleFormula::chromatic_extended();
    /// assert_eq!(chromatic_extended.note_count(), 24);
    /// for i in 0..(2 * SEMITONES_IN_OCTAVE) {
    ///     assert!(chromatic_extended.contains_semitone(i));
    /// }
    /// ```
    pub const fn chromatic_extended() -> Self {
        ScaleFormula((1u32 << (2 * SEMITONES_IN_OCTAVE)) - 1) // All 24 bits set
    }

    /// Create a major scale formula (Ionian mode).
    ///
    /// Pattern: 1, 2, 3, 4, 5, 6, 7
    /// Semitones: 0, 2, 4, 5, 7, 9, 11
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// assert_eq!(major.note_count(), 7);
    /// assert!(major.contains_semitone(0)); // Root
    /// assert!(major.contains_semitone(4)); // Major 3rd
    /// assert!(!major.contains_semitone(3)); // No minor 3rd
    /// ```
    pub const fn major() -> Self {
        // Bits: 11 10  9  8  7  6  5  4  3  2  1  0
        //        7  ♭7  6  ♭6  5 ♭5  4  3 ♭3  2 ♭2  1
        //        1   0  1   0  1  0  1  1  0  1  0  1
        ScaleFormula(0b101010110101)
    }

    /// Create an extended major scale formula spanning two octaves.
    ///
    /// Pattern: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
    /// Includes natural extensions (9th, 11th, 13th) in the second octave.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major_extended = ScaleFormula::major_extended();
    /// assert_eq!(major_extended.note_count(), 14);
    /// assert!(major_extended.contains_semitone(14)); // 9th
    /// assert!(major_extended.contains_semitone(17)); // 11th
    /// assert!(major_extended.contains_semitone(21)); // 13th
    /// ```
    pub const fn major_extended() -> Self {
        // Major scale pattern: 0, 2, 4, 5, 7, 9, 11
        // First octave: 0b101010110101
        // Second octave: same pattern shifted by SEMITONES_IN_OCTAVE (12)
        // Combined: 0b101010110101101010110101
        const MAJOR_PATTERN: u32 = 0b101010110101;
        ScaleFormula(MAJOR_PATTERN | (MAJOR_PATTERN << SEMITONES_IN_OCTAVE))
    }

    /// Create a natural minor scale formula (Aeolian mode).
    ///
    /// Pattern: 1, 2, ♭3, 4, 5, ♭6, ♭7
    /// Semitones: 0, 2, 3, 5, 7, 8, 10
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let minor = ScaleFormula::minor();
    /// assert_eq!(minor.note_count(), 7);
    /// assert!(minor.contains_semitone(3)); // Minor 3rd
    /// assert!(!minor.contains_semitone(4)); // No major 3rd
    /// ```
    pub const fn minor() -> Self {
        // Bits: 11 10  9  8  7  6  5  4  3  2  1  0
        //        7  ♭7  6  ♭6  5 ♭5  4  3 ♭3  2 ♭2  1
        //        0   1  0   1  1  0  1  0  1  1  0  1
        ScaleFormula(0b010110101101)
    }

    /// Create a pentatonic major scale formula.
    ///
    /// Pattern: 1, 2, 3, 5, 6
    /// Semitones: 0, 2, 4, 7, 9
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let pentatonic = ScaleFormula::pentatonic_major();
    /// assert_eq!(pentatonic.note_count(), 5);
    /// ```
    pub const fn pentatonic_major() -> Self {
        // Bits: 11 10  9  8  7  6  5  4  3  2  1  0
        //        7  ♭7  6  ♭6  5 ♭5  4  3 ♭3  2 ♭2  1
        //        0   0  1   0  1  0  0  1  0  1  0  1
        ScaleFormula(0b001010010101)
    }

    /// Create a pentatonic minor scale formula.
    ///
    /// Pattern: 1, ♭3, 4, 5, ♭7
    /// Semitones: 0, 3, 5, 7, 10
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let pentatonic_minor = ScaleFormula::pentatonic_minor();
    /// assert_eq!(pentatonic_minor.note_count(), 5);
    /// ```
    pub const fn pentatonic_minor() -> Self {
        // Bits: 11 10  9  8  7  6  5  4  3  2  1  0
        //        7  ♭7  6  ♭6  5 ♭5  4  3 ♭3  2 ♭2  1
        //        0   1  0   0  1  0  1  0  1  0  0  1
        ScaleFormula(0b010010101001)
    }

    /// Create a blues scale formula.
    ///
    /// Pattern: 1, ♭3, 4, ♭5, 5, ♭7
    /// Semitones: 0, 3, 5, 6, 7, 10
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let blues = ScaleFormula::blues();
    /// assert_eq!(blues.note_count(), 6);
    /// assert!(blues.contains_semitone(6)); // ♭5 (tritone)
    /// ```
    pub const fn blues() -> Self {
        // Bits: 11 10  9  8  7  6  5  4  3  2  1  0
        //        7  ♭7  6  ♭6  5 ♭5  4  3 ♭3  2 ♭2  1
        //        0   1  0   0  1  1  1  0  1  0  0  1
        ScaleFormula(0b010011101001)
    }

    /// Create a scale formula from a list of semitone offsets.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// // Major scale
    /// let major = ScaleFormula::from_semitones(&[0, 2, 4, 5, 7, 9, 11]);
    /// assert_eq!(major, ScaleFormula::major());
    ///
    /// // Custom scale
    /// let custom = ScaleFormula::from_semitones(&[0, 1, 3, 6, 8]);
    /// assert_eq!(custom.note_count(), 5);
    ///
    /// // Extended harmony with 9ths, 11ths, 13ths
    /// let extended = ScaleFormula::from_semitones(&[0, 2, 4, 5, 7, 9, 11, 14, 17, 21]);
    /// assert!(extended.contains_semitone(14)); // 9th
    /// assert!(extended.contains_semitone(17)); // 11th
    /// assert!(extended.contains_semitone(21)); // 13th
    /// ```
    pub fn from_semitones(semitones: &[u8]) -> Self {
        let mut bits = 0u32;
        for &semitone in semitones {
            if semitone < 2 * SEMITONES_IN_OCTAVE {
                bits |= 1 << semitone;
            }
        }
        ScaleFormula(bits)
    }

    /// Check if a semitone is present in the scale formula.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// assert!(major.contains_semitone(0)); // Root
    /// assert!(major.contains_semitone(4)); // Major 3rd
    /// assert!(!major.contains_semitone(1)); // No minor 2nd
    ///
    /// // Extended formulas support second octave
    /// let extended = ScaleFormula::major_extended();
    /// assert!(extended.contains_semitone(14)); // 9th
    /// assert!(extended.contains_semitone(17)); // 11th
    /// ```
    pub const fn contains_semitone(&self, semitone: u8) -> bool {
        if semitone >= 2 * SEMITONES_IN_OCTAVE {
            return false;
        }
        (self.0 & (1 << semitone)) != 0
    }

    /// Get the number of notes in the scale.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// assert_eq!(ScaleFormula::major().note_count(), 7);
    /// assert_eq!(ScaleFormula::pentatonic_major().note_count(), 5);
    /// assert_eq!(ScaleFormula::chromatic().note_count(), 12);
    /// ```
    pub const fn note_count(&self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Get all semitone offsets present in the scale as a vector.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// let semitones = major.semitones();
    /// assert_eq!(semitones, vec![0, 2, 4, 5, 7, 9, 11]);
    ///
    /// // Extended scales show notes in both octaves
    /// let extended = ScaleFormula::major_extended();
    /// let ext_semitones = extended.semitones();
    /// assert_eq!(ext_semitones, vec![0, 2, 4, 5, 7, 9, 11, 12, 14, 16, 17, 19, 21, 23]);
    /// ```
    pub fn semitones(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for i in 0..(2 * SEMITONES_IN_OCTAVE) {
            if self.contains_semitone(i) {
                result.push(i);
            }
        }
        result
    }

    /// Generate all notes in the scale starting from the given root note.
    ///
    /// Takes a root note and applies the scale formula to generate all notes
    /// in the scale. The notes are returned as an iterator in ascending semitone order.
    /// This avoids allocating a vector until explicitly collected.
    ///
    /// # Examples
    /// ```
    /// use musik_std::{ScaleFormula, Note};
    ///
    /// // C Major scale: C, D, E, F, G, A, B
    /// let major = ScaleFormula::major();
    /// let c_major_notes: Vec<Note> = major.notes_from_root(Note::new(0)).collect(); // C
    ///
    /// // Should contain: C(0), D(2), E(4), F(5), G(7), A(9), B(11)
    /// let expected_semitones: Vec<u8> = vec![0, 2, 4, 5, 7, 9, 11];
    /// let actual_semitones: Vec<u8> = c_major_notes.iter()
    ///     .map(|note| u8::from(*note))
    ///     .collect();
    /// assert_eq!(actual_semitones, expected_semitones);
    ///
    /// // F Major scale: F, G, A, Bb, C, D, E
    /// let f_major_notes: Vec<Note> = major.notes_from_root(Note::new(5)).collect(); // F
    /// let f_major_semitones: Vec<u8> = f_major_notes.iter()
    ///     .map(|note| u8::from(*note))
    ///     .collect();
    /// // F(5), G(7), A(9), Bb(10), C(12), D(14), E(16) - continues linearly
    /// assert_eq!(f_major_semitones, vec![5, 7, 9, 10, 12, 14, 16]);
    ///
    /// // Extended harmony example with jazz chords
    /// let major_extended = ScaleFormula::major_extended();
    /// let c_extended_notes: Vec<Note> = major_extended.notes_from_root(Note::new(0)).collect();
    /// assert_eq!(c_extended_notes.len(), 14); // 7 notes in each octave
    ///
    /// // Use iterator directly without collecting
    /// let note_count = major.notes_from_root(Note::new(0)).count();
    /// assert_eq!(note_count, 7);
    /// ```
    pub fn notes_from_root(&self, root: Note) -> impl Iterator<Item = Note> + '_ {
        self.semitones()
            .into_iter()
            .map(move |semitone_offset| root + Semitone::new(semitone_offset))
    }

    /// Get the internal bit representation.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// let bits = major.bits();
    /// assert_eq!(bits, 0b101010110101);
    ///
    /// // Extended formulas can have bits in second octave
    /// let extended = ScaleFormula::major_extended();
    /// assert!(extended.bits() > 0xFFF); // Uses more than 12 bits
    /// ```
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Check if the scale formula is empty (no notes).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// assert!(ScaleFormula::empty().is_empty());
    /// assert!(!ScaleFormula::major().is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if the scale contains the root note (semitone 0).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// assert!(ScaleFormula::major().has_root());
    /// assert!(!ScaleFormula::empty().has_root());
    /// ```
    pub const fn has_root(&self) -> bool {
        self.contains_semitone(0)
    }

    /// Union two scale formulas (bitwise OR).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// let minor = ScaleFormula::minor();
    /// let combined = major.union(minor);
    ///
    /// // Combined should have both major and minor thirds
    /// assert!(combined.contains_semitone(3)); // Minor 3rd
    /// assert!(combined.contains_semitone(4)); // Major 3rd
    /// ```
    pub const fn union(self, other: ScaleFormula) -> ScaleFormula {
        ScaleFormula(self.0 | other.0)
    }

    /// Intersection of two scale formulas (bitwise AND).
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// let minor = ScaleFormula::minor();
    /// let common = major.intersection(minor);
    ///
    /// // Common notes: 1, 2, 4, 5
    /// assert!(common.contains_semitone(0)); // Root
    /// assert!(common.contains_semitone(2)); // Major 2nd
    /// assert!(!common.contains_semitone(3)); // Minor 3rd (only in minor)
    /// assert!(!common.contains_semitone(4)); // Major 3rd (only in major)
    /// ```
    pub const fn intersection(self, other: ScaleFormula) -> ScaleFormula {
        ScaleFormula(self.0 & other.0)
    }

    /// Complement of the scale formula (bitwise NOT, masked to first octave only).
    ///
    /// This only considers the first 12 semitones to maintain traditional scale complement behavior.
    ///
    /// # Examples
    /// ```
    /// use musik_std::ScaleFormula;
    ///
    /// let major = ScaleFormula::major();
    /// let complement = major.complement();
    ///
    /// // Complement should contain all notes NOT in major scale
    /// assert!(!complement.contains_semitone(0)); // Root not in complement
    /// assert!(complement.contains_semitone(1)); // Minor 2nd in complement
    /// ```
    pub const fn complement(self) -> ScaleFormula {
        ScaleFormula((!self.0) & ((1u32 << SEMITONES_IN_OCTAVE) - 1)) // Mask to first octave
    }
}

impl fmt::Display for ScaleFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "Empty");
        }

        let semitones = self.semitones();
        let note_names = [
            "1", "♭2", "2", "♭3", "3", "4", "♭5", "5", "♭6", "6", "♭7", "7",
        ];

        let names: Vec<&str> = semitones.iter().map(|&s| note_names[s as usize]).collect();

        write!(f, "{}", names.join(", "))
    }
}

impl fmt::Binary for ScaleFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:012b}", self.0)
    }
}

// Bitwise operations
impl std::ops::BitOr for ScaleFormula {
    type Output = ScaleFormula;

    fn bitor(self, rhs: ScaleFormula) -> ScaleFormula {
        self.union(rhs)
    }
}

impl std::ops::BitAnd for ScaleFormula {
    type Output = ScaleFormula;

    fn bitand(self, rhs: ScaleFormula) -> ScaleFormula {
        self.intersection(rhs)
    }
}

impl std::ops::Not for ScaleFormula {
    type Output = ScaleFormula;

    fn not(self) -> ScaleFormula {
        self.complement()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_formula_creation() {
        let formula = ScaleFormula::new(0b101010110101);
        assert_eq!(formula.bits(), 0b101010110101);
        assert_eq!(formula.note_count(), 7);
    }

    #[test]
    fn test_empty_scale() {
        let empty = ScaleFormula::empty();
        assert!(empty.is_empty());
        assert_eq!(empty.note_count(), 0);
        assert!(!empty.has_root());
    }

    #[test]
    fn test_chromatic_scale() {
        let chromatic = ScaleFormula::chromatic();
        assert_eq!(chromatic.note_count(), 12);
        for i in 0..SEMITONES_IN_OCTAVE {
            assert!(chromatic.contains_semitone(i));
        }
        assert!(!chromatic.is_empty());
        assert!(chromatic.has_root());
    }

    #[test]
    fn test_major_scale() {
        let major = ScaleFormula::major();
        let expected_semitones = vec![0, 2, 4, 5, 7, 9, 11];

        assert_eq!(major.note_count(), 7);
        assert_eq!(major.semitones(), expected_semitones);
        assert!(major.has_root());

        // Test specific intervals
        assert!(major.contains_semitone(0)); // Root
        assert!(major.contains_semitone(2)); // Major 2nd
        assert!(major.contains_semitone(4)); // Major 3rd
        assert!(major.contains_semitone(5)); // Perfect 4th
        assert!(major.contains_semitone(7)); // Perfect 5th
        assert!(major.contains_semitone(9)); // Major 6th
        assert!(major.contains_semitone(11)); // Major 7th

        // Test missing intervals
        assert!(!major.contains_semitone(1)); // Minor 2nd
        assert!(!major.contains_semitone(3)); // Minor 3rd
        assert!(!major.contains_semitone(6)); // Tritone
        assert!(!major.contains_semitone(8)); // Minor 6th
        assert!(!major.contains_semitone(10)); // Minor 7th
    }

    #[test]
    fn test_minor_scale() {
        let minor = ScaleFormula::minor();
        let expected_semitones = vec![0, 2, 3, 5, 7, 8, 10];

        assert_eq!(minor.note_count(), 7);
        assert_eq!(minor.semitones(), expected_semitones);
        assert!(minor.has_root());

        // Test specific intervals
        assert!(minor.contains_semitone(0)); // Root
        assert!(minor.contains_semitone(2)); // Major 2nd
        assert!(minor.contains_semitone(3)); // Minor 3rd
        assert!(minor.contains_semitone(5)); // Perfect 4th
        assert!(minor.contains_semitone(7)); // Perfect 5th
        assert!(minor.contains_semitone(8)); // Minor 6th
        assert!(minor.contains_semitone(10)); // Minor 7th

        // Test missing intervals
        assert!(!minor.contains_semitone(4)); // Major 3rd
        assert!(!minor.contains_semitone(9)); // Major 6th
        assert!(!minor.contains_semitone(11)); // Major 7th
    }

    #[test]
    fn test_pentatonic_scales() {
        let pent_major = ScaleFormula::pentatonic_major();
        assert_eq!(pent_major.semitones(), vec![0, 2, 4, 7, 9]);
        assert_eq!(pent_major.note_count(), 5);

        let pent_minor = ScaleFormula::pentatonic_minor();
        assert_eq!(pent_minor.semitones(), vec![0, 3, 5, 7, 10]);
        assert_eq!(pent_minor.note_count(), 5);
    }

    #[test]
    fn test_blues_scale() {
        let blues = ScaleFormula::blues();
        assert_eq!(blues.semitones(), vec![0, 3, 5, 6, 7, 10]);
        assert_eq!(blues.note_count(), 6);
        assert!(blues.contains_semitone(6)); // ♭5 (tritone)
    }

    #[test]
    fn test_from_semitones() {
        let custom = ScaleFormula::from_semitones(&[0, 2, 4, 7, 9]);
        assert_eq!(custom.semitones(), vec![0, 2, 4, 7, 9]);
        assert_eq!(custom, ScaleFormula::pentatonic_major());

        // Test with extended range semitones
        let with_extended = ScaleFormula::from_semitones(&[0, 2, 12, 15]);
        assert_eq!(with_extended.semitones(), vec![0, 2, 12, 15]);

        // Test with out-of-range semitones (should be ignored)
        let with_invalid = ScaleFormula::from_semitones(&[0, 2, 2 * SEMITONES_IN_OCTAVE, 30]);
        assert_eq!(with_invalid.semitones(), vec![0, 2]);
    }

    #[test]
    fn test_bitwise_operations() {
        let major = ScaleFormula::major();
        let minor = ScaleFormula::minor();

        // Union
        let union = major | minor;
        assert!(union.contains_semitone(3)); // Minor 3rd from minor
        assert!(union.contains_semitone(4)); // Major 3rd from major
        assert!(union.contains_semitone(9)); // Major 6th from major
        assert!(union.contains_semitone(8)); // Minor 6th from minor

        // Intersection
        let intersection = major & minor;
        let common_notes = intersection.semitones();
        assert_eq!(common_notes, vec![0, 2, 5, 7]); // 1, 2, 4, 5

        // Complement
        let complement = !major;
        assert!(!complement.contains_semitone(0)); // Root not in complement
        assert!(complement.contains_semitone(1)); // Minor 2nd in complement
        assert!(complement.contains_semitone(3)); // Minor 3rd in complement
    }

    #[test]
    fn test_display() {
        let major = ScaleFormula::major();
        assert_eq!(format!("{}", major), "1, 2, 3, 4, 5, 6, 7");

        let minor = ScaleFormula::minor();
        assert_eq!(format!("{}", minor), "1, 2, ♭3, 4, 5, ♭6, ♭7");

        let empty = ScaleFormula::empty();
        assert_eq!(format!("{}", empty), "Empty");
    }

    #[test]
    fn test_binary_display() {
        let major = ScaleFormula::major();
        assert_eq!(format!("{:b}", major), "101010110101");

        let empty = ScaleFormula::empty();
        assert_eq!(format!("{:b}", empty), "000000000000");
    }

    #[test]
    fn test_equality_and_hash() {
        let major1 = ScaleFormula::major();
        let major2 = ScaleFormula::major();
        let minor = ScaleFormula::minor();

        assert_eq!(major1, major2);
        assert_ne!(major1, minor);

        // Test in hash collections
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(major1);
        set.insert(major2); // Should not increase size
        set.insert(minor); // Should increase size

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_contains_semitone_boundary() {
        let major = ScaleFormula::major();

        // Valid semitones in first octave
        assert!(major.contains_semitone(0));
        assert!(!major.contains_semitone(1));
        assert!(major.contains_semitone(11));

        // Valid semitones in extended range (but not in basic major scale)
        assert!(!major.contains_semitone(SEMITONES_IN_OCTAVE)); // Octave (12)
        assert!(!major.contains_semitone(SEMITONES_IN_OCTAVE + 3)); // Minor 3rd in second octave (15)

        // Invalid semitones (>= 24)
        assert!(!major.contains_semitone(2 * SEMITONES_IN_OCTAVE)); // 24
        assert!(!major.contains_semitone(255));
    }

    #[test]
    fn test_const_functions() {
        // Test that const functions work at compile time
        const MAJOR: ScaleFormula = ScaleFormula::major();
        const EMPTY: ScaleFormula = ScaleFormula::empty();
        const CHROMATIC: ScaleFormula = ScaleFormula::chromatic();

        assert_eq!(MAJOR.note_count(), 7);
        assert!(MAJOR.has_root());
        assert!(!MAJOR.is_empty());

        assert_eq!(EMPTY.note_count(), 0);
        assert!(!EMPTY.has_root());
        assert!(EMPTY.is_empty());

        assert_eq!(CHROMATIC.note_count(), 12);
        assert!(CHROMATIC.has_root());
        assert!(!CHROMATIC.is_empty());
    }

    #[test]
    fn test_notes_from_root() {
        let major = ScaleFormula::major();

        // Test C major scale
        let c_major_notes: Vec<Note> = major.notes_from_root(Note::new(0)).collect();
        let c_major_semitones: Vec<u8> = c_major_notes.iter().map(|note| u8::from(*note)).collect();
        assert_eq!(c_major_semitones, vec![0, 2, 4, 5, 7, 9, 11]);

        // Test F major scale
        let f_major_notes: Vec<Note> = major.notes_from_root(Note::new(5)).collect();
        let f_major_semitones: Vec<u8> = f_major_notes.iter().map(|note| u8::from(*note)).collect();
        assert_eq!(f_major_semitones, vec![5, 7, 9, 10, 12, 14, 16]);

        // Test extended harmony
        let major_extended = ScaleFormula::major_extended();
        let c_extended_count = major_extended.notes_from_root(Note::new(0)).count();
        assert_eq!(c_extended_count, 14);

        // Test pentatonic scale
        let pentatonic = ScaleFormula::pentatonic_major();
        let g_pentatonic_notes: Vec<Note> = pentatonic.notes_from_root(Note::new(7)).collect(); // G
        let g_pentatonic_semitones: Vec<u8> = g_pentatonic_notes
            .iter()
            .map(|note| u8::from(*note))
            .collect();
        assert_eq!(g_pentatonic_semitones, vec![7, 9, 11, 14, 16]); // G, A, B, D, E

        // Test empty scale
        let empty = ScaleFormula::empty();
        let empty_count = empty.notes_from_root(Note::new(0)).count();
        assert_eq!(empty_count, 0);
    }
}
