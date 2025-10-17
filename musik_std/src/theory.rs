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

/// Musical scale operations and note enumeration
pub trait MusicalScale<T>
where
    T: ChromaticNote + Copy,
{
    /// Get the root note of the scale
    fn root(&self) -> T;

    /// Get all notes in the scale as a vector
    fn notes(&self) -> Vec<T>;

    /// Get the degree (1-based) of a note in this scale
    /// Returns None if the note is not in the scale
    fn degree_of(&self, note: &T) -> Option<u8>;

    /// Get the note at a specific degree (1-based)
    /// Returns None if the degree is out of range
    fn note_at_degree(&self, degree: u8) -> Option<T>;

    /// Check if a note belongs to this scale
    fn contains(&self, note: &T) -> bool {
        self.degree_of(note).is_some()
    }

    /// Get the number of notes in the scale
    fn len(&self) -> usize {
        self.notes().len()
    }

    /// Check if the scale is empty (should typically be false for valid scales)
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the scale pattern as semitone intervals from the root
    fn intervals(&self) -> Vec<u8>;

    /// Create a scale mode starting from a different degree
    /// Returns a new scale with the same interval pattern but starting from the specified degree
    fn mode(&self, degree: u8) -> Option<Self>
    where
        Self: Sized;
}

/// Common scale types with predefined interval patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    ChromaticScale,
    PentatonicMajor,
    PentatonicMinor,
    BluesScale,
    WholeTone,
}

impl ScaleType {
    /// Get the interval pattern (semitones from root) for this scale type
    pub fn intervals(&self) -> Vec<u8> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::NaturalMinor => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleType::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            ScaleType::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            ScaleType::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            ScaleType::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            ScaleType::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            ScaleType::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            ScaleType::Locrian => vec![0, 1, 3, 5, 6, 8, 10],
            ScaleType::ChromaticScale => vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            ScaleType::PentatonicMajor => vec![0, 2, 4, 7, 9],
            ScaleType::PentatonicMinor => vec![0, 3, 5, 7, 10],
            ScaleType::BluesScale => vec![0, 3, 5, 6, 7, 10],
            ScaleType::WholeTone => vec![0, 2, 4, 6, 8, 10],
        }
    }

    /// Get the name of the scale type
    pub fn name(&self) -> &'static str {
        match self {
            ScaleType::Major => "Major",
            ScaleType::NaturalMinor => "Natural Minor",
            ScaleType::HarmonicMinor => "Harmonic Minor",
            ScaleType::MelodicMinor => "Melodic Minor",
            ScaleType::Dorian => "Dorian",
            ScaleType::Phrygian => "Phrygian",
            ScaleType::Lydian => "Lydian",
            ScaleType::Mixolydian => "Mixolydian",
            ScaleType::Locrian => "Locrian",
            ScaleType::ChromaticScale => "Chromatic",
            ScaleType::PentatonicMajor => "Pentatonic Major",
            ScaleType::PentatonicMinor => "Pentatonic Minor",
            ScaleType::BluesScale => "Blues",
            ScaleType::WholeTone => "Whole Tone",
        }
    }
}

/// Represents a scale degree with possible alterations (flat/sharp)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Degree {
    /// Natural degree (1, 2, 3, 4, 5, 6, 7)
    Natural(u8),
    /// Flat degree (♭2, ♭3, ♭5, ♭6, ♭7)
    Flat(u8),
    /// Sharp degree (♯1, ♯2, ♯4, ♯5, ♯6)
    Sharp(u8),
}

impl Degree {
    /// Create a natural degree
    pub fn natural(degree: u8) -> Self {
        Degree::Natural(degree)
    }
    
    /// Create a flat degree
    pub fn flat(degree: u8) -> Self {
        Degree::Flat(degree)
    }
    
    /// Create a sharp degree
    pub fn sharp(degree: u8) -> Self {
        Degree::Sharp(degree)
    }
    
    /// Get the base degree number (1-7)
    pub fn base_degree(&self) -> u8 {
        match self {
            Degree::Natural(d) | Degree::Flat(d) | Degree::Sharp(d) => *d,
        }
    }
    
    /// Get the alteration (-1 for flat, 0 for natural, +1 for sharp)
    pub fn alteration(&self) -> i8 {
        match self {
            Degree::Natural(_) => 0,
            Degree::Flat(_) => -1,
            Degree::Sharp(_) => 1,
        }
    }
    
