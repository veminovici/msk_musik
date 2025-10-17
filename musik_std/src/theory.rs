//! Music theory primitives and concepts

use std::fmt;

/// Core functionality for chromatic notes
pub trait ChromaticNote {
    /// Get the semitone value (0-11) within an octave
    fn semitone(&self) -> u8;

    /// Create from a semitone value
    fn from_semitone(semitone: u8) -> Option<Self>
    where
        Self: Sized;

    /// Get the note name as a string
    fn name(&self) -> &'static str;

    /// Check if this is a sharp/flat note (accidental)
    fn is_accidental(&self) -> bool;
}

/// Musical operations for transposing notes
pub trait TransposableNote: ChromaticNote {
    /// Transpose by a number of semitones
    fn transpose(self, semitones: i8) -> Self;

    /// Get the interval to another note in semitones
    fn interval_to(&self, other: &Self) -> u8;

    /// Apply an interval to get a new note
    fn add_interval(&self, interval: Interval) -> Self;
}

/// Enharmonic equivalent handling
pub trait EnharmonicNote: ChromaticNote {
    /// Get all enharmonic equivalents (e.g., C# and Db)
    fn enharmonic_equivalents(&self) -> Vec<String>;

    /// Get the flat equivalent name if applicable
    fn flat_name(&self) -> Option<&'static str>;

    /// Get the sharp equivalent name if applicable  
    fn sharp_name(&self) -> Option<&'static str>;

    /// Get the most common spelling in a given key context
    fn spelling_in_key(&self, key_signature_sharps: i8) -> String;
}

/// Scale degree relationships
pub trait ScaleDegree: ChromaticNote {
    /// Get the scale degree in a major scale (1-7, or None if not in scale)
    fn degree_in_major(&self, tonic: &Self) -> Option<u8>;

    /// Get the scale degree in a natural minor scale
    fn degree_in_minor(&self, tonic: &Self) -> Option<u8>;

    /// Check if this note is in a major scale
    fn is_in_major_scale(&self, tonic: &Self) -> bool;

    /// Check if this note is in a natural minor scale
    fn is_in_minor_scale(&self, tonic: &Self) -> bool;
}

/// Frequency and MIDI relationships
pub trait FrequencyNote: ChromaticNote {
    /// Get frequency for this note in a specific octave (A4 = 440Hz)
    fn frequency(&self, octave: i8) -> f64;

    /// Get the MIDI note number for a specific octave
    fn midi_number(&self, octave: i8) -> Option<u8>;

    /// Create from a frequency (finds closest note and returns cents offset)
    fn from_frequency(frequency: f64) -> (Self, i8, f64)
    where
        Self: Sized;
}

/// Circle of fifths relationships
pub trait CircleOfFifths: ChromaticNote {
    /// Get the next note in the circle of fifths
    fn next_fifth(&self) -> Self;

    /// Get the previous note in the circle of fifths  
    fn prev_fifth(&self) -> Self;

    /// Get position in circle of fifths (0-11, C=0)
    fn circle_position(&self) -> u8;

    /// Get the relative minor/major
    fn relative_key(&self) -> Self;
}

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

impl ChromaticNote for Note {
    fn semitone(&self) -> u8 {
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

    fn from_semitone(semitone: u8) -> Option<Self> {
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

    fn name(&self) -> &'static str {
        match self {
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
        }
    }

    fn is_accidental(&self) -> bool {
        matches!(
            self,
            Note::CSharp | Note::DSharp | Note::FSharp | Note::GSharp | Note::ASharp
        )
    }
}

impl TransposableNote for Note {
    fn transpose(self, semitones: i8) -> Self {
        let current = self.semitone() as i8;
        let new_semitone = (current + semitones).rem_euclid(12) as u8;
        Self::from_semitone(new_semitone).unwrap()
    }

    fn interval_to(&self, other: &Self) -> u8 {
        (other.semitone() + 12 - self.semitone()) % 12
    }

