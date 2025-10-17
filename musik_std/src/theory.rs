//! Music theory primitives and concepts

use std::fmt;

/// Musical notes in chromatic scale
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Note {
    /// Get the semitone value (0-11) of the note
    pub fn semitone(&self) -> u8 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11,
        }
    }

    /// Create a note from a semitone value (0-11)
    pub fn from_semitone(semitone: u8) -> Option<Self> {
        match semitone % 12 {
            0 => Some(Note::C),
            1 => Some(Note::CSharp),
            2 => Some(Note::D),
            3 => Some(Note::DSharp),
            4 => Some(Note::E),
            5 => Some(Note::F),
            6 => Some(Note::FSharp),
            7 => Some(Note::G),
            8 => Some(Note::GSharp),
            9 => Some(Note::A),
            10 => Some(Note::ASharp),
            11 => Some(Note::B),
            _ => None,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
        };
        write!(f, "{}", name)
    }
}

/// Musical intervals in semitones
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Unison = 0,
    MinorSecond = 1,
    MajorSecond = 2,
    MinorThird = 3,
    MajorThird = 4,
    PerfectFourth = 5,
    Tritone = 6,
    PerfectFifth = 7,
    MinorSixth = 8,
    MajorSixth = 9,
    MinorSeventh = 10,
    MajorSeventh = 11,
    Octave = 12,
}

impl Interval {
    /// Get the semitone value of the interval
    pub fn semitones(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_semitone() {
        assert_eq!(Note::C.semitone(), 0);
        assert_eq!(Note::CSharp.semitone(), 1);
        assert_eq!(Note::B.semitone(), 11);
    }

    #[test]
    fn test_note_from_semitone() {
        assert_eq!(Note::from_semitone(0), Some(Note::C));
        assert_eq!(Note::from_semitone(1), Some(Note::CSharp));
        assert_eq!(Note::from_semitone(12), Some(Note::C)); // Wraps around
    }

    #[test]
    fn test_note_display() {
        assert_eq!(format!("{}", Note::C), "C");
        assert_eq!(format!("{}", Note::CSharp), "C#");
        assert_eq!(format!("{}", Note::FSharp), "F#");
    }

    #[test]
    fn test_interval_semitones() {
        assert_eq!(Interval::Unison.semitones(), 0);
        assert_eq!(Interval::PerfectFifth.semitones(), 7);
        assert_eq!(Interval::Octave.semitones(), 12);
    }
}