    /// Convert degree to semitone offset from root in a major scale context
    pub fn to_semitone_offset(&self) -> Option<u8> {
        let base_semitones = match self.base_degree() {
            1 => 0,  // Root
            2 => 2,  // Major 2nd
            3 => 4,  // Major 3rd
            4 => 5,  // Perfect 4th
            5 => 7,  // Perfect 5th
            6 => 9,  // Major 6th
            7 => 11, // Major 7th
            _ => return None,
        };
        
        let adjusted = (base_semitones as i8 + self.alteration()) as u8;
        Some(adjusted % 12)
    }
    
    /// Get the symbol representation of the degree
    pub fn symbol(&self) -> String {
        match self {
            Degree::Natural(d) => d.to_string(),
            Degree::Flat(d) => format!("♭{}", d),
            Degree::Sharp(d) => format!("♯{}", d),
        }
    }
    
    /// Get the name representation of the degree
    pub fn name(&self) -> String {
        match self {
            Degree::Natural(d) => format!("{}", d),
            Degree::Flat(d) => format!("flat {}", d),
            Degree::Sharp(d) => format!("sharp {}", d),
        }
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// Trait for working with collections of degrees
pub trait DegreeCollection<T>
where
    T: ChromaticNote + Copy,
{
    /// Get notes for a collection of degrees
    fn notes_for_degrees(&self, degrees: &[Degree]) -> Vec<Option<T>>;
    
    /// Get notes for degrees, filtering out invalid degrees
    fn valid_notes_for_degrees(&self, degrees: &[Degree]) -> Vec<T>;
    
    /// Check if a degree is valid for this scale
    fn is_valid_degree(&self, degree: &Degree) -> bool;
    
    /// Get the note for a specific degree
    fn note_for_degree(&self, degree: &Degree) -> Option<T>;
}

/// A concrete implementation of a musical scale
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale<T>
where
    T: ChromaticNote + Copy,
{
    root: T,
    scale_type: ScaleType,
}

impl<T> Scale<T>
where
    T: ChromaticNote + Copy,
{
    /// Create a new scale with the given root note and scale type
    pub fn new(root: T, scale_type: ScaleType) -> Self {
        Self { root, scale_type }
    }

    /// Get the scale type
    pub fn scale_type(&self) -> ScaleType {
        self.scale_type
    }

    /// Get the name of the scale (e.g., "C Major", "A Minor")
    pub fn name(&self) -> String {
        format!("{} {}", self.root.name(), self.scale_type.name())
    }
}

impl<T> MusicalScale<T> for Scale<T>
where
    T: ChromaticNote + Copy,
{
    fn root(&self) -> T {
        self.root
    }

    fn notes(&self) -> Vec<T> {
        self.intervals()
            .into_iter()
            .filter_map(|interval| {
                let semitone = (self.root.semitone() + interval) % 12;
                T::from_semitone(semitone)
            })
            .collect()
    }

    fn degree_of(&self, note: &T) -> Option<u8> {
        let note_semitone = note.semitone();
        let root_semitone = self.root.semitone();
        let interval = (note_semitone + 12 - root_semitone) % 12;

        self.intervals()
            .iter()
            .position(|&scale_interval| scale_interval == interval)
            .map(|pos| (pos + 1) as u8)
    }

    fn note_at_degree(&self, degree: u8) -> Option<T> {
        if degree == 0 || degree as usize > self.intervals().len() {
            return None;
        }

        let interval = self.intervals()[(degree - 1) as usize];
        let semitone = (self.root.semitone() + interval) % 12;
        T::from_semitone(semitone)
    }

    fn intervals(&self) -> Vec<u8> {
        self.scale_type.intervals()
    }

    fn mode(&self, degree: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(new_root) = self.note_at_degree(degree) {
            // For modes, we need to rotate the interval pattern
            let intervals = self.intervals();
            let start_idx = (degree - 1) as usize;

            if start_idx >= intervals.len() {
                return None;
            }

            // Create a new interval pattern starting from the specified degree
            let mut new_intervals = vec![0];
            let base_interval = intervals[start_idx];

            for i in 1..intervals.len() {
                let idx = (start_idx + i) % intervals.len();
                let interval = (intervals[idx] + 12 - base_interval) % 12;
                new_intervals.push(interval);
            }

            // Create a temporary scale type for the mode
            // Note: This is a simplified approach; in a full implementation,
            // you might want to have specific mode types or a more flexible system
            Some(Scale::new(new_root, self.scale_type))
        } else {
            None
        }
    }
}

impl<T> DegreeCollection<T> for Scale<T>
where
    T: ChromaticNote + Copy,
{
    fn notes_for_degrees(&self, degrees: &[Degree]) -> Vec<Option<T>> {
        degrees.iter().map(|degree| self.note_for_degree(degree)).collect()
    }
    
    fn valid_notes_for_degrees(&self, degrees: &[Degree]) -> Vec<T> {
        degrees
            .iter()
            .filter_map(|degree| self.note_for_degree(degree))
            .collect()
    }
    
    fn is_valid_degree(&self, degree: &Degree) -> bool {
        self.note_for_degree(degree).is_some()
    }
    
    fn note_for_degree(&self, degree: &Degree) -> Option<T> {
        // Get the semitone offset for this degree
        let semitone_offset = degree.to_semitone_offset()?;
        
        // Calculate the actual semitone value
        let target_semitone = (self.root.semitone() + semitone_offset) % 12;
        
        // Create the note from semitone
        T::from_semitone(target_semitone)
    }
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

    // MusicalScale and Scale tests
    #[test]
    fn test_scale_creation() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        assert_eq!(c_major.root(), Note::C);
        assert_eq!(c_major.scale_type(), ScaleType::Major);
        assert_eq!(c_major.name(), "C Major");

        let a_minor = Scale::new(Note::A, ScaleType::NaturalMinor);
        assert_eq!(a_minor.root(), Note::A);
        assert_eq!(a_minor.scale_type(), ScaleType::NaturalMinor);
        assert_eq!(a_minor.name(), "A Natural Minor");
    }

    #[test]
    fn test_c_major_scale() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        let notes = c_major.notes();

        // C major: C D E F G A B
        let expected = vec![
            Note::C,
            Note::D,
            Note::E,
            Note::F,
            Note::G,
            Note::A,
            Note::B,
        ];
        assert_eq!(notes, expected);
        assert_eq!(c_major.len(), 7);
        assert!(!c_major.is_empty());
    }