    fn add_interval(&self, interval: Interval) -> Self {
        self.transpose(interval.semitones() as i8)
    }
}

impl EnharmonicNote for Note {
    fn enharmonic_equivalents(&self) -> Vec<String> {
        match self {
            Note::CSharp => vec!["C#".to_string(), "Db".to_string()],
            Note::DSharp => vec!["D#".to_string(), "Eb".to_string()],
            Note::FSharp => vec!["F#".to_string(), "Gb".to_string()],
            Note::GSharp => vec!["G#".to_string(), "Ab".to_string()],
            Note::ASharp => vec!["A#".to_string(), "Bb".to_string()],
            _ => vec![self.name().to_string()],
        }
    }

    fn flat_name(&self) -> Option<&'static str> {
        match self {
            Note::CSharp => Some("Db"),
            Note::DSharp => Some("Eb"),
            Note::FSharp => Some("Gb"),
            Note::GSharp => Some("Ab"),
            Note::ASharp => Some("Bb"),
            _ => None,
        }
    }

    fn sharp_name(&self) -> Option<&'static str> {
        match self {
            Note::CSharp => Some("C#"),
            Note::DSharp => Some("D#"),
            Note::FSharp => Some("F#"),
            Note::GSharp => Some("G#"),
            Note::ASharp => Some("A#"),
            _ => None,
        }
    }

    fn spelling_in_key(&self, key_signature_sharps: i8) -> String {
        if !self.is_accidental() {
            return self.name().to_string();
        }

        // For sharp keys (positive), prefer sharps
        // For flat keys (negative), prefer flats
        if key_signature_sharps >= 0 {
            self.sharp_name().unwrap_or(self.name()).to_string()
        } else {
            self.flat_name().unwrap_or(self.name()).to_string()
        }
    }
}

impl ScaleDegree for Note {
    fn degree_in_major(&self, tonic: &Self) -> Option<u8> {
        // Major scale intervals from tonic: 0, 2, 4, 5, 7, 9, 11
        let major_scale = [0, 2, 4, 5, 7, 9, 11];
        let interval = tonic.interval_to(self); // interval FROM tonic TO self

        major_scale
            .iter()
            .position(|&x| x == interval)
            .map(|pos| (pos + 1) as u8)
    }

    fn degree_in_minor(&self, tonic: &Self) -> Option<u8> {
        // Natural minor scale intervals from tonic: 0, 2, 3, 5, 7, 8, 10
        let minor_scale = [0, 2, 3, 5, 7, 8, 10];
        let interval = tonic.interval_to(self); // interval FROM tonic TO self

        minor_scale
            .iter()
            .position(|&x| x == interval)
            .map(|pos| (pos + 1) as u8)
    }

    fn is_in_major_scale(&self, tonic: &Self) -> bool {
        self.degree_in_major(tonic).is_some()
    }

    fn is_in_minor_scale(&self, tonic: &Self) -> bool {
        self.degree_in_minor(tonic).is_some()
    }
}

impl FrequencyNote for Note {
    fn frequency(&self, octave: i8) -> f64 {
        // A4 (MIDI note 69) = 440 Hz
        let midi_note = (octave + 1) as f64 * 12.0 + self.semitone() as f64;
        440.0 * 2.0_f64.powf((midi_note - 69.0) / 12.0)
    }

    fn midi_number(&self, octave: i8) -> Option<u8> {
        let midi_note = (octave + 1) as i16 * 12 + self.semitone() as i16;
        if (0..=127).contains(&midi_note) {
            Some(midi_note as u8)
        } else {
            None
        }
    }

    fn from_frequency(frequency: f64) -> (Self, i8, f64) {
        // Convert frequency to MIDI note number
        let midi_float = 69.0 + 12.0 * (frequency / 440.0).log2();
        let midi_rounded = midi_float.round() as u8;

        // Extract note and octave
        let note_value = midi_rounded % 12;
        let octave = (midi_rounded / 12) as i8 - 1;
        let note = Self::from_semitone(note_value).unwrap_or(Note::C);

        // Calculate cents offset
        let exact_frequency = note.frequency(octave);
        let cents = 1200.0 * (frequency / exact_frequency).log2();

        (note, octave, cents)
    }
}

