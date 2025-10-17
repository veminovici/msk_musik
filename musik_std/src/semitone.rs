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
    pub const fn octave(self) -> crate::Octave {
        // MIDI convention: semitone 0 = C-1, semitone 12 = C0, semitone 60 = C4
        // Formula: octave = (semitone / SEMITONES_IN_OCTAVE) - 1
        let octave_number = (self.0 as i16 / SEMITONES_IN_OCTAVE as i16) - 1;
        crate::Octave::new(octave_number as i8)
    }

    /// Returns the pitch class for this semitone.
    ///
    /// The pitch class represents the chroma of the semitone, ignoring octave information.
    /// This is calculated using modular arithmetic with the number of semitones in an octave (12).
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Semitone, PitchClass};
    ///
    /// // C in different octaves all have the same pitch class
    /// let c0 = Semitone::new(12);   // C0
    /// let c4 = Semitone::new(60);   // C4 (middle C)
    /// let c8 = Semitone::new(108);  // C8
    ///
    /// assert_eq!(c0.pitch_class(), PitchClass::new(0));
    /// assert_eq!(c4.pitch_class(), PitchClass::new(0));
    /// assert_eq!(c8.pitch_class(), PitchClass::new(0));
    ///
    /// // Different pitch classes
    /// let f_sharp = Semitone::new(66); // F#4
    /// assert_eq!(f_sharp.pitch_class(), PitchClass::new(6));
    /// ```
    pub const fn pitch_class(self) -> crate::PitchClass {
        crate::PitchClass::new(self.0 % SEMITONES_IN_OCTAVE)
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

impl std::ops::Shr<u8> for Semitone {
    type Output = Semitone;

    /// Shifts a `Semitone` up by the given number of octaves.
    ///
    /// This operation moves the semitone up by N octaves, where each octave
    /// contains 12 semitones. The shift right operator (`>>`) is used as
    /// a musical metaphor for octave transposition.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c = Semitone::new(0);        // C-1 (MIDI note 0)
    /// let c_up_1_octave = c >> 1u8;    // C0 (MIDI note 12)
    /// assert_eq!(u8::from(c_up_1_octave), 12);
    ///
    /// let c4 = Semitone::new(60);      // C4 (middle C)
    /// let c6 = c4 >> 2u8;              // C6 (2 octaves up)
    /// assert_eq!(u8::from(c6), 84);
    ///
    /// // Works with any note in the octave
    /// let f_sharp = Semitone::new(6);  // F#-1
    /// let f_sharp_up = f_sharp >> 3u8; // F#2 (3 octaves up)
    /// assert_eq!(u8::from(f_sharp_up), 42);
    /// ```
    fn shr(self, rhs: u8) -> Self::Output {
        let octaves_in_semitones = rhs.saturating_mul(SEMITONES_IN_OCTAVE);
        Semitone::new(self.0.saturating_add(octaves_in_semitones))
    }
}

impl std::ops::Shl<u8> for Semitone {
    type Output = Semitone;

    /// Shifts a `Semitone` down by the given number of octaves.
    ///
    /// This operation moves the semitone down by N octaves, where each octave
    /// contains 12 semitones. The shift left operator (`<<`) is used as
    /// a musical metaphor for octave transposition downward.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c4 = Semitone::new(60);      // C4 (middle C)
    /// let c3 = c4 << 1u8;              // C3 (1 octave down)
    /// assert_eq!(u8::from(c3), 48);
    ///
    /// let c0 = c4 << 5u8;              // C-1 (5 octaves down)
    /// assert_eq!(u8::from(c0), 0);
    ///
    /// // Works with any note in the octave
    /// let f_sharp4 = Semitone::new(66); // F#4
    /// let f_sharp2 = f_sharp4 << 2u8;   // F#2 (2 octaves down)
    /// assert_eq!(u8::from(f_sharp2), 42);
    /// ```
    fn shl(self, rhs: u8) -> Self::Output {
        let octaves_in_semitones = rhs.saturating_mul(SEMITONES_IN_OCTAVE);
        Semitone::new(self.0.saturating_sub(octaves_in_semitones))
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

    #[test]
    fn test_semitone_pitch_class() {
        // Test that semitones in different octaves have the same pitch class

        // Test C in different octaves (all should have pitch class 0)
        let c_minus_1 = Semitone::new(0); // C-1
        let c0 = Semitone::new(12); // C0
        let c4 = Semitone::new(60); // C4 (middle C)
        let c8 = Semitone::new(108); // C8

        assert_eq!(c_minus_1.pitch_class().value(), 0);
        assert_eq!(c0.pitch_class().value(), 0);
        assert_eq!(c4.pitch_class().value(), 0);
        assert_eq!(c8.pitch_class().value(), 0);

        // Test all 12 pitch classes with different octaves
        let test_cases = [
            (0, 0),   // C
            (1, 1),   // C#/Db
            (2, 2),   // D
            (3, 3),   // D#/Eb
            (4, 4),   // E
            (5, 5),   // F
            (6, 6),   // F#/Gb
            (7, 7),   // G
            (8, 8),   // G#/Ab
            (9, 9),   // A
            (10, 10), // A#/Bb
            (11, 11), // B
        ];

        for (semitone_offset, expected_pitch_class) in test_cases.iter() {
            // Test in multiple octaves
            for octave in 0..=9 {
                let semitone_value = octave * 12 + semitone_offset;
                let semitone = Semitone::new(semitone_value);
                let pitch_class = semitone.pitch_class();

                assert_eq!(
                    pitch_class.value(),
                    *expected_pitch_class,
                    "Semitone {} (octave {}, offset {}) should have pitch class {}, but got {}",
                    semitone_value,
                    octave,
                    semitone_offset,
                    expected_pitch_class,
                    pitch_class.value()
                );
            }
        }

        // Test edge cases
        let max_semitone = Semitone::new(255);
        let expected_pitch_class = 255 % 12; // Should be 3 (D#/Eb)
        assert_eq!(max_semitone.pitch_class().value(), expected_pitch_class);
    }

    #[test]
    fn test_semitone_shr_octave_shift() {
        // Test basic octave shifting with Shr operator

        // Test C-1 (semitone 0) shifted up by octaves
        let c_minus_1 = Semitone::new(0);
        let c0 = c_minus_1 >> 1u8; // 1 octave up
        assert_eq!(u8::from(c0), 12);

        let c1 = c_minus_1 >> 2u8; // 2 octaves up
        assert_eq!(u8::from(c1), 24);

        let c4 = c_minus_1 >> 5u8; // 5 octaves up (middle C)
        assert_eq!(u8::from(c4), 60);

        // Test middle C (semitone 60) shifted up
        let middle_c = Semitone::new(60);
        let c5 = middle_c >> 1u8; // 1 octave up
        assert_eq!(u8::from(c5), 72);

        let c6 = middle_c >> 2u8; // 2 octaves up
        assert_eq!(u8::from(c6), 84);

        let c7 = middle_c >> 3u8; // 3 octaves up
        assert_eq!(u8::from(c7), 96);

        // Test with non-C notes to ensure pitch class is preserved
        let f_sharp = Semitone::new(6); // F#-1
        let f_sharp_0 = f_sharp >> 1u8; // F#0
        assert_eq!(u8::from(f_sharp_0), 18);

        let f_sharp_2 = f_sharp >> 3u8; // F#2
        assert_eq!(u8::from(f_sharp_2), 42);

        // Test zero octave shift (no change)
        let g = Semitone::new(7);
        let g_same = g >> 0u8;
        assert_eq!(u8::from(g_same), 7);
    }

    #[test]
    fn test_semitone_shr_pitch_class_preservation() {
        // Test that octave shifting preserves the pitch class (note within octave)

        // Generate all pitch classes (0-11) and test octave shifts (1-5)
        (0..12u8)
            .map(Semitone::new)
            .enumerate()
            .for_each(|(pitch_class, original)| {
                (1..=5u8)
                    .map(|octaves| (octaves, original >> octaves))
                    .map(|(octaves, shifted)| {
                        (octaves, shifted, u8::from(shifted), pitch_class as u8)
                    })
                    .for_each(|(octaves, shifted, shifted_value, expected_pitch_class)| {
                        // Verify the absolute value is correct
                        let expected_value = expected_pitch_class + (octaves * SEMITONES_IN_OCTAVE);
                        assert_eq!(
                            shifted_value, expected_value,
                            "Pitch class {}: {} >> {} octaves should be {}, got {}",
                            expected_pitch_class, expected_pitch_class, octaves, expected_value, shifted_value
                        );

                        // Verify pitch class is preserved using modulo
                        let actual_pitch_class = shifted_value % SEMITONES_IN_OCTAVE;
                        assert_eq!(
                            actual_pitch_class, expected_pitch_class,
                            "Pitch class should be preserved: {} >> {} octaves, expected pitch class {}, got {}",
                            expected_pitch_class, octaves, expected_pitch_class, actual_pitch_class
                        );

                        // Verify the shifted semitone can be converted back
                        assert_eq!(u8::from(shifted), shifted_value);
                    });
            });

        // Additional functional test: verify all combinations produce unique results within each octave
        let all_results: Vec<_> = (0..12u8)
            .flat_map(|pitch_class| {
                (1..=5u8).map(move |octaves| {
                    let original = Semitone::new(pitch_class);
                    let shifted = original >> octaves;
                    (pitch_class, octaves, u8::from(shifted))
                })
            })
            .collect();

        // Verify that within each octave range, all pitch classes are represented
        (1..=5u8).for_each(|target_octaves| {
            let octave_results: std::collections::HashSet<u8> = all_results
                .iter()
                .filter(|(_, octaves, _)| *octaves == target_octaves)
                .map(|(original_pitch_class, _, shifted_value)| {
                    let computed_pitch_class = shifted_value % 12;
                    // Verify consistency: the computed pitch class should match the original
                    assert_eq!(computed_pitch_class, *original_pitch_class);
                    computed_pitch_class
                })
                .collect();

            assert_eq!(
                octave_results.len(),
                12,
                "Octave {} should have all 12 pitch classes represented",
                target_octaves
            );
        });
    }

    #[test]
    fn test_semitone_shr_saturation_behavior() {
        // Test saturation behavior at u8 boundaries

        // Test with high semitone values that would overflow
        let high_semitone = Semitone::new(200);
        let shifted = high_semitone >> 5u8; // 5 octaves = 60 semitones
                                            // 200 + 60 = 260, but u8 max is 255, so should saturate at 255
        assert_eq!(u8::from(shifted), 255);

        // Test with maximum u8 value
        let max_semitone = Semitone::new(255);
        let shifted_max = max_semitone >> 1u8; // 1 octave = 12 semitones
                                               // 255 + 12 would overflow, should saturate at 255
        assert_eq!(u8::from(shifted_max), 255);

        // Test edge case: exactly at the boundary
        let boundary_test = Semitone::new(243); // 255 - 12 = 243
        let boundary_shifted = boundary_test >> 1u8;
        assert_eq!(u8::from(boundary_shifted), 255); // Should exactly hit 255

        // Test that smaller shifts from boundary work correctly
        let near_boundary = Semitone::new(250);
        let small_shift = near_boundary >> 0u8; // No shift
        assert_eq!(u8::from(small_shift), 250);
    }

    #[test]
    fn test_semitone_shr_large_octave_shifts() {
        // Test with large octave shift values

        let c = Semitone::new(0);

        // Test large shift that would cause overflow
        let large_shift = c >> 50u8; // 50 octaves = 600 semitones
                                     // 0 + 600 would overflow u8, should saturate at 255
        assert_eq!(u8::from(large_shift), 255);

        // Test maximum possible u8 octave value
        let max_octave_shift = c >> 255u8;
        assert_eq!(u8::from(max_octave_shift), 255);

        // Test moderate values that work within range
        let moderate_note = Semitone::new(10);
        let moderate_shift = moderate_note >> 10u8; // 10 octaves = 120 semitones
                                                    // 10 + 120 = 130, within u8 range
        assert_eq!(u8::from(moderate_shift), 130);
    }

    #[test]
    fn test_semitone_shr_musical_examples() {
        // Test realistic musical octave transposition examples

        // Piano range examples (A0 = 21, C8 = 108 in MIDI)
        let a0 = Semitone::new(21); // A0
        let a1 = a0 >> 1u8; // A1
        assert_eq!(u8::from(a1), 33);

        let a4 = a0 >> 4u8; // A4 (440 Hz)
        assert_eq!(u8::from(a4), 69);

        // Orchestra tuning: A4 transposed to different octaves
        let a4_standard = Semitone::new(69); // A4 = 440 Hz
        let a5 = a4_standard >> 1u8; // A5 = 880 Hz
        assert_eq!(u8::from(a5), 81);

        let a3 = Semitone::new(57); // A3 = 220 Hz
        let a4_from_a3 = a3 >> 1u8;
        assert_eq!(u8::from(a4_from_a3), 69);
        assert_eq!(a4_from_a3, a4_standard);

        // Guitar string transposition (6th string E2 to higher octaves)
        let e2 = Semitone::new(40); // E2 (low E on guitar)
        let e3 = e2 >> 1u8; // E3
        let e4 = e2 >> 2u8; // E4
        let e5 = e2 >> 3u8; // E5

        assert_eq!(u8::from(e3), 52);
        assert_eq!(u8::from(e4), 64);
        assert_eq!(u8::from(e5), 76);

        // Verify all Es have the same pitch class (4)
        assert_eq!(u8::from(e2) % 12, 4);
        assert_eq!(u8::from(e3) % 12, 4);
        assert_eq!(u8::from(e4) % 12, 4);
        assert_eq!(u8::from(e5) % 12, 4);
    }

    #[test]
    fn test_semitone_shr_with_constant() {
        // Test using the SEMITONES_IN_OCTAVE constant to verify calculations

        let base_note = Semitone::new(25); // C#1

        // Manual calculation: 1 octave up
        let manual_calc = Semitone::new(25 + SEMITONES_IN_OCTAVE);
        let shr_calc = base_note >> 1u8;
        assert_eq!(manual_calc, shr_calc);

        // Manual calculation: 3 octaves up
        let manual_calc_3 = Semitone::new(25 + (3 * SEMITONES_IN_OCTAVE));
        let shr_calc_3 = base_note >> 3u8;
        assert_eq!(manual_calc_3, shr_calc_3);

        // Verify the relationship between octave shift and semitone addition
        for octaves in 1..=8 {
            let shr_result = base_note >> octaves;
            let add_result = base_note + (octaves * SEMITONES_IN_OCTAVE);
            assert_eq!(shr_result, add_result);
        }
    }

    #[test]
    fn test_semitone_shl_octave_shift() {
        // Test basic octave shifting with Shl operator (downward)

        // Test middle C (semitone 60) shifted down by octaves
        let middle_c = Semitone::new(60);
        let c3 = middle_c << 1u8; // 1 octave down
        assert_eq!(u8::from(c3), 48);

        let c2 = middle_c << 2u8; // 2 octaves down
        assert_eq!(u8::from(c2), 36);

        let c_minus_1 = middle_c << 5u8; // 5 octaves down (C-1)
        assert_eq!(u8::from(c_minus_1), 0);

        // Test C8 (semitone 96) shifted down
        let c8 = Semitone::new(96);
        let c7 = c8 << 1u8; // 1 octave down
        assert_eq!(u8::from(c7), 84);

        let c5 = c8 << 3u8; // 3 octaves down
        assert_eq!(u8::from(c5), 60);

        // Test with non-C notes to ensure pitch class is preserved
        let f_sharp4 = Semitone::new(66); // F#4
        let f_sharp3 = f_sharp4 << 1u8; // F#3
        assert_eq!(u8::from(f_sharp3), 54);

        let f_sharp1 = f_sharp4 << 3u8; // F#1
        assert_eq!(u8::from(f_sharp1), 30);

        // Test zero octave shift (no change)
        let g = Semitone::new(67);
        let g_same = g << 0u8;
        assert_eq!(u8::from(g_same), 67);
    }

    #[test]
    fn test_semitone_shl_pitch_class_preservation() {
        // Test that octave shifting preserves the pitch class (note within octave)

        // Generate all pitch classes (0-11) starting from a high octave to allow downward shifts
        let base_octave = 6u8;

        (0..12u8)
            .map(|pitch_class| {
                let original = Semitone::new(pitch_class + (base_octave * SEMITONES_IN_OCTAVE));
                (pitch_class, original)
            })
            .for_each(|(pitch_class, original)| {
                // Test downward shifts (1-5 octaves) that don't exceed the base octave
                (1..=5u8)
                    .filter(|&octaves| octaves <= base_octave)
                    .map(|octaves| (octaves, original << octaves))
                    .map(|(octaves, shifted)| {
                        (octaves, shifted, u8::from(shifted), u8::from(original))
                    })
                    .for_each(|(octaves, shifted, shifted_value, original_value)| {
                        // Verify the absolute value is correct (downward shift)
                        let expected_value = original_value - (octaves * SEMITONES_IN_OCTAVE);
                        assert_eq!(
                            shifted_value, expected_value,
                            "Pitch class {}: {} << {} octaves should be {}, got {}",
                            pitch_class, original_value, octaves, expected_value, shifted_value
                        );

                        // Verify pitch class is preserved using modulo
                        let actual_pitch_class = shifted_value % SEMITONES_IN_OCTAVE;
                        assert_eq!(
                            actual_pitch_class, pitch_class,
                            "Pitch class should be preserved: {} << {} octaves, expected pitch class {}, got {}",
                            original_value, octaves, pitch_class, actual_pitch_class
                        );

                        // Verify the shifted semitone can be converted back
                        assert_eq!(u8::from(shifted), shifted_value);
                    });
            });

        // Additional functional test: verify that bidirectional operations preserve pitch classes
        let bidirectional_tests: Vec<_> = (0..12u8)
            .flat_map(|pitch_class| {
                let mid_octave_note = Semitone::new(pitch_class + (3 * SEMITONES_IN_OCTAVE)); // Start from octave 3
                (1..=2u8).map(move |octaves| {
                    let down_then_up = (mid_octave_note << octaves) >> octaves;
                    let up_then_down = (mid_octave_note >> octaves) << octaves;
                    (
                        pitch_class,
                        mid_octave_note,
                        down_then_up,
                        up_then_down,
                        octaves,
                    )
                })
            })
            .collect();

        // Verify round-trip consistency
        bidirectional_tests.iter().for_each(
            |(pitch_class, original, down_then_up, up_then_down, octaves)| {
                assert_eq!(
                    down_then_up, original,
                    "Pitch class {}: down {} then up {} should return to original",
                    pitch_class, octaves, octaves
                );
                assert_eq!(
                    up_then_down, original,
                    "Pitch class {}: up {} then down {} should return to original",
                    pitch_class, octaves, octaves
                );
            },
        );
    }

    #[test]
    fn test_semitone_shl_saturation_behavior() {
        // Test saturation behavior at u8 boundaries (underflow protection)

        // Test with low semitone values that would underflow
        let low_semitone = Semitone::new(10);
        let shifted = low_semitone << 5u8; // 5 octaves = 60 semitones
                                           // 10 - 60 = -50, but u8 min is 0, so should saturate at 0
        assert_eq!(u8::from(shifted), 0);

        // Test with minimum u8 value
        let min_semitone = Semitone::new(0);
        let shifted_min = min_semitone << 1u8; // 1 octave = 12 semitones
                                               // 0 - 12 would underflow, should saturate at 0
        assert_eq!(u8::from(shifted_min), 0);

        // Test edge case: exactly at the boundary
        let boundary_test = Semitone::new(12); // Exactly 1 octave
        let boundary_shifted = boundary_test << 1u8;
        assert_eq!(u8::from(boundary_shifted), 0); // Should exactly hit 0

        // Test that smaller shifts work correctly
        let small_test = Semitone::new(5);
        let small_shift = small_test << 0u8; // No shift
        assert_eq!(u8::from(small_shift), 5);

        // Test partial underflow
        let partial_test = Semitone::new(6);
        let partial_shift = partial_test << 1u8; // 6 - 12 = -6, saturates to 0
        assert_eq!(u8::from(partial_shift), 0);
    }

    #[test]
    fn test_semitone_shl_large_octave_shifts() {
        // Test with large octave shift values

        let high_c = Semitone::new(108); // C8

        // Test large shift that would cause underflow
        let large_shift = high_c << 50u8; // 50 octaves = 600 semitones
                                          // 108 - 600 would underflow u8, should saturate at 0
        assert_eq!(u8::from(large_shift), 0);

        // Test maximum possible u8 octave value
        let max_octave_shift = high_c << 255u8;
        assert_eq!(u8::from(max_octave_shift), 0);

        // Test moderate values that work within range
        let moderate_note = Semitone::new(120);
        let moderate_shift = moderate_note << 5u8; // 5 octaves = 60 semitones
                                                   // 120 - 60 = 60, within u8 range
        assert_eq!(u8::from(moderate_shift), 60);
    }

    #[test]
    fn test_semitone_shl_musical_examples() {
        // Test realistic musical octave transposition examples (downward)

        // Piano range examples going down
        let c8 = Semitone::new(108); // C8 (highest C on piano)
        let c7 = c8 << 1u8; // C7
        assert_eq!(u8::from(c7), 96);

        let c4 = c8 << 4u8; // C4 (middle C, 4 octaves down)
        assert_eq!(u8::from(c4), 60);

        // Orchestra tuning: A4 transposed to lower octaves
        let a4_standard = Semitone::new(69); // A4 = 440 Hz
        let a3 = a4_standard << 1u8; // A3 = 220 Hz
        assert_eq!(u8::from(a3), 57);

        let a2 = a4_standard << 2u8; // A2 = 110 Hz
        assert_eq!(u8::from(a2), 45);

        let a1 = a4_standard << 3u8; // A1 = 55 Hz
        assert_eq!(u8::from(a1), 33);

        // Guitar string transposition (1st string E4 to lower octaves)
        let e4 = Semitone::new(64); // E4 (high E on guitar)
        let e3 = e4 << 1u8; // E3
        let e2 = e4 << 2u8; // E2 (6th string)
        let e1 = e4 << 3u8; // E1

        assert_eq!(u8::from(e3), 52);
        assert_eq!(u8::from(e2), 40);
        assert_eq!(u8::from(e1), 28);

        // Verify all Es have the same pitch class (4)
        assert_eq!(u8::from(e4) % 12, 4);
        assert_eq!(u8::from(e3) % 12, 4);
        assert_eq!(u8::from(e2) % 12, 4);
        assert_eq!(u8::from(e1) % 12, 4);

        // Vocal range examples
        let soprano_c6 = Semitone::new(84); // Soprano high C
        let alto_c5 = soprano_c6 << 1u8; // Alto range
        let tenor_c4 = soprano_c6 << 2u8; // Tenor range
        let bass_c3 = soprano_c6 << 3u8; // Bass range

        assert_eq!(u8::from(alto_c5), 72);
        assert_eq!(u8::from(tenor_c4), 60);
        assert_eq!(u8::from(bass_c3), 48);
    }

    #[test]
    fn test_semitone_shl_with_constant() {
        // Test using the SEMITONES_IN_OCTAVE constant to verify calculations

        let base_note = Semitone::new(85); // C#6 (high enough to shift down)

        // Manual calculation: 1 octave down
        let manual_calc = Semitone::new(85 - SEMITONES_IN_OCTAVE);
        let shl_calc = base_note << 1u8;
        assert_eq!(manual_calc, shl_calc);

        // Manual calculation: 3 octaves down
        let manual_calc_3 = Semitone::new(85 - (3 * SEMITONES_IN_OCTAVE));
        let shl_calc_3 = base_note << 3u8;
        assert_eq!(manual_calc_3, shl_calc_3);

        // Verify the relationship between octave shift and semitone subtraction
        for octaves in 1..=6 {
            let shl_result = base_note << octaves;
            let sub_result = base_note - (octaves * SEMITONES_IN_OCTAVE);
            assert_eq!(shl_result, sub_result);
        }
    }

    #[test]
    fn test_semitone_shr_shl_symmetry() {
        // Test that Shr and Shl operations are symmetric

        let middle_c = Semitone::new(60); // C4 (middle C)

        // Test round trip: up then down
        for octaves in 1..=3 {
            let up_then_down = (middle_c >> octaves) << octaves;
            assert_eq!(up_then_down, middle_c);
        }

        // Test round trip: down then up (starting from higher note)
        let high_c = Semitone::new(96); // C7
        for octaves in 1..=5 {
            let down_then_up = (high_c << octaves) >> octaves;
            assert_eq!(down_then_up, high_c);
        }

        // Test with various starting notes
        let test_notes = [24, 36, 48, 60, 72, 84, 96]; // C notes in different octaves

        for &note_value in &test_notes {
            let note = Semitone::new(note_value);

            // Test symmetry within safe ranges
            for octaves in 1..=3 {
                // Up then down (check for overflow before testing)
                let octave_semitones = octaves * SEMITONES_IN_OCTAVE;
                if note_value.saturating_add(octave_semitones) == note_value + octave_semitones {
                    let up_down = (note >> octaves) << octaves;
                    assert_eq!(up_down, note);
                }

                // Down then up
                if note_value >= octave_semitones {
                    let down_up = (note << octaves) >> octaves;
                    assert_eq!(down_up, note);
                }
            }
        }
    }

    #[test]
    fn test_semitone_shl_shr_boundary_cases() {
        // Test edge cases combining Shl and Shr operations

        // Test at lower boundary
        let low_note = Semitone::new(12); // C0
        let shifted_down = low_note << 1u8; // Should saturate to 0
        assert_eq!(u8::from(shifted_down), 0);

        let back_up = shifted_down >> 5u8; // Shift up from 0
        assert_eq!(u8::from(back_up), 60); // Should be C4

        // Test at upper boundary - start with a value that won't saturate immediately
        let high_note = Semitone::new(200);
        let shifted_up = high_note >> 2u8; // 200 + 24 = 224, won't saturate yet
        assert_eq!(u8::from(shifted_up), 224);

        // Now test a case that will saturate
        let very_high = Semitone::new(250);
        let saturated_up = very_high >> 1u8; // 250 + 12 = 262, saturates to 255
        assert_eq!(u8::from(saturated_up), 255);

        let back_down = saturated_up << 10u8; // Large shift down from 255: 255 - 120 = 135
        assert_eq!(u8::from(back_down), 135); // Should be 135, not 0

        // Test a case that actually saturates to 0
        let extreme_down = saturated_up << 25u8; // 25 octaves = 300 semitones, will saturate to 0
        assert_eq!(u8::from(extreme_down), 0); // Should saturate to 0

        // Test zero operations
        let any_note = Semitone::new(72);
        assert_eq!(any_note << 0u8, any_note);
        assert_eq!(any_note >> 0u8, any_note);

        // Test boundary exactly at octave intervals
        let boundary_c = Semitone::new(24); // C1
        let boundary_down = boundary_c << 2u8; // C1 - 2 octaves = C-1 (semitone 0)
        assert_eq!(u8::from(boundary_down), 0);

        let boundary_up = boundary_down >> 2u8; // C-1 + 2 octaves = C1 (semitone 24)
        assert_eq!(u8::from(boundary_up), 24);
        assert_eq!(boundary_up, boundary_c);
    }
}
