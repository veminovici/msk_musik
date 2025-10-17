//! Octave implementation for musical octave representation.

/// An octave represents a musical octave position.
///
/// In music theory, an octave is the interval between one musical pitch and another
/// with double its frequency. Octaves are numbered, where middle C is in octave 4.
/// This allows for negative octaves (sub-bass ranges) and positive octaves.
///
/// # Examples
///
/// ```
/// use musik_std::Octave;
///
/// let middle_octave = Octave::new(4);  // Middle C octave
/// let sub_bass = Octave::new(-1);      // Sub-bass octave
/// let high_octave = Octave::new(8);    // High octave
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Octave(i8);

impl Octave {
    /// Creates a new `Octave` from an `i8` value.
    ///
    /// This is a const function that can be used in const contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Octave;
    ///
    /// const MIDDLE_OCTAVE: Octave = Octave::new(4);
    /// const SUB_BASS: Octave = Octave::new(-1);
    /// const HIGH_OCTAVE: Octave = Octave::new(8);
    /// ```
    pub const fn new(value: i8) -> Self {
        Self(value)
    }

    /// Returns the octave number as an `i8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Octave;
    ///
    /// let octave = Octave::new(4);
    /// assert_eq!(octave.value(), 4);
    ///
    /// let sub_bass = Octave::new(-1);
    /// assert_eq!(sub_bass.value(), -1);
    /// ```
    pub fn value(self) -> i8 {
        self.0
    }
}

impl From<i8> for Octave {
    /// Creates an `Octave` from an `i8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Octave;
    ///
    /// let octave = Octave::from(4i8);
    /// assert_eq!(octave.value(), 4);
    /// ```
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<Octave> for i8 {
    /// Converts an `Octave` to an `i8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Octave;
    ///
    /// let octave = Octave::new(4);
    /// let value: i8 = octave.into();
    /// assert_eq!(value, 4);
    /// ```
    fn from(octave: Octave) -> Self {
        octave.0
    }
}

impl std::fmt::Display for Octave {
    /// Formats the octave for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Octave;
    ///
    /// let octave = Octave::new(4);
    /// assert_eq!(format!("{}", octave), "4");
    ///
    /// let sub_bass = Octave::new(-1);
    /// assert_eq!(format!("{}", sub_bass), "-1");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octave_creation() {
        let octave = Octave::new(4);
        assert_eq!(octave.value(), 4);

        let sub_bass = Octave::new(-1);
        assert_eq!(sub_bass.value(), -1);

        let high_octave = Octave::new(8);
        assert_eq!(high_octave.value(), 8);
    }

    #[test]
    fn test_octave_const_creation() {
        const MIDDLE_OCTAVE: Octave = Octave::new(4);
        const SUB_BASS: Octave = Octave::new(-1);
        const HIGH_OCTAVE: Octave = Octave::new(8);

        assert_eq!(MIDDLE_OCTAVE.value(), 4);
        assert_eq!(SUB_BASS.value(), -1);
        assert_eq!(HIGH_OCTAVE.value(), 8);
    }

    #[test]
    fn test_octave_from_i8() {
        let octave = Octave::from(4i8);
        assert_eq!(octave.value(), 4);

        let negative_octave = Octave::from(-2i8);
        assert_eq!(negative_octave.value(), -2);

        let zero_octave = Octave::from(0i8);
        assert_eq!(zero_octave.value(), 0);
    }

    #[test]
    fn test_octave_into_i8() {
        let octave = Octave::new(4);
        let value: i8 = octave.into();
        assert_eq!(value, 4);

        let negative_octave = Octave::new(-3);
        let negative_value: i8 = negative_octave.into();
        assert_eq!(negative_value, -3);
    }

    #[test]
    fn test_octave_display() {
        let octave = Octave::new(4);
        assert_eq!(format!("{}", octave), "4");

        let sub_bass = Octave::new(-1);
        assert_eq!(format!("{}", sub_bass), "-1");

        let zero_octave = Octave::new(0);
        assert_eq!(format!("{}", zero_octave), "0");
    }

    #[test]
    fn test_octave_equality() {
        let octave1 = Octave::new(4);
        let octave2 = Octave::new(4);
        let octave3 = Octave::new(5);

        assert_eq!(octave1, octave2);
        assert_ne!(octave1, octave3);
    }

    #[test]
    fn test_octave_ordering() {
        let low_octave = Octave::new(-1);
        let middle_octave = Octave::new(4);
        let high_octave = Octave::new(8);

        assert!(low_octave < middle_octave);
        assert!(middle_octave < high_octave);
        assert!(low_octave < high_octave);

        // Test ordering with arrays
        let mut octaves = [high_octave, low_octave, middle_octave];
        octaves.sort();
        assert_eq!(octaves, [low_octave, middle_octave, high_octave]);
    }

    #[test]
    fn test_octave_debug() {
        let octave = Octave::new(4);
        let debug_str = format!("{:?}", octave);
        assert!(debug_str.contains("Octave"));
        assert!(debug_str.contains("4"));
    }

    #[test]
    fn test_octave_musical_ranges() {
        // Test common musical octave ranges
        let sub_contra = Octave::new(-1); // Sub-contra octave
        let contra = Octave::new(0); // Contra octave
        let great = Octave::new(1); // Great octave
        let small = Octave::new(2); // Small octave
        let one_line = Octave::new(3); // One-line octave
        let two_line = Octave::new(4); // Two-line octave (middle C)
        let three_line = Octave::new(5); // Three-line octave
        let four_line = Octave::new(6); // Four-line octave
        let five_line = Octave::new(7); // Five-line octave
        let six_line = Octave::new(8); // Six-line octave

        // Test they're all different and properly ordered
        let octaves = [
            sub_contra, contra, great, small, one_line, two_line, three_line, four_line, five_line,
            six_line,
        ];

        // Verify they're in ascending order
        for i in 1..octaves.len() {
            assert!(octaves[i - 1] < octaves[i]);
        }

        // Test specific values
        assert_eq!(sub_contra.value(), -1);
        assert_eq!(two_line.value(), 4); // Middle C octave
        assert_eq!(six_line.value(), 8);
    }
}
