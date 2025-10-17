//! Semitone implementation for musical intervals.

/// The number of semitones in an octave.
///
/// In Western music theory, an octave contains exactly 12 semitones.
/// This constant is useful for octave calculations and modular arithmetic.
///
/// # Examples
///
/// ```
/// use musik_std::SEMITONES_IN_OCTAVE;
///
/// assert_eq!(SEMITONES_IN_OCTAVE, 12);
///
/// // Use for octave calculations
/// let octave_count = 3;
/// let total_semitones = octave_count * SEMITONES_IN_OCTAVE as usize;
/// assert_eq!(total_semitones, 36);
/// ```
pub const SEMITONES_IN_OCTAVE: u8 = 12;

/// A semitone represents the smallest musical interval in Western music.
///
/// Semitones are the basic unit of measurement for musical intervals.
/// There are 12 semitones in an octave.
///
/// # Examples
///
/// ```
/// use musik_std::Semitone;
///
/// let semitone = Semitone::from(5u8);
/// let value: u8 = semitone.into();
/// assert_eq!(value, 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Semitone(u8);

impl Semitone {
    /// Creates a new `Semitone` from a `u8` value.
    ///
    /// This is a const function that can be used in const contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// const C_TO_C_SHARP: Semitone = Semitone::new(1);
    /// const OCTAVE: Semitone = Semitone::new(12);
    /// ```
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Returns the octave number for this semitone using MIDI convention.
    ///
    /// In MIDI notation, semitone 0 corresponds to C-1, semitone 12 to C0,
    /// semitone 60 to C4 (middle C), etc. This function calculates the octave
    /// number based on this convention.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// // Middle C (MIDI note 60) is in octave 4
    /// let middle_c = Semitone::new(60);
    /// assert_eq!(middle_c.octave().value(), 4);
    ///
    /// // C0 (MIDI note 12) is in octave 0
    /// let c0 = Semitone::new(12);
    /// assert_eq!(c0.octave().value(), 0);
    ///
    /// // C-1 (MIDI note 0) is in octave -1
    /// let c_minus_1 = Semitone::new(0);
    /// assert_eq!(c_minus_1.octave().value(), -1);
    /// ```
    pub fn octave(self) -> crate::Octave {
        // MIDI convention: semitone 0 = C-1, semitone 12 = C0, semitone 60 = C4
        // Formula: octave = (semitone / SEMITONES_IN_OCTAVE) - 1
        let octave_number = (self.0 as i16 / SEMITONES_IN_OCTAVE as i16) - 1;
        crate::Octave::new(octave_number as i8)
    }
}

