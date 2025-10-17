//! MIDI support and utilities

use crate::theory::Note;

/// MIDI note number (0-127)
pub type MidiNote = u8;

/// MIDI velocity (0-127)
pub type Velocity = u8;

/// MIDI channel (0-15)
pub type Channel = u8;

/// Convert a MIDI note number to frequency in Hz
pub fn midi_to_frequency(midi_note: MidiNote) -> f64 {
    // A4 (MIDI note 69) = 440 Hz
    440.0 * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0)
}

/// Convert frequency in Hz to the nearest MIDI note number
pub fn frequency_to_midi(frequency: f64) -> MidiNote {
    let midi_float = 69.0 + 12.0 * (frequency / 440.0).log2();
    midi_float.round().max(0.0).min(127.0) as MidiNote
}

/// Convert a Note and octave to MIDI note number
pub fn note_to_midi(note: Note, octave: i8) -> Option<MidiNote> {
    let midi_note = (octave + 1) as i16 * 12 + note.semitone() as i16;
    if midi_note >= 0 && midi_note <= 127 {
        Some(midi_note as MidiNote)
    } else {
        None
    }
}

/// Convert MIDI note number to Note and octave
pub fn midi_to_note(midi_note: MidiNote) -> (Note, i8) {
    let octave = (midi_note / 12) as i8 - 1;
    let note_value = midi_note % 12;
    let note = Note::from_semitone(note_value).unwrap_or(Note::C);
    (note, octave)
}

/// MIDI message types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidiMessage {
    /// Note On: channel, note, velocity
    NoteOn(Channel, MidiNote, Velocity),
    /// Note Off: channel, note, velocity
    NoteOff(Channel, MidiNote, Velocity),
    /// Control Change: channel, controller, value
    ControlChange(Channel, u8, u8),
    /// Program Change: channel, program
    ProgramChange(Channel, u8),
    /// Pitch Bend: channel, value (14-bit)
    PitchBend(Channel, u16),
}

impl MidiMessage {
    /// Get the channel of the MIDI message
    pub fn channel(&self) -> Channel {
        match self {
            MidiMessage::NoteOn(ch, _, _) => *ch,
            MidiMessage::NoteOff(ch, _, _) => *ch,
            MidiMessage::ControlChange(ch, _, _) => *ch,
            MidiMessage::ProgramChange(ch, _) => *ch,
            MidiMessage::PitchBend(ch, _) => *ch,
        }
    }

    /// Check if this is a note message
    pub fn is_note_message(&self) -> bool {
        matches!(self, MidiMessage::NoteOn(_, _, _) | MidiMessage::NoteOff(_, _, _))
    }
}

/// Common MIDI constants
pub mod constants {
    /// MIDI note numbers for common notes
    pub mod notes {
        pub const C4: u8 = 60;  // Middle C
        pub const A4: u8 = 69;  // Concert A (440 Hz)
    }

    /// Common MIDI controller numbers
    pub mod controllers {
        pub const MODULATION: u8 = 1;
        pub const VOLUME: u8 = 7;
        pub const PAN: u8 = 10;
        pub const EXPRESSION: u8 = 11;
        pub const SUSTAIN_PEDAL: u8 = 64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_frequency() {
        // A4 (MIDI 69) should be 440 Hz
        assert!((midi_to_frequency(69) - 440.0).abs() < 0.001);
        
        // A5 (MIDI 81) should be 880 Hz
        assert!((midi_to_frequency(81) - 880.0).abs() < 0.001);
        
        // C4 (MIDI 60) should be ~261.63 Hz
        assert!((midi_to_frequency(60) - 261.625).abs() < 0.1);
    }

    #[test]
    fn test_frequency_to_midi() {
        assert_eq!(frequency_to_midi(440.0), 69);
        assert_eq!(frequency_to_midi(880.0), 81);
        assert_eq!(frequency_to_midi(261.625), 60);
    }

    #[test]
    fn test_note_to_midi() {
        // C4 (middle C)
        assert_eq!(note_to_midi(Note::C, 4), Some(60));
        
        // A4 (concert A)
        assert_eq!(note_to_midi(Note::A, 4), Some(69));
        
        // Test bounds
        assert_eq!(note_to_midi(Note::C, -2), None); // Too low
        assert_eq!(note_to_midi(Note::G, 10), None); // Too high
    }

    #[test]
    fn test_midi_to_note() {
        assert_eq!(midi_to_note(60), (Note::C, 4));
        assert_eq!(midi_to_note(69), (Note::A, 4));
        assert_eq!(midi_to_note(61), (Note::CSharp, 4));
    }

    #[test]
    fn test_midi_message_channel() {
        let msg = MidiMessage::NoteOn(5, 60, 100);
        assert_eq!(msg.channel(), 5);
        assert!(msg.is_note_message());
        
        let cc_msg = MidiMessage::ControlChange(2, 7, 127);
        assert_eq!(cc_msg.channel(), 2);
        assert!(!cc_msg.is_note_message());
    }
}