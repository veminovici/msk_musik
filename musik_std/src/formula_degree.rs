//! Formula degrees for chord construction and harmonic analysis.
//!
//! This module provides the `FormulaDegree` type for representing degrees in chord formulas,
//! supporting extended harmony (9ths, 11ths, 13ths) and alterations (flat, sharp).

use std::fmt;

/// Represents a degree in a chord formula, supporting extended harmony (9, 11, 13, etc.)
/// and alterations (flat, sharp). Used for chord construction and analysis.
///
/// # Examples
/// ```
/// use musik_std::FormulaDegree;
///
/// // Basic triad degrees
/// let root = FormulaDegree::natural(1);           // 1
/// let third = FormulaDegree::natural(3);          // 3
/// let flat_third = FormulaDegree::flat(3);        // ♭3
///
/// // Extended harmony
/// let ninth = FormulaDegree::natural(9);          // 9
/// let flat_ninth = FormulaDegree::flat(9);        // ♭9
/// let sharp_ninth = FormulaDegree::sharp(9);      // ♯9
/// let eleventh = FormulaDegree::natural(11);      // 11
/// let sharp_eleventh = FormulaDegree::sharp(11);  // ♯11
/// let thirteenth = FormulaDegree::natural(13);    // 13
/// let flat_thirteenth = FormulaDegree::flat(13);  // ♭13
///
/// // Common jazz chord degrees
/// let sharp_four = FormulaDegree::sharp(4);       // ♯4 (same as ♯11)
/// let flat_seven = FormulaDegree::flat(7);        // ♭7
/// let maj_seven = FormulaDegree::natural(7);      // 7
///
/// assert_eq!(ninth.symbol(), "9");
/// assert_eq!(flat_ninth.symbol(), "♭9");
/// assert_eq!(sharp_eleventh.symbol(), "♯11");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormulaDegree {
    /// Natural degree (1, 2, 3, 4, 5, 6, 7, 9, 11, 13, etc.)
    Natural(u8),
    /// Flat degree (♭2, ♭3, ♭5, ♭6, ♭7, ♭9, ♭11, ♭13, etc.)
    Flat(u8),
    /// Sharp degree (♯1, ♯2, ♯4, ♯5, ♯6, ♯9, ♯11, etc.)
    Sharp(u8),
}

impl FormulaDegree {
    /// Create a natural formula degree
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// let root = FormulaDegree::natural(1);    // 1
    /// let ninth = FormulaDegree::natural(9);   // 9
    /// let eleventh = FormulaDegree::natural(11); // 11
    /// ```
    pub const fn natural(degree: u8) -> Self {
        FormulaDegree::Natural(degree)
    }

    /// Create a flat formula degree
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// let flat_third = FormulaDegree::flat(3);   // ♭3
    /// let flat_ninth = FormulaDegree::flat(9);   // ♭9
    /// let flat_thirteen = FormulaDegree::flat(13); // ♭13
    /// ```
    pub const fn flat(degree: u8) -> Self {
        FormulaDegree::Flat(degree)
    }

    /// Create a sharp formula degree
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// let sharp_fourth = FormulaDegree::sharp(4);  // ♯4
    /// let sharp_ninth = FormulaDegree::sharp(9);   // ♯9
    /// let sharp_eleventh = FormulaDegree::sharp(11); // ♯11
    /// ```
    pub const fn sharp(degree: u8) -> Self {
        FormulaDegree::Sharp(degree)
    }

    /// Get the base degree number (1, 2, 3, 4, 5, 6, 7, 9, 11, 13, etc.)
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert_eq!(FormulaDegree::natural(9).base_degree(), 9);
    /// assert_eq!(FormulaDegree::flat(13).base_degree(), 13);
    /// assert_eq!(FormulaDegree::sharp(11).base_degree(), 11);
    /// ```
    pub const fn base_degree(&self) -> u8 {
        match self {
            FormulaDegree::Natural(d) | FormulaDegree::Flat(d) | FormulaDegree::Sharp(d) => *d,
        }
    }