impl From<u8> for Semitone {
    /// Creates a `Semitone` from a `u8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let semitone = Semitone::from(7u8);
    /// ```
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Semitone> for u8 {
    /// Converts a `Semitone` into a `u8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let semitone = Semitone::from(3u8);
    /// let value: u8 = semitone.into();
    /// assert_eq!(value, 3);
    /// ```
    fn from(val: Semitone) -> Self {
        val.0
    }
}

impl<T> std::ops::Add<T> for Semitone
where
    T: Into<u8>,
{
    type Output = Semitone;

    /// Adds a value that can be converted to `u8` to a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c = Semitone::new(0);  // C
    /// let c_sharp = c + 1u8;     // C# (using u8)
    /// assert_eq!(u8::from(c_sharp), 1);
    ///
    /// // Also works with other types that implement Into<u8>
    /// let d = c + true;          // D (bool converts to 1u8)
    /// assert_eq!(u8::from(d), 1);
    ///
    /// let e = c + Semitone::new(4); // E (Semitone implements Into<u8>)
    /// assert_eq!(u8::from(e), 4);
    /// ```
    fn add(self, rhs: T) -> Self::Output {
        Semitone::new(self.0.wrapping_add(rhs.into()))
    }
}

impl<T> std::ops::Sub<T> for Semitone
where
    T: Into<u8>,
{
    type Output = Semitone;

    /// Subtracts a value that can be converted to `u8` from a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c_sharp = Semitone::new(1);  // C#
    /// let c = c_sharp - 1u8;           // C (using u8)
    /// assert_eq!(u8::from(c), 0);
    ///
    /// // Also works with other types that implement Into<u8>
    /// let f = Semitone::new(5) - true; // F - 1 semitone (bool converts to 1u8)
    /// assert_eq!(u8::from(f), 4);
    ///
    /// let d = Semitone::new(5) - Semitone::new(3); // F - D# interval
    /// assert_eq!(u8::from(d), 2);
    /// ```
    fn sub(self, rhs: T) -> Self::Output {
        Semitone::new(self.0.wrapping_sub(rhs.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semitone_from_u8() {
        let semitone = Semitone::from(0u8);
        let value: u8 = semitone.into();
        assert_eq!(value, 0);

        let semitone = Semitone::from(12u8);
        let value: u8 = semitone.into();
        assert_eq!(value, 12);

        let semitone = Semitone::from(255u8);
        let value: u8 = semitone.into();
        assert_eq!(value, 255);
    }

    #[test]
    fn test_semitone_into_u8() {
        let semitone = Semitone::from(7u8);
        let value: u8 = semitone.into();
        assert_eq!(value, 7);
    }

    #[test]
    fn test_semitone_equality() {
        let s1 = Semitone::from(5u8);
        let s2 = Semitone::from(5u8);
        let s3 = Semitone::from(6u8);

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_semitone_ordering() {
        let s1 = Semitone::from(3u8);
        let s2 = Semitone::from(7u8);

        assert!(s1 < s2);
        assert!(s2 > s1);
        assert!(s1 <= s2);
        assert!(s2 >= s1);
    }

    #[test]
    fn test_semitone_debug() {
        let semitone = Semitone::from(11u8);
        let debug_str = format!("{:?}", semitone);
        assert!(debug_str.contains("Semitone"));
        assert!(debug_str.contains("11"));
    }

    #[test]
    fn test_semitone_size() {
        use std::mem;

        // Test that Semitone is the same size as u8 (no overhead)
        assert_eq!(mem::size_of::<Semitone>(), mem::size_of::<u8>());
        assert_eq!(mem::size_of::<Semitone>(), 1);

        // Test alignment
        assert_eq!(mem::align_of::<Semitone>(), mem::align_of::<u8>());
        assert_eq!(mem::align_of::<Semitone>(), 1);

        // Test that the wrapper doesn't add any runtime overhead
        let semitone = Semitone::from(42u8);
        let value: u8 = semitone.into();
        assert_eq!(value, 42);

        // Verify size_of_val for a specific instance
        assert_eq!(mem::size_of_val(&semitone), 1);
    }

    #[test]
    fn test_semitone_const_new() {
        // Test const function works in const contexts
        const UNISON: Semitone = Semitone::new(0);
        const SEMITONE: Semitone = Semitone::new(1);
        const WHOLE_TONE: Semitone = Semitone::new(2);
        const OCTAVE: Semitone = Semitone::new(12);

        // Test const values are correct
        assert_eq!(u8::from(UNISON), 0);
        assert_eq!(u8::from(SEMITONE), 1);
        assert_eq!(u8::from(WHOLE_TONE), 2);
        assert_eq!(u8::from(OCTAVE), 12);

        // Test that new() and from() produce equivalent results
        let runtime_semitone = Semitone::from(7u8);
        let const_semitone = Semitone::new(7);
        assert_eq!(runtime_semitone, const_semitone);
    }

    #[test]
    fn test_semitones_in_octave_constant() {
        // Test the constant value is correct
        assert_eq!(SEMITONES_IN_OCTAVE, 12);

        // Test using the constant in musical calculations
        let c = Semitone::new(0); // C
        let c_next_octave = c + SEMITONES_IN_OCTAVE;
        assert_eq!(u8::from(c_next_octave), 12);

        // Test octave modulo arithmetic
        let high_note = Semitone::new(25); // C + 2 octaves + 1 semitone
        let normalized = Semitone::new(u8::from(high_note) % SEMITONES_IN_OCTAVE);
        assert_eq!(u8::from(normalized), 1); // Should be C#

        // Test that the constant works in const contexts
        const OCTAVE_SIZE: u8 = SEMITONES_IN_OCTAVE;
        assert_eq!(OCTAVE_SIZE, 12);
    }

    #[test]
    fn test_semitone_add_various_types() {
        // Test basic addition with u8
        let c = Semitone::new(0);
        let c_sharp = c + 1u8;
        assert_eq!(u8::from(c_sharp), 1);

        // Test addition with another Semitone (which implements Into<u8>)
        let f = Semitone::new(5);
        let interval = Semitone::new(1);
        let f_sharp = f + interval;
        assert_eq!(u8::from(f_sharp), 6);

        // Test addition with bool (which implements Into<u8>)
        let g = Semitone::new(7);
        let g_sharp = g + true; // true converts to 1u8
        assert_eq!(u8::from(g_sharp), 8);

        let g_same = g + false; // false converts to 0u8
        assert_eq!(u8::from(g_same), 7);

        // Test that the generic implementation works with our own From trait implementation
        let from_note = crate::Note::new(3);
        let a = Semitone::new(9);
        let a_plus_note = a + from_note;
        assert_eq!(u8::from(a_plus_note), 12); // 9 + 3 = 12

        // Test addition that would go beyond octave
        let b = Semitone::new(11);
        let c_next_octave = b + 1u8;
        assert_eq!(u8::from(c_next_octave), 12);

        // Test wrapping behavior at u8 boundary
        let high_semitone = Semitone::new(254);
        let wrapped = high_semitone + 2u8;
        assert_eq!(u8::from(wrapped), 0); // 254 + 2 = 256 wraps to 0
    }

    #[test]
    fn test_semitone_sub_various_types() {
        // Test basic subtraction with u8
        let c_sharp = Semitone::new(1);
        let c = c_sharp - 1u8;
        assert_eq!(u8::from(c), 0);

        // Test subtraction with another Semitone
        let f_sharp = Semitone::new(6);
        let interval = Semitone::new(1);
        let f = f_sharp - interval;
        assert_eq!(u8::from(f), 5);

        // Test subtraction with bool
        let g_sharp = Semitone::new(8);
        let g = g_sharp - true; // true converts to 1u8
        assert_eq!(u8::from(g), 7);

        let g_same = g_sharp - false; // false converts to 0u8
        assert_eq!(u8::from(g_same), 8);

        // Test subtraction with Note
        let from_note = crate::Note::new(2);
        let d = Semitone::new(2);
        let c = d - from_note;
        assert_eq!(u8::from(c), 0); // 2 - 2 = 0

        // Test subtraction that would go below zero
        let c = Semitone::new(0);
        let wrapped_high = c - 1u8;
        assert_eq!(u8::from(wrapped_high), 255); // 0 - 1 wraps to 255

        // Test larger subtraction
        let d = Semitone::new(2);
        let b_prev_octave = d - 3u8;
        assert_eq!(u8::from(b_prev_octave), 255); // 2 - 3 = -1 wraps to 255
    }

    #[test]
    fn test_semitone_arithmetic_musical_examples() {
        // Musical interval examples using different types
        let c = Semitone::new(0);

        // Major scale intervals from C using various types
        let d = c + 2u8; // Whole step (u8)
        let e = d + Semitone::new(2); // Whole step (Semitone)
        let f = e + true; // Half step (bool -> 1u8)
        let g = f + 2u8; // Whole step (u8)
        let a = g + Semitone::new(2); // Whole step (Semitone)
        let b = a + 2u8; // Whole step (u8)
        let c_octave = b + true; // Half step (bool -> 1u8)

        assert_eq!(u8::from(d), 2);
        assert_eq!(u8::from(e), 4);
        assert_eq!(u8::from(f), 5);
        assert_eq!(u8::from(g), 7);
        assert_eq!(u8::from(a), 9);
        assert_eq!(u8::from(b), 11);
        assert_eq!(u8::from(c_octave), 12);

        // Going back down using mixed types
        let b_from_octave = c_octave - true; // true -> 1u8
        assert_eq!(u8::from(b_from_octave), 11);
        assert_eq!(b_from_octave, b);

        // Test with Note type
        let interval_note = crate::Note::new(5); // Perfect fourth
        let f_from_c = c + interval_note;
        assert_eq!(u8::from(f_from_c), 5);
        assert_eq!(f_from_c, f);
    }

    #[test]
    fn test_semitone_octave() {
        // Test MIDI convention: semitone 0 = C-1, semitone 12 = C0, semitone 60 = C4

        // Test middle C (MIDI note 60) is in octave 4
        let middle_c = Semitone::new(60);
        assert_eq!(middle_c.octave().value(), 4);

        // Test C0 (MIDI note 12) is in octave 0
        let c0 = Semitone::new(12);
        assert_eq!(c0.octave().value(), 0);

        // Test C-1 (MIDI note 0) is in octave -1
        let c_minus_1 = Semitone::new(0);
        assert_eq!(c_minus_1.octave().value(), -1);

        // Test various octaves
        let test_cases = [
            (0, -1),  // C-1
            (11, -1), // B-1
            (12, 0),  // C0
            (23, 0),  // B0
            (24, 1),  // C1
            (35, 1),  // B1
            (36, 2),  // C2
            (47, 2),  // B2
            (48, 3),  // C3
            (59, 3),  // B3
            (60, 4),  // C4 (middle C)
            (71, 4),  // B4
            (72, 5),  // C5
            (84, 6),  // C6
            (96, 7),  // C7
            (108, 8), // C8
            (120, 9), // C9
            (127, 9), // G9 (highest MIDI note)
        ];

        for (semitone, expected_octave) in test_cases.iter() {
            let s = Semitone::new(*semitone);
            let octave = s.octave();
            assert_eq!(
                octave.value(),
                *expected_octave,
                "Semitone {} should be in octave {}, but got {}",
                semitone,
                expected_octave,
                octave.value()
            );
        }

        // Test that consecutive octaves work correctly
        for octave_num in -1..=9 {
            let c_note = Semitone::new(((octave_num + 1) * 12) as u8);
            assert_eq!(c_note.octave().value(), octave_num);
        }
    }
}