    #[test]
    fn test_a_minor_scale() {
        let a_minor = Scale::new(Note::A, ScaleType::NaturalMinor);
        let notes = a_minor.notes();

        // A minor: A B C D E F G
        let expected = vec![
            Note::A,
            Note::B,
            Note::C,
            Note::D,
            Note::E,
            Note::F,
            Note::G,
        ];
        assert_eq!(notes, expected);
    }

    #[test]
    fn test_scale_degrees() {
        let c_major = Scale::new(Note::C, ScaleType::Major);

        // Test degree_of
        assert_eq!(c_major.degree_of(&Note::C), Some(1));
        assert_eq!(c_major.degree_of(&Note::D), Some(2));
        assert_eq!(c_major.degree_of(&Note::E), Some(3));
        assert_eq!(c_major.degree_of(&Note::F), Some(4));
        assert_eq!(c_major.degree_of(&Note::G), Some(5));
        assert_eq!(c_major.degree_of(&Note::A), Some(6));
        assert_eq!(c_major.degree_of(&Note::B), Some(7));
        assert_eq!(c_major.degree_of(&Note::CSharp), None);

        // Test note_at_degree
        assert_eq!(c_major.note_at_degree(1), Some(Note::C));
        assert_eq!(c_major.note_at_degree(2), Some(Note::D));
        assert_eq!(c_major.note_at_degree(7), Some(Note::B));
        assert_eq!(c_major.note_at_degree(0), None);
        assert_eq!(c_major.note_at_degree(8), None);
    }

    #[test]
    fn test_scale_contains() {
        let c_major = Scale::new(Note::C, ScaleType::Major);

        assert!(c_major.contains(&Note::C));
        assert!(c_major.contains(&Note::E));
        assert!(c_major.contains(&Note::G));
        assert!(!c_major.contains(&Note::CSharp));
        assert!(!c_major.contains(&Note::FSharp));
    }

    #[test]
    fn test_scale_intervals() {
        let major_intervals = ScaleType::Major.intervals();
        assert_eq!(major_intervals, vec![0, 2, 4, 5, 7, 9, 11]);

        let minor_intervals = ScaleType::NaturalMinor.intervals();
        assert_eq!(minor_intervals, vec![0, 2, 3, 5, 7, 8, 10]);

        let pentatonic_intervals = ScaleType::PentatonicMajor.intervals();
        assert_eq!(pentatonic_intervals, vec![0, 2, 4, 7, 9]);
    }