    /// Get the alteration (-1 for flat, 0 for natural, +1 for sharp)
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert_eq!(FormulaDegree::natural(9).alteration(), 0);
    /// assert_eq!(FormulaDegree::flat(9).alteration(), -1);
    /// assert_eq!(FormulaDegree::sharp(9).alteration(), 1);
    /// ```
    pub const fn alteration(&self) -> i8 {
        match self {
            FormulaDegree::Natural(_) => 0,
            FormulaDegree::Flat(_) => -1,
            FormulaDegree::Sharp(_) => 1,
        }
    }

    /// Convert formula degree to semitone offset from root in a major scale context.
    /// Extended degrees (9, 11, 13) are reduced to their octave equivalents (2, 4, 6).
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// // Basic degrees
    /// assert_eq!(FormulaDegree::natural(1).to_semitone_offset(), Some(0));  // Root
    /// assert_eq!(FormulaDegree::natural(3).to_semitone_offset(), Some(4));  // Major 3rd
    /// assert_eq!(FormulaDegree::flat(3).to_semitone_offset(), Some(3));     // Minor 3rd
    ///
    /// // Extended degrees (reduced to octave)
    /// assert_eq!(FormulaDegree::natural(9).to_semitone_offset(), Some(2));  // 9th = 2nd
    /// assert_eq!(FormulaDegree::flat(9).to_semitone_offset(), Some(1));     // ♭9th = ♭2nd
    /// assert_eq!(FormulaDegree::sharp(9).to_semitone_offset(), Some(3));    // ♯9th = ♯2nd
    /// assert_eq!(FormulaDegree::natural(11).to_semitone_offset(), Some(5)); // 11th = 4th
    /// assert_eq!(FormulaDegree::sharp(11).to_semitone_offset(), Some(6));   // ♯11th = ♯4th
    /// assert_eq!(FormulaDegree::natural(13).to_semitone_offset(), Some(9)); // 13th = 6th
    /// assert_eq!(FormulaDegree::flat(13).to_semitone_offset(), Some(8));    // ♭13th = ♭6th
    /// ```
    pub fn to_semitone_offset(&self) -> Option<u8> {
        // Reduce extended degrees to their basic equivalents within the octave
        let reduced_degree = match self.base_degree() {
            d if d <= 7 => d,
            8 => 1,           // 8th = 1st (octave)
            9 => 2,           // 9th = 2nd
            10 => 3,          // 10th = 3rd
            11 => 4,          // 11th = 4th
            12 => 5,          // 12th = 5th
            13 => 6,          // 13th = 6th
            14 => 7,          // 14th = 7th
            15 => 1,          // 15th = 1st (double octave)
            _ => return None, // Degrees beyond 15 are not supported
        };

        let base_semitones = match reduced_degree {
            1 => 0,  // Root
            2 => 2,  // Major 2nd
            3 => 4,  // Major 3rd
            4 => 5,  // Perfect 4th
            5 => 7,  // Perfect 5th
            6 => 9,  // Major 6th
            7 => 11, // Major 7th
            _ => return None,
        };

        let adjusted = base_semitones as i8 + self.alteration();
        // Handle negative values by wrapping around the octave
        let result = if adjusted < 0 {
            (12 + adjusted) as u8
        } else {
            (adjusted as u8) % 12
        };

        Some(result)
    }

    /// Get the symbol representation of the formula degree
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert_eq!(FormulaDegree::natural(1).symbol(), "1");
    /// assert_eq!(FormulaDegree::natural(9).symbol(), "9");
    /// assert_eq!(FormulaDegree::flat(9).symbol(), "♭9");
    /// assert_eq!(FormulaDegree::sharp(11).symbol(), "♯11");
    /// assert_eq!(FormulaDegree::flat(13).symbol(), "♭13");
    /// ```
    pub fn symbol(&self) -> String {
        match self {
            FormulaDegree::Natural(d) => d.to_string(),
            FormulaDegree::Flat(d) => format!("♭{}", d),
            FormulaDegree::Sharp(d) => format!("♯{}", d),
        }
    }

