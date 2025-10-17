use musik_std::Note;

fn main() {
    println!("Note Display Implementation Demo");
    println!("================================");

    // Show some common notes
    println!("Common notes:");
    println!("  MIDI 60 (Middle C): {}", Note::new(60));
    println!("  MIDI 69 (A440): {}", Note::new(69));
    println!("  MIDI 48 (C3): {}", Note::new(48));
    println!("  MIDI 72 (C5): {}", Note::new(72));

    println!("\nChromatic scale in octave 4:");
    for i in 60..72 {
        print!("  {} ", Note::new(i));
    }
    println!();

    println!("\nOctave progression for C:");
    for octave in 0..6 {
        let midi_note = 12 + octave * 12;
        println!(
            "  C{}: {} (MIDI {})",
            octave,
            Note::new(midi_note),
            midi_note
        );
    }

    println!("\nLow and high notes:");
    println!("  MIDI 0: {}", Note::new(0));
    println!("  MIDI 127: {}", Note::new(127));

    println!("\nOctave method demonstration:");
    let demo_notes = [0, 12, 24, 60, 69, 72, 127];
    for &midi_note in &demo_notes {
        let note = Note::new(midi_note);
        println!(
            "  MIDI {}: {} (octave: {})",
            midi_note,
            note,
            note.octave().value()
        );
    }
}