    #[test]
    fn test_pentatonic_scales() {
        let c_penta_major = Scale::new(Note::C, ScaleType::PentatonicMajor);
        let notes = c_penta_major.notes();

        // C pentatonic major: C D E G A
        let expected = vec![Note::C, Note::D, Note::E, Note::G, Note::A];
        assert_eq!(notes, expected);
        assert_eq!(c_penta_major.len(), 5);

        let a_penta_minor = Scale::new(Note::A, ScaleType::PentatonicMinor);
        let minor_notes = a_penta_minor.notes();

        // A pentatonic minor: A C D E G
        let expected_minor = vec![Note::A, Note::C, Note::D, Note::E, Note::G];
        assert_eq!(minor_notes, expected_minor);
    }

    #[test]
    fn test_blues_scale() {
        let c_blues = Scale::new(Note::C, ScaleType::BluesScale);
        let notes = c_blues.notes();

        // C blues: C Eb F F# G Bb
        let expected = vec![
            Note::C,
            Note::DSharp,
            Note::F,
            Note::FSharp,
            Note::G,
            Note::ASharp,
        ];
        assert_eq!(notes, expected);
        assert_eq!(c_blues.len(), 6);
    }

    #[test]
    fn test_chromatic_scale() {
        let c_chromatic = Scale::new(Note::C, ScaleType::ChromaticScale);
        let notes = c_chromatic.notes();

        assert_eq!(notes.len(), 12);
        assert_eq!(notes[0], Note::C);
        assert_eq!(notes[1], Note::CSharp);
        assert_eq!(notes[11], Note::B);
    }

    #[test]
    fn test_modes() {
        let c_major = Scale::new(Note::C, ScaleType::Major);

        // D Dorian (2nd mode of C major)
        let d_dorian = c_major.mode(2);
        assert!(d_dorian.is_some());
        let d_mode = d_dorian.unwrap();
        assert_eq!(d_mode.root(), Note::D);

        // E Phrygian (3rd mode of C major)
        let e_phrygian = c_major.mode(3);
        assert!(e_phrygian.is_some());
        let e_mode = e_phrygian.unwrap();
        assert_eq!(e_mode.root(), Note::E);

        // Invalid mode
        let invalid_mode = c_major.mode(8);
        assert!(invalid_mode.is_none());
    }

    #[test]
    fn test_scale_type_names() {
        assert_eq!(ScaleType::Major.name(), "Major");
        assert_eq!(ScaleType::NaturalMinor.name(), "Natural Minor");
        assert_eq!(ScaleType::Dorian.name(), "Dorian");
        assert_eq!(ScaleType::PentatonicMajor.name(), "Pentatonic Major");
        assert_eq!(ScaleType::BluesScale.name(), "Blues");
    }

    #[test]
    fn test_transposed_scales() {
        // Test that scales work correctly with different root notes
        let g_major = Scale::new(Note::G, ScaleType::Major);
        let notes = g_major.notes();

        // G major: G A B C D E F#
        let expected = vec![
            Note::G,
            Note::A,
            Note::B,
            Note::C,
            Note::D,
            Note::E,
            Note::FSharp,
        ];
        assert_eq!(notes, expected);

        // Test degrees in G major
        assert_eq!(g_major.degree_of(&Note::G), Some(1));
        assert_eq!(g_major.degree_of(&Note::B), Some(3));
        assert_eq!(g_major.degree_of(&Note::FSharp), Some(7));
        assert_eq!(g_major.degree_of(&Note::F), None); // F natural not in G major
    }

    #[test]
    fn test_harmonic_minor() {
        let a_harmonic_minor = Scale::new(Note::A, ScaleType::HarmonicMinor);
        let notes = a_harmonic_minor.notes();

        // A harmonic minor: A B C D E F G#
        let expected = vec![
            Note::A,
            Note::B,
            Note::C,
            Note::D,
            Note::E,
            Note::F,
            Note::GSharp,
        ];
        assert_eq!(notes, expected);
    }

    #[test]
    fn test_whole_tone_scale() {
        let c_whole_tone = Scale::new(Note::C, ScaleType::WholeTone);
        let notes = c_whole_tone.notes();

        // C whole tone: C D E F# G# A#
        let expected = vec![
            Note::C,
            Note::D,
            Note::E,
            Note::FSharp,
            Note::GSharp,
            Note::ASharp,
        ];
        assert_eq!(notes, expected);
        assert_eq!(c_whole_tone.len(), 6);
    }

