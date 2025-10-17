//! Note implementation for musical notes.

use crate::semitone::Semitone;

/// A musical note represented by its semitone offset from C.
///
/// Notes are fundamental building blocks in music theory, representing
/// specific pitches. This implementation uses semitones as the underlying
/// representation, where C = 0, C# = 1, D = 2, etc.
///
/// # Examples
///
/// ```
/// use musik_std::Note;
///
/// let c = Note::new(0);
/// let c_sharp = Note::new(1);
/// let d = Note::new(2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(u8);

impl Note {
    /// Creates a new `Note` from a semitone value.
    ///
    /// This is a const function that can be used in const contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// const C: Note = Note::new(0);
    /// const C_SHARP: Note = Note::new(1);
    /// const D: Note = Note::new(2);
    /// ```
    pub const fn new(semitone: u8) -> Self {
        Self(semitone)
    }

    /// Returns the semitone offset of this note.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// let c_sharp = Note::new(1);
    /// assert_eq!(c_sharp.semitone(), 1);
    /// ```
    pub fn semitone(self) -> u8 {
        self.0
    }

    /// Returns the semitone offset of this note as a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Note, Semitone};
    ///
    /// let d = Note::new(2);
    /// let semitone: Semitone = d.as_semitone();
    /// assert_eq!(u8::from(semitone), 2);
    /// ```
    pub const fn as_semitone(self) -> Semitone {
        Semitone::new(self.0)
    }
}

impl From<u8> for Note {
    /// Creates a `Note` from a `u8` semitone value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// let e = Note::from(4u8);
    /// assert_eq!(e.semitone(), 4);
    /// ```
    fn from(semitone: u8) -> Self {
        Self::new(semitone)
    }
}

impl From<Semitone> for Note {
    /// Creates a `Note` from a `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Note, Semitone};
    ///
    /// let semitone = Semitone::new(7);
    /// let g = Note::from(semitone);
    /// assert_eq!(g.semitone(), 7);
    /// ```
    fn from(semitone: Semitone) -> Self {
        Self(u8::from(semitone))
    }
}

impl From<Note> for u8 {
    /// Converts a `Note` into its semitone `u8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// let f = Note::new(5);
    /// let semitone: u8 = f.into();
    /// assert_eq!(semitone, 5);
    /// ```
    fn from(note: Note) -> Self {
        note.semitone()
    }
}

impl From<Note> for Semitone {
    /// Converts a `Note` into its `Semitone`.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Note, Semitone};
    ///
    /// let a = Note::new(9);
    /// let semitone: Semitone = a.into();
    /// assert_eq!(u8::from(semitone), 9);
    /// ```
    fn from(note: Note) -> Self {
        Semitone::new(note.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let c = Note::new(0);
        let c_sharp = Note::new(1);
        let d = Note::new(2);

        assert_eq!(c.semitone(), 0);
        assert_eq!(c_sharp.semitone(), 1);
        assert_eq!(d.semitone(), 2);
    }

    #[test]
    fn test_note_const_creation() {
        const C: Note = Note::new(0);
        const G: Note = Note::new(7);
        const OCTAVE: Note = Note::new(12);

        assert_eq!(C.semitone(), 0);
        assert_eq!(G.semitone(), 7);
        assert_eq!(OCTAVE.semitone(), 12);
    }

    #[test]
    fn test_note_from_u8() {
        let e = Note::from(4u8);
        let f = Note::from(5u8);

        assert_eq!(e.semitone(), 4);
        assert_eq!(f.semitone(), 5);
    }

    #[test]
    fn test_note_from_semitone() {
        let semitone = Semitone::new(9);
        let a = Note::from(semitone);

        assert_eq!(a.semitone(), 9);
        assert_eq!(a.as_semitone(), semitone);
    }

    #[test]
    fn test_note_into_u8() {
        let b = Note::new(11);
        let semitone_val: u8 = b.into();

        assert_eq!(semitone_val, 11);
    }

    #[test]
    fn test_note_into_semitone() {
        let f_sharp = Note::new(6);
        let semitone: Semitone = f_sharp.into();

        assert_eq!(u8::from(semitone), 6);
    }

    #[test]
    fn test_note_equality() {
        let c1 = Note::new(0);
        let c2 = Note::from(0u8);
        let d = Note::new(2);

        assert_eq!(c1, c2);
        assert_ne!(c1, d);
    }

    #[test]
    fn test_note_ordering() {
        let c = Note::new(0);
        let d = Note::new(2);
        let e = Note::new(4);

        assert!(c < d);
        assert!(d < e);
        assert!(c <= d);
        assert!(e >= d);
    }

    #[test]
    fn test_note_debug() {
        let note = Note::new(7);
        let debug_str = format!("{:?}", note);
        assert!(debug_str.contains("Note"));
    }

    #[test]
    fn test_note_musical_examples() {
        // Chromatic scale from C
        let c = Note::new(0);
        let c_sharp = Note::new(1);
        let d = Note::new(2);
        let d_sharp = Note::new(3);
        let e = Note::new(4);
        let f = Note::new(5);
        let f_sharp = Note::new(6);
        let g = Note::new(7);
        let g_sharp = Note::new(8);
        let a = Note::new(9);
        let a_sharp = Note::new(10);
        let b = Note::new(11);

        // Test that they're in ascending order
        assert!(c < c_sharp);
        assert!(c_sharp < d);
        assert!(d < d_sharp);
        assert!(d_sharp < e);
        assert!(e < f);
        assert!(f < f_sharp);
        assert!(f_sharp < g);
        assert!(g < g_sharp);
        assert!(g_sharp < a);
        assert!(a < a_sharp);
        assert!(a_sharp < b);

        // Test specific semitone values
        assert_eq!(c.semitone(), 0);
        assert_eq!(e.semitone(), 4); // Major third
        assert_eq!(g.semitone(), 7); // Perfect fifth
        assert_eq!(b.semitone(), 11); // Major seventh
    }
}
