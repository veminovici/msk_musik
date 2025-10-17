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
        Self(value)
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
}