    // Degree tests
    #[test]
    fn test_degree_creation() {
        let natural_3 = Degree::natural(3);
        let flat_3 = Degree::flat(3);
        let sharp_5 = Degree::sharp(5);

        assert_eq!(natural_3.base_degree(), 3);
        assert_eq!(flat_3.base_degree(), 3);
        assert_eq!(sharp_5.base_degree(), 5);

        assert_eq!(natural_3.alteration(), 0);
        assert_eq!(flat_3.alteration(), -1);
        assert_eq!(sharp_5.alteration(), 1);
    }

    #[test]
    fn test_degree_symbols() {
        assert_eq!(Degree::natural(1).symbol(), "1");
        assert_eq!(Degree::flat(3).symbol(), "♭3");
        assert_eq!(Degree::sharp(5).symbol(), "♯5");
        
        assert_eq!(Degree::natural(7).name(), "7");
        assert_eq!(Degree::flat(7).name(), "flat 7");
        assert_eq!(Degree::sharp(4).name(), "sharp 4");
    }

    #[test]
    fn test_degree_to_semitone_offset() {
        // Natural degrees
        assert_eq!(Degree::natural(1).to_semitone_offset(), Some(0));  // Root
        assert_eq!(Degree::natural(2).to_semitone_offset(), Some(2));  // Major 2nd
        assert_eq!(Degree::natural(3).to_semitone_offset(), Some(4));  // Major 3rd
        assert_eq!(Degree::natural(4).to_semitone_offset(), Some(5));  // Perfect 4th
        assert_eq!(Degree::natural(5).to_semitone_offset(), Some(7));  // Perfect 5th
        assert_eq!(Degree::natural(6).to_semitone_offset(), Some(9));  // Major 6th
        assert_eq!(Degree::natural(7).to_semitone_offset(), Some(11)); // Major 7th

        // Flat degrees
        assert_eq!(Degree::flat(2).to_semitone_offset(), Some(1));  // Minor 2nd
        assert_eq!(Degree::flat(3).to_semitone_offset(), Some(3));  // Minor 3rd
        assert_eq!(Degree::flat(5).to_semitone_offset(), Some(6));  // Diminished 5th
        assert_eq!(Degree::flat(6).to_semitone_offset(), Some(8));  // Minor 6th
        assert_eq!(Degree::flat(7).to_semitone_offset(), Some(10)); // Minor 7th

        // Sharp degrees
        assert_eq!(Degree::sharp(1).to_semitone_offset(), Some(1));  // Sharp root
        assert_eq!(Degree::sharp(4).to_semitone_offset(), Some(6));  // Sharp 4th
        assert_eq!(Degree::sharp(5).to_semitone_offset(), Some(8));  // Sharp 5th
    }

    #[test]
    fn test_note_for_degree_c_major() {
        let c_major = Scale::new(Note::C, ScaleType::Major);

        // Natural degrees in C major
        assert_eq!(c_major.note_for_degree(&Degree::natural(1)), Some(Note::C));
        assert_eq!(c_major.note_for_degree(&Degree::natural(2)), Some(Note::D));
        assert_eq!(c_major.note_for_degree(&Degree::natural(3)), Some(Note::E));
        assert_eq!(c_major.note_for_degree(&Degree::natural(4)), Some(Note::F));
        assert_eq!(c_major.note_for_degree(&Degree::natural(5)), Some(Note::G));
        assert_eq!(c_major.note_for_degree(&Degree::natural(6)), Some(Note::A));
        assert_eq!(c_major.note_for_degree(&Degree::natural(7)), Some(Note::B));

        // Flat degrees
        assert_eq!(c_major.note_for_degree(&Degree::flat(2)), Some(Note::CSharp)); // Db
        assert_eq!(c_major.note_for_degree(&Degree::flat(3)), Some(Note::DSharp)); // Eb
        assert_eq!(c_major.note_for_degree(&Degree::flat(7)), Some(Note::ASharp)); // Bb
    }

    #[test]
    fn test_notes_for_degrees_major_triad() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // Major triad: 1, 3, 5
        let major_degrees = vec![
            Degree::natural(1),
            Degree::natural(3),
            Degree::natural(5),
        ];
        
        let notes = c_major.valid_notes_for_degrees(&major_degrees);
        assert_eq!(notes, vec![Note::C, Note::E, Note::G]);
        
