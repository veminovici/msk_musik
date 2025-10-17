//! Pitch class implementation for the 12 chromatic pitch classes.

use std::fmt;

/// A pitch class represents one of the 12 chromatic pitch classes in Western music.
///
/// Pitch classes are equivalence classes of pitches under octave equivalence.
/// They represent the 12 distinct pitch positions within an octave:
/// C, C#, D, D#, E, F, F#, G, G#, A, A#, B.
///
/// The internal representation uses values 0-11 where:
/// - 0 = C
/// - 1 = C#/Db
/// - 2 = D
/// - 3 = D#/Eb
/// - 4 = E
/// - 5 = F
/// - 6 = F#/Gb
/// - 7 = G
/// - 8 = G#/Ab
/// - 9 = A
/// - 10 = A#/Bb
/// - 11 = B
///
/// # Examples
///
/// ```
/// use musik_std::PitchClass;
///
/// let c = PitchClass::new(0);
/// let c_sharp = PitchClass::new(1);
/// let d = PitchClass::new(2);
///
/// assert_eq!(format!("{}", c), "C");
/// assert_eq!(format!("{}", c_sharp), "C#");
/// assert_eq!(format!("{}", d), "D");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PitchClass(u8);

/// Common pitch class constants for convenience.
///
/// These constants provide a convenient way to reference the 12 chromatic pitch classes
/// without needing to remember their numeric values. Both sharp and flat enharmonic
/// equivalents are provided where applicable.
///
/// # Examples
///
/// ```
/// use musik_std::{C, C_SHARP, D_FLAT, G, PitchClass};
///
/// assert_eq!(C.value(), 0);
/// assert_eq!(C_SHARP, D_FLAT); // Enharmonic equivalents
/// assert_eq!(G.value(), 7);
/// ```
pub const C: PitchClass = PitchClass::new(0);
pub const C_SHARP: PitchClass = PitchClass::new(1);
pub const D_FLAT: PitchClass = PitchClass::new(1);
pub const D: PitchClass = PitchClass::new(2);
pub const D_SHARP: PitchClass = PitchClass::new(3);
pub const E_FLAT: PitchClass = PitchClass::new(3);
pub const E: PitchClass = PitchClass::new(4);
pub const F: PitchClass = PitchClass::new(5);
pub const F_SHARP: PitchClass = PitchClass::new(6);
pub const G_FLAT: PitchClass = PitchClass::new(6);
pub const G: PitchClass = PitchClass::new(7);
pub const G_SHARP: PitchClass = PitchClass::new(8);
pub const A_FLAT: PitchClass = PitchClass::new(8);
pub const A: PitchClass = PitchClass::new(9);
pub const A_SHARP: PitchClass = PitchClass::new(10);
pub const B_FLAT: PitchClass = PitchClass::new(10);
pub const B: PitchClass = PitchClass::new(11);

impl PitchClass {
    /// The number of pitch classes in the chromatic scale.
    pub const COUNT: u8 = 12;

    /// Creates a new `PitchClass` from a u8 value.
    ///
    /// The value is taken modulo 12 to ensure it's within the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::PitchClass;
    ///
    /// let c = PitchClass::new(0);
    /// let c_again = PitchClass::new(12); // Wraps around to C
    /// let b = PitchClass::new(11);
    ///
    /// assert_eq!(c.value(), 0);
    /// assert_eq!(c_again.value(), 0);
    /// assert_eq!(b.value(), 11);
    /// ```
    pub const fn new(value: u8) -> Self {
        Self(value % Self::COUNT)
    }

