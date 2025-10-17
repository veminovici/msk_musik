//! Semitone implementation for musical intervals.

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

impl std::ops::Add<u8> for Semitone {
    type Output = Semitone;

    /// Adds a `u8` value to a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c = Semitone::new(0);  // C
    /// let c_sharp = c + 1u8;     // C#
    /// assert_eq!(u8::from(c_sharp), 1);
    /// ```
    fn add(self, rhs: u8) -> Self::Output {
        Semitone::new(self.0.wrapping_add(rhs))
    }
}

impl std::ops::Sub<u8> for Semitone {
    type Output = Semitone;

    /// Subtracts a `u8` value from a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Semitone;
    ///
    /// let c_sharp = Semitone::new(1);  // C#
    /// let c = c_sharp - 1u8;           // C
    /// assert_eq!(u8::from(c), 0);
    /// ```
    fn sub(self, rhs: u8) -> Self::Output {
        Semitone::new(self.0.wrapping_sub(rhs))
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
    fn test_semitone_add_u8() {
        // Test basic addition
        let c = Semitone::new(0);
        let c_sharp = c + 1u8;
        assert_eq!(u8::from(c_sharp), 1);

        // Test addition with larger values
        let f = Semitone::new(5);
        let f_sharp = f + 1u8;
        assert_eq!(u8::from(f_sharp), 6);

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
    fn test_semitone_sub_u8() {
        // Test basic subtraction
        let c_sharp = Semitone::new(1);
        let c = c_sharp - 1u8;
        assert_eq!(u8::from(c), 0);

        // Test subtraction with larger values
        let f_sharp = Semitone::new(6);
        let f = f_sharp - 1u8;
        assert_eq!(u8::from(f), 5);

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
        // Musical interval examples
        let c = Semitone::new(0);

        // Major scale intervals from C
        let d = c + 2u8; // Whole step
        let e = d + 2u8; // Whole step
        let f = e + 1u8; // Half step
        let g = f + 2u8; // Whole step
        let a = g + 2u8; // Whole step
        let b = a + 2u8; // Whole step
        let c_octave = b + 1u8; // Half step

        assert_eq!(u8::from(d), 2);
        assert_eq!(u8::from(e), 4);
        assert_eq!(u8::from(f), 5);
        assert_eq!(u8::from(g), 7);
        assert_eq!(u8::from(a), 9);
        assert_eq!(u8::from(b), 11);
        assert_eq!(u8::from(c_octave), 12);

        // Going back down
        let b_from_octave = c_octave - 1u8;
        assert_eq!(u8::from(b_from_octave), 11);
        assert_eq!(b_from_octave, b);
    }
}