impl CircleOfFifths for Note {
    fn next_fifth(&self) -> Self {
        // Circle of fifths: C -> G -> D -> A -> E -> B -> F# -> C# -> G# -> D# -> A# -> F -> C
        let fifths_order = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5]; // semitone values
        let current_pos = fifths_order
            .iter()
            .position(|&x| x == self.semitone())
            .unwrap();
        let next_pos = (current_pos + 1) % 12;
        Self::from_semitone(fifths_order[next_pos]).unwrap()
    }

    fn prev_fifth(&self) -> Self {
        let fifths_order = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
        let current_pos = fifths_order
            .iter()
            .position(|&x| x == self.semitone())
            .unwrap();
        let prev_pos = (current_pos + 11) % 12; // +11 is same as -1 mod 12
        Self::from_semitone(fifths_order[prev_pos]).unwrap()
    }

    fn circle_position(&self) -> u8 {
        let fifths_order = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
        fifths_order
            .iter()
            .position(|&x| x == self.semitone())
            .unwrap() as u8
    }

    fn relative_key(&self) -> Self {
        // Relative minor is a minor third down from major
        // Relative major is a minor third up from minor
        self.transpose(-3)
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

    // ChromaticNote trait tests
    #[test]
    fn test_chromatic_note_name() {
        assert_eq!(Note::C.name(), "C");
        assert_eq!(Note::CSharp.name(), "C#");
        assert_eq!(Note::FSharp.name(), "F#");
    }

    #[test]
    fn test_chromatic_note_is_accidental() {
        assert!(!Note::C.is_accidental());
        assert!(!Note::D.is_accidental());
        assert!(!Note::E.is_accidental());
        assert!(Note::CSharp.is_accidental());
        assert!(Note::FSharp.is_accidental());
        assert!(Note::ASharp.is_accidental());
    }

    // TransposableNote trait tests
    #[test]
    fn test_transpose() {
        assert_eq!(Note::C.transpose(0), Note::C);
        assert_eq!(Note::C.transpose(1), Note::CSharp);
        assert_eq!(Note::C.transpose(7), Note::G);
        assert_eq!(Note::C.transpose(12), Note::C); // Octave
        assert_eq!(Note::C.transpose(-1), Note::B);
        assert_eq!(Note::G.transpose(5), Note::C);
    }

    #[test]
    fn test_interval_to() {
        assert_eq!(Note::C.interval_to(&Note::C), 0);
        assert_eq!(Note::C.interval_to(&Note::G), 7); // Perfect fifth
        assert_eq!(Note::C.interval_to(&Note::B), 11);
        assert_eq!(Note::G.interval_to(&Note::C), 5); // Perfect fourth
    }

    #[test]
    fn test_add_interval() {
        assert_eq!(Note::C.add_interval(Interval::PerfectFifth), Note::G);
        assert_eq!(Note::C.add_interval(Interval::MajorThird), Note::E);
        assert_eq!(Note::A.add_interval(Interval::MinorThird), Note::C);
    }

    // EnharmonicNote trait tests
    #[test]
    fn test_enharmonic_equivalents() {
        let csharp_equiv = Note::CSharp.enharmonic_equivalents();
        assert!(csharp_equiv.contains(&"C#".to_string()));
        assert!(csharp_equiv.contains(&"Db".to_string()));

        let c_equiv = Note::C.enharmonic_equivalents();
        assert_eq!(c_equiv, vec!["C".to_string()]);
    }

    #[test]
    fn test_flat_and_sharp_names() {
        assert_eq!(Note::CSharp.flat_name(), Some("Db"));
        assert_eq!(Note::CSharp.sharp_name(), Some("C#"));
        assert_eq!(Note::C.flat_name(), None);
        assert_eq!(Note::C.sharp_name(), None);
    }

    #[test]
    fn test_spelling_in_key() {
        // Sharp key context (G major = 1 sharp)
        assert_eq!(Note::FSharp.spelling_in_key(1), "F#");

        // Flat key context (F major = 1 flat)
        assert_eq!(Note::ASharp.spelling_in_key(-1), "Bb");

        // Natural notes are unaffected
        assert_eq!(Note::C.spelling_in_key(1), "C");
        assert_eq!(Note::C.spelling_in_key(-1), "C");
    }

    // ScaleDegree trait tests
    #[test]
    fn test_degree_in_major() {
        // C major scale: C D E F G A B
        assert_eq!(Note::C.degree_in_major(&Note::C), Some(1));
        assert_eq!(Note::D.degree_in_major(&Note::C), Some(2));
        assert_eq!(Note::E.degree_in_major(&Note::C), Some(3));
        assert_eq!(Note::F.degree_in_major(&Note::C), Some(4));
        assert_eq!(Note::G.degree_in_major(&Note::C), Some(5));
        assert_eq!(Note::A.degree_in_major(&Note::C), Some(6));
        assert_eq!(Note::B.degree_in_major(&Note::C), Some(7));
        assert_eq!(Note::CSharp.degree_in_major(&Note::C), None);
    }

    #[test]
    fn test_degree_in_minor() {
        // A minor scale: A B C D E F G
        assert_eq!(Note::A.degree_in_minor(&Note::A), Some(1));
        assert_eq!(Note::B.degree_in_minor(&Note::A), Some(2));
        assert_eq!(Note::C.degree_in_minor(&Note::A), Some(3));
        assert_eq!(Note::D.degree_in_minor(&Note::A), Some(4));
        assert_eq!(Note::E.degree_in_minor(&Note::A), Some(5));
        assert_eq!(Note::F.degree_in_minor(&Note::A), Some(6));
        assert_eq!(Note::G.degree_in_minor(&Note::A), Some(7));
        assert_eq!(Note::ASharp.degree_in_minor(&Note::A), None);
    }

    #[test]
    fn test_is_in_scale() {
        assert!(Note::C.is_in_major_scale(&Note::C));
        assert!(Note::E.is_in_major_scale(&Note::C));
        assert!(!Note::CSharp.is_in_major_scale(&Note::C));

        assert!(Note::A.is_in_minor_scale(&Note::A));
        assert!(Note::C.is_in_minor_scale(&Note::A));
        assert!(!Note::ASharp.is_in_minor_scale(&Note::A));
    }

    // FrequencyNote trait tests
    #[test]
    fn test_frequency() {
        // A4 should be 440 Hz
        assert!((Note::A.frequency(4) - 440.0).abs() < 0.001);

        // C4 should be approximately 261.63 Hz
        assert!((Note::C.frequency(4) - 261.625).abs() < 0.1);

        // A5 should be 880 Hz (octave up)
        assert!((Note::A.frequency(5) - 880.0).abs() < 0.001);
    }

    #[test]
    fn test_midi_number() {
        // Middle C (C4) = MIDI 60
        assert_eq!(Note::C.midi_number(4), Some(60));

        // A4 = MIDI 69
        assert_eq!(Note::A.midi_number(4), Some(69));

        // Test bounds
        assert_eq!(Note::C.midi_number(-2), None); // Too low
        assert_eq!(Note::G.midi_number(10), None); // Too high
    }

    #[test]
    fn test_from_frequency() {
        // Test A4 = 440 Hz
        let (note, octave, cents) = Note::from_frequency(440.0);
        assert_eq!(note, Note::A);
        assert_eq!(octave, 4);
        assert!(cents.abs() < 1.0); // Should be very close

        // Test slightly detuned note
        let (note, octave, _cents) = Note::from_frequency(445.0);
        assert_eq!(note, Note::A);
        assert_eq!(octave, 4);
    }

    // CircleOfFifths trait tests
    #[test]
    fn test_circle_of_fifths() {
        // C -> G -> D -> A -> E -> B -> F# -> C# -> G# -> D# -> A# -> F -> C
        assert_eq!(Note::C.next_fifth(), Note::G);
        assert_eq!(Note::G.next_fifth(), Note::D);
        assert_eq!(Note::F.next_fifth(), Note::C);

        assert_eq!(Note::G.prev_fifth(), Note::C);
        assert_eq!(Note::D.prev_fifth(), Note::G);
        assert_eq!(Note::C.prev_fifth(), Note::F);
    }

    #[test]
    fn test_circle_position() {
        assert_eq!(Note::C.circle_position(), 0);
        assert_eq!(Note::G.circle_position(), 1);
        assert_eq!(Note::D.circle_position(), 2);
        assert_eq!(Note::F.circle_position(), 11);
    }

    #[test]
    fn test_relative_key() {
        // C major -> A minor (down a minor third)
        assert_eq!(Note::C.relative_key(), Note::A);

        // A minor -> C major (up a minor third)
        assert_eq!(Note::A.relative_key(), Note::FSharp);

        // G major -> E minor
        assert_eq!(Note::G.relative_key(), Note::E);
    }
}