    /// Returns the raw value of this pitch class (0-11).
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::PitchClass;
    ///
    /// let f_sharp = PitchClass::new(6);
    /// assert_eq!(f_sharp.value(), 6);
    /// ```
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns the note name of this pitch class using sharp notation.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::PitchClass;
    ///
    /// let c = PitchClass::new(0);
    /// let c_sharp = PitchClass::new(1);
    /// let d = PitchClass::new(2);
    ///
    /// assert_eq!(c.name(), "C");
    /// assert_eq!(c_sharp.name(), "C#");
    /// assert_eq!(d.name(), "D");
    /// ```
    pub const fn name(self) -> &'static str {
        match self.0 {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            // This case should never occur due to the modulo operation in new()
            _ => "C", // Safe fallback, though this should never be reached
        }
    }

    /// Returns an iterator over all 12 pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{PitchClass, C, B};
    ///
    /// let all_pitch_classes: Vec<PitchClass> = PitchClass::all().collect();
    /// assert_eq!(all_pitch_classes.len(), 12);
    /// assert_eq!(all_pitch_classes[0], C);
    /// assert_eq!(all_pitch_classes[11], B);
    /// ```
    pub fn all() -> impl Iterator<Item = PitchClass> {
        (0..Self::COUNT).map(PitchClass::new)
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<u8> for PitchClass {
    /// Converts a u8 to a PitchClass, wrapping around the 12 pitch classes.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{PitchClass, C, B};
    ///
    /// let c: PitchClass = 0u8.into();
    /// let also_c: PitchClass = 12u8.into(); // Wraps around
    /// let b: PitchClass = 23u8.into(); // 23 % 12 = 11 = B
    ///
    /// assert_eq!(c, C);
    /// assert_eq!(also_c, C);
    /// assert_eq!(b, B);
    /// ```
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<PitchClass> for u8 {
    /// Converts a PitchClass to its underlying u8 value (0-11).
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{C_SHARP};
    ///
    /// let c_sharp = C_SHARP;
    /// let value: u8 = c_sharp.into();
    /// assert_eq!(value, 1);
    /// ```
    fn from(pitch_class: PitchClass) -> Self {
        pitch_class.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wrapping() {
        assert_eq!(PitchClass::new(0).value(), 0);
        assert_eq!(PitchClass::new(11).value(), 11);
        assert_eq!(PitchClass::new(12).value(), 0); // Wraps around
        assert_eq!(PitchClass::new(13).value(), 1); // Wraps around
        assert_eq!(PitchClass::new(23).value(), 11); // 23 % 12 = 11
    }

    #[test]
    fn test_constants() {
        assert_eq!(C.value(), 0);
        assert_eq!(C_SHARP.value(), 1);
        assert_eq!(D_FLAT.value(), 1); // Enharmonic with C#
        assert_eq!(D.value(), 2);
        assert_eq!(B.value(), 11);
    }

    #[test]
    fn test_names() {
        assert_eq!(C.name(), "C");
        assert_eq!(C_SHARP.name(), "C#");
        assert_eq!(D.name(), "D");
        assert_eq!(D_SHARP.name(), "D#");
        assert_eq!(E.name(), "E");
        assert_eq!(F.name(), "F");
        assert_eq!(F_SHARP.name(), "F#");
        assert_eq!(G.name(), "G");
        assert_eq!(G_SHARP.name(), "G#");
        assert_eq!(A.name(), "A");
        assert_eq!(A_SHARP.name(), "A#");
        assert_eq!(B.name(), "B");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", C), "C");
        assert_eq!(format!("{}", C_SHARP), "C#");
        assert_eq!(format!("{}", G), "G");
        assert_eq!(format!("{}", B), "B");
    }

    #[test]
    fn test_from_u8() {
        let c: PitchClass = 0u8.into();
        let c_sharp: PitchClass = 1u8.into();
        let wrapping: PitchClass = 12u8.into();

        assert_eq!(c, C);
        assert_eq!(c_sharp, C_SHARP);
        assert_eq!(wrapping, C);
    }

    #[test]
    fn test_into_u8() {
        let value: u8 = C_SHARP.into();
        assert_eq!(value, 1);

        let value: u8 = B.into();
        assert_eq!(value, 11);
    }


    #[test]
    fn test_all_iterator() {
        let all_pitch_classes: Vec<PitchClass> = PitchClass::all().collect();
        assert_eq!(all_pitch_classes.len(), 12);

        for (i, pc) in all_pitch_classes.iter().enumerate() {
            assert_eq!(pc.value(), i as u8);
        }

        assert_eq!(all_pitch_classes[0], C);
        assert_eq!(all_pitch_classes[11], B);
    }

    #[test]
    fn test_ordering() {
        assert!(C < C_SHARP);
        assert!(C_SHARP < D);
        assert!(A_SHARP < B);

        let mut pitch_classes = vec![G, C, F];
        pitch_classes.sort();
        assert_eq!(pitch_classes, vec![C, F, G]);
    }

    #[test]
    fn test_equality() {
        assert_eq!(C_SHARP, D_FLAT); // Same internal value
        assert_eq!(F_SHARP, G_FLAT); // Same internal value
        assert_eq!(A_SHARP, B_FLAT); // Same internal value
    }
}
