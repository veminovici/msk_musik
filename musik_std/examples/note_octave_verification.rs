use musik_std::{Note, Octave};

fn main() {
    println!("Note.octave() Method Verification");
    println!("=================================");

    // User's specific example: Note(60) should return Octave(4)
    let note_60 = Note::new(60);
    let octave_4 = note_60.octave();

    println!("Note::new(60).octave() = Octave({})", octave_4.value());
    println!("Expected: Octave(4)");
    println!("Match: {}", octave_4 == Octave::new(4));

    println!("\nAdditional examples:");

    // Test various notes
    let examples = [
        (0, -1, "C-1"), // MIDI 0 -> Octave -1
        (12, 0, "C0"),  // MIDI 12 -> Octave 0
        (24, 1, "C1"),  // MIDI 24 -> Octave 1
        (48, 3, "C3"),  // MIDI 48 -> Octave 3
        (60, 4, "C4"),  // MIDI 60 -> Octave 4 (middle C)
        (72, 5, "C5"),  // MIDI 72 -> Octave 5
        (69, 4, "A4"),  // MIDI 69 -> Octave 4 (A440)
    ];

    for (midi_note, expected_octave, display_name) in examples {
        let note = Note::new(midi_note);
        let octave = note.octave();
        println!(
            "  MIDI {}: {} -> Octave({}) [{}]",
            midi_note,
            display_name,
            octave.value(),
            if octave.value() == expected_octave {
                "✓"
            } else {
                "✗"
            }
        );
    }
}