        let all_notes = c_major.notes_for_degrees(&major_degrees);
        assert_eq!(all_notes, vec![Some(Note::C), Some(Note::E), Some(Note::G)]);
    }

    #[test]
    fn test_notes_for_degrees_minor_triad() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // Minor triad: 1, ♭3, 5
        let minor_degrees = vec![
            Degree::natural(1),
            Degree::flat(3),
            Degree::natural(5),
        ];
        
        let notes = c_major.valid_notes_for_degrees(&minor_degrees);
        assert_eq!(notes, vec![Note::C, Note::DSharp, Note::G]); // C Eb G
    }

    #[test]
    fn test_notes_for_degrees_seventh_chords() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // Major 7th chord: 1, 3, 5, 7
        let maj7_degrees = vec![
            Degree::natural(1),
            Degree::natural(3),
            Degree::natural(5),
            Degree::natural(7),
        ];
        
        let maj7_notes = c_major.valid_notes_for_degrees(&maj7_degrees);
        assert_eq!(maj7_notes, vec![Note::C, Note::E, Note::G, Note::B]);
        
        // Dominant 7th chord: 1, 3, 5, ♭7
        let dom7_degrees = vec![
            Degree::natural(1),
            Degree::natural(3),
            Degree::natural(5),
            Degree::flat(7),
        ];
        
        let dom7_notes = c_major.valid_notes_for_degrees(&dom7_degrees);
        assert_eq!(dom7_notes, vec![Note::C, Note::E, Note::G, Note::ASharp]); // C E G Bb
    }

    #[test]
    fn test_notes_for_degrees_complex_chord() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // Complex chord: 1, ♭3, ♯4, ♭7
        let complex_degrees = vec![
            Degree::natural(1),
            Degree::flat(3),
            Degree::sharp(4),
            Degree::flat(7),
        ];
        
        let notes = c_major.valid_notes_for_degrees(&complex_degrees);
        assert_eq!(notes, vec![Note::C, Note::DSharp, Note::FSharp, Note::ASharp]); // C Eb F# Bb
    }

    #[test]
    fn test_notes_for_degrees_different_keys() {
        // Test in G major
        let g_major = Scale::new(Note::G, ScaleType::Major);
        
        let major_triad = vec![
            Degree::natural(1),
            Degree::natural(3),
            Degree::natural(5),
        ];
        
        let g_notes = g_major.valid_notes_for_degrees(&major_triad);
        assert_eq!(g_notes, vec![Note::G, Note::B, Note::D]); // G B D
        
        // Test minor triad in same key
        let minor_triad = vec![
            Degree::natural(1),
            Degree::flat(3),
            Degree::natural(5),
        ];
        
        let g_minor_notes = g_major.valid_notes_for_degrees(&minor_triad);
        assert_eq!(g_minor_notes, vec![Note::G, Note::ASharp, Note::D]); // G Bb D
    }

    #[test]
    fn test_is_valid_degree() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // All natural degrees should be valid
        assert!(c_major.is_valid_degree(&Degree::natural(1)));
        assert!(c_major.is_valid_degree(&Degree::natural(7)));
        
        // Flat and sharp degrees should also be valid (they resolve to chromatic notes)
        assert!(c_major.is_valid_degree(&Degree::flat(3)));
        assert!(c_major.is_valid_degree(&Degree::sharp(4)));
        
        // Invalid degree numbers should be invalid
        assert_eq!(c_major.note_for_degree(&Degree::natural(0)), None);
        assert_eq!(c_major.note_for_degree(&Degree::natural(8)), None);
    }

    #[test]
    fn test_degree_display() {
        assert_eq!(format!("{}", Degree::natural(1)), "1");
        assert_eq!(format!("{}", Degree::flat(3)), "♭3");
        assert_eq!(format!("{}", Degree::sharp(5)), "♯5");
    }

    #[test]
    fn test_blues_scale_degrees() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        
        // Blues scale degrees: 1, ♭3, 4, ♭5, 5, ♭7
        let blues_degrees = vec![
            Degree::natural(1),
            Degree::flat(3),
            Degree::natural(4),
            Degree::flat(5),
            Degree::natural(5),
            Degree::flat(7),
        ];
        
        let blues_notes = c_major.valid_notes_for_degrees(&blues_degrees);
        assert_eq!(blues_notes, vec![
            Note::C,      // 1
            Note::DSharp, // ♭3 (Eb)
            Note::F,      // 4
            Note::FSharp, // ♭5 (F#/Gb)
            Note::G,      // 5
            Note::ASharp, // ♭7 (Bb)
        ]);
    }
}