    /// Get the name representation of the formula degree
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert_eq!(FormulaDegree::natural(9).name(), "9");
    /// assert_eq!(FormulaDegree::flat(9).name(), "flat 9");
    /// assert_eq!(FormulaDegree::sharp(11).name(), "sharp 11");
    /// ```
    pub fn name(&self) -> String {
        match self {
            FormulaDegree::Natural(d) => d.to_string(),
            FormulaDegree::Flat(d) => format!("flat {}", d),
            FormulaDegree::Sharp(d) => format!("sharp {}", d),
        }
    }

    /// Check if this is an extended degree (beyond the basic 1-7)
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert!(!FormulaDegree::natural(1).is_extended());
    /// assert!(!FormulaDegree::natural(7).is_extended());
    /// assert!(FormulaDegree::natural(9).is_extended());
    /// assert!(FormulaDegree::flat(9).is_extended());
    /// assert!(FormulaDegree::sharp(11).is_extended());
    /// assert!(FormulaDegree::natural(13).is_extended());
    /// ```
    pub const fn is_extended(&self) -> bool {
        self.base_degree() > 7
    }

    /// Check if this is a basic chord tone (1, 3, 5, 7)
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert!(FormulaDegree::natural(1).is_chord_tone());
    /// assert!(FormulaDegree::natural(3).is_chord_tone());
    /// assert!(FormulaDegree::flat(3).is_chord_tone());
    /// assert!(FormulaDegree::natural(5).is_chord_tone());
    /// assert!(FormulaDegree::natural(7).is_chord_tone());
    /// assert!(FormulaDegree::flat(7).is_chord_tone());
    ///
    /// assert!(!FormulaDegree::natural(2).is_chord_tone());
    /// assert!(!FormulaDegree::natural(9).is_chord_tone());
    /// assert!(!FormulaDegree::natural(11).is_chord_tone());
    /// ```
    pub const fn is_chord_tone(&self) -> bool {
        matches!(self.base_degree(), 1 | 3 | 5 | 7)
    }

    /// Check if this is a tension/extension (2, 4, 6, 9, 11, 13)
    ///
    /// # Examples
    /// ```
    /// use musik_std::FormulaDegree;
    ///
    /// assert!(FormulaDegree::natural(2).is_tension());
    /// assert!(FormulaDegree::natural(4).is_tension());
    /// assert!(FormulaDegree::sharp(4).is_tension());
    /// assert!(FormulaDegree::natural(6).is_tension());
    /// assert!(FormulaDegree::natural(9).is_tension());
    /// assert!(FormulaDegree::flat(9).is_tension());
    /// assert!(FormulaDegree::sharp(11).is_tension());
    /// assert!(FormulaDegree::natural(13).is_tension());
    ///
    /// assert!(!FormulaDegree::natural(1).is_tension());
    /// assert!(!FormulaDegree::natural(3).is_tension());
    /// assert!(!FormulaDegree::natural(5).is_tension());
    /// ```
    pub const fn is_tension(&self) -> bool {
        matches!(self.base_degree(), 2 | 4 | 6 | 9 | 11 | 13)
    }
}

impl fmt::Display for FormulaDegree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula_degree_creation() {
        // Natural degrees
        let root = FormulaDegree::natural(1);
        let third = FormulaDegree::natural(3);
        let ninth = FormulaDegree::natural(9);

        assert_eq!(root.base_degree(), 1);
        assert_eq!(third.base_degree(), 3);
        assert_eq!(ninth.base_degree(), 9);
        assert_eq!(root.alteration(), 0);
        assert_eq!(third.alteration(), 0);
        assert_eq!(ninth.alteration(), 0);

