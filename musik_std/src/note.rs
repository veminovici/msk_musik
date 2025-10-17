//! Note implementation for musical notes.

use crate::semitone::Semitone;
use std::ops::{Add, Sub};

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

impl Add<Semitone> for Note {
    type Output = Note;

    /// Adds a `Semitone` to a `Note`, returning a new `Note`.
    ///
    /// This operation transposes the note upward by the given number of semitones.
    /// The result wraps around the 12-semitone chromatic scale if it exceeds the
    /// maximum semitone value.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Note, Semitone};
    ///
    /// let c = Note::new(0);        // C
    /// let major_third = Semitone::new(4);
    /// let e = c + major_third;     // E
    /// assert_eq!(e.semitone(), 4);
    ///
    /// let g = Note::new(7);        // G
    /// let perfect_fifth = Semitone::new(7);
    /// let d = g + perfect_fifth;   // D (next octave: 7 + 7 = 14, wraps to 2)
    /// assert_eq!(d.semitone(), 14);
    /// ```
    fn add(self, rhs: Semitone) -> Self::Output {
        Note::new(self.0.saturating_add(u8::from(rhs)))
    }
}

impl Sub<Semitone> for Note {
    type Output = Note;

    /// Subtracts a `Semitone` from a `Note`, returning a new `Note`.
    ///
    /// This operation transposes the note downward by the given number of semitones.
    /// The result uses saturating subtraction to prevent underflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::{Note, Semitone};
    ///
    /// let e = Note::new(4);        // E
    /// let major_third = Semitone::new(4);
    /// let c = e - major_third;     // C
    /// assert_eq!(c.semitone(), 0);
    ///
    /// let c = Note::new(0);        // C
    /// let octave = Semitone::new(12);
    /// let low_c = c - octave;      // Saturates at 0
    /// assert_eq!(low_c.semitone(), 0);
    /// ```
    fn sub(self, rhs: Semitone) -> Self::Output {
        Note::new(self.0.saturating_sub(u8::from(rhs)))
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

    #[test]
    fn test_note_add_semitone() {
        let c = Note::new(0);
        let major_third = Semitone::new(4);
        let e = c + major_third;
        assert_eq!(e.semitone(), 4);

        let g = Note::new(7);
        let perfect_fifth = Semitone::new(7);
        let d_next_octave = g + perfect_fifth;
        assert_eq!(d_next_octave.semitone(), 14);

        // Test with large values (saturation)
        let high_note = Note::new(250);
        let small_interval = Semitone::new(10);
        let result = high_note + small_interval;
        assert_eq!(result.semitone(), 255); // Should saturate at u8::MAX
    }

    #[test]
    fn test_note_sub_semitone() {
        let e = Note::new(4);
        let major_third = Semitone::new(4);
        let c = e - major_third;
        assert_eq!(c.semitone(), 0);

        let g = Note::new(7);
        let minor_third = Semitone::new(3);
        let e_flat = g - minor_third;
        assert_eq!(e_flat.semitone(), 4);

        // Test saturation at zero
        let c = Note::new(0);
        let octave = Semitone::new(12);
        let result = c - octave;
        assert_eq!(result.semitone(), 0); // Should saturate at 0
    }

    #[test]
    fn test_note_add_sub_musical_intervals() {
        let c = Note::new(0);

        // Major scale intervals from C
        let major_second = Semitone::new(2);
        let major_third = Semitone::new(4);
        let perfect_fourth = Semitone::new(5);
        let perfect_fifth = Semitone::new(7);
        let major_sixth = Semitone::new(9);
        let major_seventh = Semitone::new(11);
        let octave = Semitone::new(12);

        assert_eq!((c + major_second).semitone(), 2); // D
        assert_eq!((c + major_third).semitone(), 4); // E
        assert_eq!((c + perfect_fourth).semitone(), 5); // F
        assert_eq!((c + perfect_fifth).semitone(), 7); // G
        assert_eq!((c + major_sixth).semitone(), 9); // A
        assert_eq!((c + major_seventh).semitone(), 11); // B
        assert_eq!((c + octave).semitone(), 12); // C next octave

        // Test subtraction brings us back
        let high_c = c + octave;
        assert_eq!((high_c - octave).semitone(), 0); // Back to C
        assert_eq!((high_c - major_seventh).semitone(), 1); // C# when subtracting major 7th from high C
    }

    #[test]
    fn test_note_add_sub_chromatic_operations() {
        let notes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let semitone_step = Semitone::new(1);

        // Test adding a semitone to each note
        for &note_value in &notes {
            let note = Note::new(note_value);
            let next_note = note + semitone_step;
            assert_eq!(next_note.semitone(), note_value + 1);
        }

        // Test subtracting a semitone from each note (except 0)
        for &note_value in &notes[1..] {
            let note = Note::new(note_value);
            let prev_note = note - semitone_step;
            assert_eq!(prev_note.semitone(), note_value - 1);
        }
    }

    #[test]
    fn test_note_add_sub_identity_operations() {
        let notes = [0, 5, 12, 24, 48, 60, 72, 127];
        let zero_semitones = Semitone::new(0);

        for &note_value in &notes {
            let note = Note::new(note_value);

            // Adding zero should return the same note
            assert_eq!((note + zero_semitones).semitone(), note_value);

            // Subtracting zero should return the same note
            assert_eq!((note - zero_semitones).semitone(), note_value);
        }
    }
}
