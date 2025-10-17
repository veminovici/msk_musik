//! Note implementation for musical notes.

use crate::semitone::Semitone;
use std::ops::{Add, Shl, Shr, Sub};

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

impl Shr<u8> for Note {
    type Output = Note;

    /// Shifts a `Note` upward by the given number of octaves.
    ///
    /// This operation transposes the note upward by N octaves, where each octave
    /// is 12 semitones. The result uses saturating addition to prevent overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// let middle_c = Note::new(60);    // Middle C (MIDI 60)
    /// let high_c = middle_c >> 1u8;    // Up 1 octave → C7 (MIDI 72)
    /// assert_eq!(high_c.semitone(), 72);
    ///
    /// let c = Note::new(0);            // C0
    /// let c_two_octaves_up = c >> 2u8; // Up 2 octaves → C2 (MIDI 24)
    /// assert_eq!(c_two_octaves_up.semitone(), 24);
    /// ```
    fn shr(self, rhs: u8) -> Self::Output {
        let octave_shift = rhs.saturating_mul(12);
        Note::new(self.0.saturating_add(octave_shift))
    }
}

impl Shl<u8> for Note {
    type Output = Note;

    /// Shifts a `Note` downward by the given number of octaves.
    ///
    /// This operation transposes the note downward by N octaves, where each octave
    /// is 12 semitones. The result uses saturating subtraction to prevent underflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use musik_std::Note;
    ///
    /// let high_c = Note::new(72);      // C7 (MIDI 72)
    /// let middle_c = high_c << 1u8;    // Down 1 octave → C6 (MIDI 60)
    /// assert_eq!(middle_c.semitone(), 60);
    ///
    /// let c = Note::new(24);           // C2 (MIDI 24)
    /// let low_c = c << 2u8;            // Down 2 octaves → C0 (MIDI 0)
    /// assert_eq!(low_c.semitone(), 0);
    /// ```
    fn shl(self, rhs: u8) -> Self::Output {
        let octave_shift = rhs.saturating_mul(12);
        Note::new(self.0.saturating_sub(octave_shift))
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

    #[test]
    fn test_note_shr_octave_shift() {
        let middle_c = Note::new(60); // Middle C (MIDI 60)
        let high_c = middle_c >> 1u8; // Up 1 octave
        assert_eq!(high_c.semitone(), 72);

        let c0 = Note::new(0); // C0
        let c2 = c0 >> 2u8; // Up 2 octaves
        assert_eq!(c2.semitone(), 24);

        let note = Note::new(36); // C3
        let shifted = note >> 3u8; // Up 3 octaves
        assert_eq!(shifted.semitone(), 72); // C6
    }

    #[test]
    fn test_note_shl_octave_shift() {
        let high_c = Note::new(72); // C6 (MIDI 72)
        let middle_c = high_c << 1u8; // Down 1 octave
        assert_eq!(middle_c.semitone(), 60);

        let c2 = Note::new(24); // C2
        let c0 = c2 << 2u8; // Down 2 octaves
        assert_eq!(c0.semitone(), 0);

        let note = Note::new(60); // Middle C
        let shifted = note << 3u8; // Down 3 octaves (should go to 60 - 36 = 24)
        assert_eq!(shifted.semitone(), 24); // C2
    }

    #[test]
    fn test_note_shr_shl_saturation() {
        // Test upward saturation (Shr)
        let high_note = Note::new(250);
        let shifted_up = high_note >> 1u8;
        assert_eq!(shifted_up.semitone(), 255); // Should saturate at u8::MAX

        let very_high = Note::new(240);
        let max_shift = very_high >> 10u8; // Large shift
        assert_eq!(max_shift.semitone(), 255); // Should saturate

        // Test downward saturation (Shl)
        let low_note = Note::new(5);
        let shifted_down = low_note << 1u8; // 5 - 12 should saturate at 0
        assert_eq!(shifted_down.semitone(), 0);

        let c1 = Note::new(12);
        let over_shift = c1 << 5u8; // 12 - 60 should saturate at 0
        assert_eq!(over_shift.semitone(), 0);
    }

    #[test]
    fn test_note_shr_shl_musical_examples() {
        // Vocal range examples
        let soprano_c = Note::new(72); // C6 - high soprano note
        let alto_c = soprano_c << 1u8; // Down to C5
        let tenor_c = alto_c << 1u8; // Down to C4
        let bass_c = tenor_c << 1u8; // Down to C3

        assert_eq!(alto_c.semitone(), 60); // C5
        assert_eq!(tenor_c.semitone(), 48); // C4
        assert_eq!(bass_c.semitone(), 36); // C3

        // Piano range examples
        let middle_c = Note::new(60); // Middle C
        let low_c = middle_c << 2u8; // Down 2 octaves
        let high_c = middle_c >> 2u8; // Up 2 octaves

        assert_eq!(low_c.semitone(), 36); // C3
        assert_eq!(high_c.semitone(), 84); // C7

        // Chord inversions across octaves
        let c_major_root = Note::new(60); // C4
        let c_major_first_inv = c_major_root >> 1u8; // C5 (bass note moved up)
        assert_eq!(c_major_first_inv.semitone(), 72);
    }

    #[test]
    fn test_note_shr_shl_symmetry() {
        let test_notes = [12, 24, 36, 48, 60, 72, 84, 96];

        for &note_value in &test_notes {
            let note = Note::new(note_value);

            // Test 1-octave symmetry (within safe ranges)
            // For upward then downward: note should be >= 12 and <= 243 to stay in bounds
            if (12..=243).contains(&note_value) {
                let up_then_down = (note >> 1u8) << 1u8;
                assert_eq!(up_then_down.semitone(), note_value);
            }

            // For downward then upward: note should be >= 12 and result should not overflow
            if (12..=243).contains(&note_value) {
                let down_then_up = (note << 1u8) >> 1u8;
                assert_eq!(down_then_up.semitone(), note_value);
            }

            // Test 2-octave symmetry (within safe ranges)
            if (24..=231).contains(&note_value) {
                let up_then_down_2 = (note >> 2u8) << 2u8;
                assert_eq!(up_then_down_2.semitone(), note_value);

                let down_then_up_2 = (note << 2u8) >> 2u8;
                assert_eq!(down_then_up_2.semitone(), note_value);
            }
        }
    }

    #[test]
    fn test_note_shr_shl_zero_operations() {
        let test_notes = [0, 12, 36, 60, 84, 127];

        for &note_value in &test_notes {
            let note = Note::new(note_value);

            // Shifting by 0 octaves should return the same note
            assert_eq!((note >> 0u8).semitone(), note_value);
            assert_eq!((note << 0u8).semitone(), note_value);
        }
    }

    #[test]
    fn test_note_shr_shl_chromatic_operations() {
        // Test that pitch class is preserved across octave shifts
        let chromatic_notes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        for &pitch_class in &chromatic_notes {
            let note = Note::new(pitch_class + 48); // Start at octave 4

            // Shift up then down should preserve pitch class (within safe ranges)
            let shifted = (note >> 1u8) << 1u8;
            assert_eq!(shifted.semitone() % 12, pitch_class);

            // Shift down then up should preserve pitch class
            let note_high = Note::new(pitch_class + 60); // Start higher for safe downward shift
            let shifted_2 = (note_high << 1u8) >> 1u8;
            assert_eq!(shifted_2.semitone() % 12, pitch_class);
        }
    }

    #[test]
    fn test_note_shr_shl_boundary_cases() {
        // Test at the boundaries of u8 range
        let min_note = Note::new(0);
        let max_note = Note::new(255);

        // Shifting min note down should stay at 0
        assert_eq!((min_note << 1u8).semitone(), 0);
        assert_eq!((min_note << 10u8).semitone(), 0);

        // Shifting max note up should stay at 255
        assert_eq!((max_note >> 1u8).semitone(), 255);
        assert_eq!((max_note >> 10u8).semitone(), 255);

        // Test near boundaries
        let near_min = Note::new(11);
        assert_eq!((near_min << 1u8).semitone(), 0); // 11 - 12 = saturates to 0

        let near_max = Note::new(244);
        assert_eq!((near_max >> 1u8).semitone(), 255); // 244 + 12 = saturates to 255
    }
}