        // Flat degrees
        let flat_third = FormulaDegree::flat(3);
        let flat_ninth = FormulaDegree::flat(9);
        let flat_thirteen = FormulaDegree::flat(13);

        assert_eq!(flat_third.base_degree(), 3);
        assert_eq!(flat_ninth.base_degree(), 9);
        assert_eq!(flat_thirteen.base_degree(), 13);
        assert_eq!(flat_third.alteration(), -1);
        assert_eq!(flat_ninth.alteration(), -1);
        assert_eq!(flat_thirteen.alteration(), -1);

        // Sharp degrees
        let sharp_fourth = FormulaDegree::sharp(4);
        let sharp_ninth = FormulaDegree::sharp(9);
        let sharp_eleventh = FormulaDegree::sharp(11);

        assert_eq!(sharp_fourth.base_degree(), 4);
        assert_eq!(sharp_ninth.base_degree(), 9);
        assert_eq!(sharp_eleventh.base_degree(), 11);
        assert_eq!(sharp_fourth.alteration(), 1);
        assert_eq!(sharp_ninth.alteration(), 1);
        assert_eq!(sharp_eleventh.alteration(), 1);
    }

    #[test]
    fn test_formula_degree_symbols() {
        assert_eq!(FormulaDegree::natural(1).symbol(), "1");
        assert_eq!(FormulaDegree::natural(9).symbol(), "9");
        assert_eq!(FormulaDegree::flat(3).symbol(), "♭3");
        assert_eq!(FormulaDegree::flat(9).symbol(), "♭9");
        assert_eq!(FormulaDegree::sharp(4).symbol(), "♯4");
        assert_eq!(FormulaDegree::sharp(11).symbol(), "♯11");
        assert_eq!(FormulaDegree::flat(13).symbol(), "♭13");
    }

    #[test]
    fn test_formula_degree_names() {
        assert_eq!(FormulaDegree::natural(1).name(), "1");
        assert_eq!(FormulaDegree::natural(9).name(), "9");
        assert_eq!(FormulaDegree::flat(3).name(), "flat 3");
        assert_eq!(FormulaDegree::flat(9).name(), "flat 9");
        assert_eq!(FormulaDegree::sharp(4).name(), "sharp 4");
        assert_eq!(FormulaDegree::sharp(11).name(), "sharp 11");
    }

    #[test]
    fn test_formula_degree_display() {
        assert_eq!(format!("{}", FormulaDegree::natural(1)), "1");
        assert_eq!(format!("{}", FormulaDegree::flat(9)), "♭9");
        assert_eq!(format!("{}", FormulaDegree::sharp(11)), "♯11");
    }

    #[test]
    fn test_formula_degree_semitone_offsets() {
        // Basic degrees (1-7)
        assert_eq!(FormulaDegree::natural(1).to_semitone_offset(), Some(0));
        assert_eq!(FormulaDegree::natural(2).to_semitone_offset(), Some(2));
        assert_eq!(FormulaDegree::natural(3).to_semitone_offset(), Some(4));
        assert_eq!(FormulaDegree::flat(3).to_semitone_offset(), Some(3));
        assert_eq!(FormulaDegree::natural(4).to_semitone_offset(), Some(5));
        assert_eq!(FormulaDegree::sharp(4).to_semitone_offset(), Some(6));
        assert_eq!(FormulaDegree::natural(5).to_semitone_offset(), Some(7));
        assert_eq!(FormulaDegree::natural(6).to_semitone_offset(), Some(9));
        assert_eq!(FormulaDegree::natural(7).to_semitone_offset(), Some(11));
        assert_eq!(FormulaDegree::flat(7).to_semitone_offset(), Some(10));

        // Extended degrees (reduced to octave equivalents)
        assert_eq!(FormulaDegree::natural(9).to_semitone_offset(), Some(2)); // 9th = 2nd
        assert_eq!(FormulaDegree::flat(9).to_semitone_offset(), Some(1)); // ♭9th = ♭2nd
        assert_eq!(FormulaDegree::sharp(9).to_semitone_offset(), Some(3)); // ♯9th = ♯2nd
        assert_eq!(FormulaDegree::natural(11).to_semitone_offset(), Some(5)); // 11th = 4th
        assert_eq!(FormulaDegree::sharp(11).to_semitone_offset(), Some(6)); // ♯11th = ♯4th
        assert_eq!(FormulaDegree::natural(13).to_semitone_offset(), Some(9)); // 13th = 6th
        assert_eq!(FormulaDegree::flat(13).to_semitone_offset(), Some(8)); // ♭13th = ♭6th
    }

    #[test]
    fn test_formula_degree_extended_check() {
        // Basic degrees (1-7) are not extended
        assert!(!FormulaDegree::natural(1).is_extended());
        assert!(!FormulaDegree::natural(3).is_extended());
        assert!(!FormulaDegree::flat(3).is_extended());
        assert!(!FormulaDegree::natural(5).is_extended());
        assert!(!FormulaDegree::natural(7).is_extended());
        assert!(!FormulaDegree::flat(7).is_extended());

        // Extended degrees (8+) are extended
        assert!(FormulaDegree::natural(8).is_extended());
        assert!(FormulaDegree::natural(9).is_extended());
        assert!(FormulaDegree::flat(9).is_extended());
        assert!(FormulaDegree::sharp(9).is_extended());
        assert!(FormulaDegree::natural(11).is_extended());
        assert!(FormulaDegree::sharp(11).is_extended());
        assert!(FormulaDegree::natural(13).is_extended());
        assert!(FormulaDegree::flat(13).is_extended());
    }

    #[test]
    fn test_formula_degree_chord_tone_check() {
        // Chord tones (1, 3, 5, 7)
        assert!(FormulaDegree::natural(1).is_chord_tone());
        assert!(FormulaDegree::natural(3).is_chord_tone());
        assert!(FormulaDegree::flat(3).is_chord_tone());
        assert!(FormulaDegree::sharp(3).is_chord_tone());
        assert!(FormulaDegree::natural(5).is_chord_tone());
        assert!(FormulaDegree::flat(5).is_chord_tone());
        assert!(FormulaDegree::sharp(5).is_chord_tone());
        assert!(FormulaDegree::natural(7).is_chord_tone());
        assert!(FormulaDegree::flat(7).is_chord_tone());

        // Non-chord tones
        assert!(!FormulaDegree::natural(2).is_chord_tone());
        assert!(!FormulaDegree::natural(4).is_chord_tone());
        assert!(!FormulaDegree::natural(6).is_chord_tone());
        assert!(!FormulaDegree::natural(9).is_chord_tone());
        assert!(!FormulaDegree::natural(11).is_chord_tone());
        assert!(!FormulaDegree::natural(13).is_chord_tone());
    }

    #[test]
    fn test_formula_degree_tension_check() {
        // Tensions (2, 4, 6, 9, 11, 13)
        assert!(FormulaDegree::natural(2).is_tension());
        assert!(FormulaDegree::flat(2).is_tension());
        assert!(FormulaDegree::natural(4).is_tension());
        assert!(FormulaDegree::sharp(4).is_tension());
        assert!(FormulaDegree::natural(6).is_tension());
        assert!(FormulaDegree::flat(6).is_tension());
        assert!(FormulaDegree::natural(9).is_tension());
        assert!(FormulaDegree::flat(9).is_tension());
        assert!(FormulaDegree::sharp(9).is_tension());
        assert!(FormulaDegree::natural(11).is_tension());
        assert!(FormulaDegree::sharp(11).is_tension());
        assert!(FormulaDegree::natural(13).is_tension());
        assert!(FormulaDegree::flat(13).is_tension());

        // Non-tensions (chord tones)
        assert!(!FormulaDegree::natural(1).is_tension());
        assert!(!FormulaDegree::natural(3).is_tension());
        assert!(!FormulaDegree::natural(5).is_tension());
        assert!(!FormulaDegree::natural(7).is_tension());
    }

    #[test]
    fn test_formula_degree_jazz_chord_examples() {
        // Common jazz chord formulas using FormulaDegree

        // Dominant 7th with tensions: 1, 3, 5, ♭7, 9, 13
        let dom7_add_tensions = [
            FormulaDegree::natural(1),
            FormulaDegree::natural(3),
            FormulaDegree::natural(5),
            FormulaDegree::flat(7),
            FormulaDegree::natural(9),
            FormulaDegree::natural(13),
        ];
        assert_eq!(dom7_add_tensions.len(), 6);

        // Altered dominant: 1, 3, ♭5, ♭7, ♭9, ♯9, ♯11, ♭13
        let altered_dom = [
            FormulaDegree::natural(1),
            FormulaDegree::natural(3),
            FormulaDegree::flat(5),
            FormulaDegree::flat(7),
            FormulaDegree::flat(9),
            FormulaDegree::sharp(9),
            FormulaDegree::sharp(11),
            FormulaDegree::flat(13),
        ];
        assert_eq!(altered_dom.len(), 8);

        // Major 7 sharp 11: 1, 3, 5, 7, ♯11
        let maj7_sharp11 = [
            FormulaDegree::natural(1),
            FormulaDegree::natural(3),
            FormulaDegree::natural(5),
            FormulaDegree::natural(7),
            FormulaDegree::sharp(11),
        ];
        assert_eq!(maj7_sharp11.len(), 5);

        // Verify semitone calculations for altered tensions
        assert_eq!(FormulaDegree::flat(9).to_semitone_offset(), Some(1)); // ♭9 = ♭2
        assert_eq!(FormulaDegree::sharp(9).to_semitone_offset(), Some(3)); // ♯9 = ♯2
        assert_eq!(FormulaDegree::sharp(11).to_semitone_offset(), Some(6)); // ♯11 = ♯4
        assert_eq!(FormulaDegree::flat(13).to_semitone_offset(), Some(8)); // ♭13 = ♭6
    }

    #[test]
    fn test_formula_degree_equality_and_hash() {
        let degree1 = FormulaDegree::natural(9);
        let degree2 = FormulaDegree::natural(9);
        let degree3 = FormulaDegree::flat(9);

        assert_eq!(degree1, degree2);
        assert_ne!(degree1, degree3);

        // Test that they can be used in hash collections
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(degree1);
        set.insert(degree2); // Should not increase size
        set.insert(degree3); // Should increase size

        assert_eq!(set.len(), 2);
        assert!(set.contains(&FormulaDegree::natural(9)));
        assert!(set.contains(&FormulaDegree::flat(9)));
    }

    #[test]
    fn test_formula_degree_const_functions() {
        // Test that const functions work at compile time
        const ROOT: FormulaDegree = FormulaDegree::natural(1);
        const FLAT_NINTH: FormulaDegree = FormulaDegree::flat(9);
        const SHARP_ELEVEN: FormulaDegree = FormulaDegree::sharp(11);

        assert_eq!(ROOT.base_degree(), 1);
        assert_eq!(ROOT.alteration(), 0);
        assert!(!ROOT.is_extended());
        assert!(ROOT.is_chord_tone());
        assert!(!ROOT.is_tension());

        assert_eq!(FLAT_NINTH.base_degree(), 9);
        assert_eq!(FLAT_NINTH.alteration(), -1);
        assert!(FLAT_NINTH.is_extended());
        assert!(!FLAT_NINTH.is_chord_tone());
        assert!(FLAT_NINTH.is_tension());

        assert_eq!(SHARP_ELEVEN.base_degree(), 11);
        assert_eq!(SHARP_ELEVEN.alteration(), 1);
        assert!(SHARP_ELEVEN.is_extended());
        assert!(!SHARP_ELEVEN.is_chord_tone());
        assert!(SHARP_ELEVEN.is_tension());
    }
}
