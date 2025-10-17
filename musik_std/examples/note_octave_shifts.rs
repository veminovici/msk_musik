//! Example demonstrating Note octave shifting with Shr and Shl traits.
//!
//! This example shows how to use the >> and << operators with Note and u8
//! to perform octave transpositions up and down the musical range.

use musik_std::Note;

fn main() {
    println!("🎼 Note Octave Shifting Demo");
    println!("============================\n");

    // Basic octave shifting
    println!("📈 Basic Octave Shifting:");
    let middle_c = Note::new(60); // Middle C (MIDI 60)
    println!("Starting note: Middle C ({})", middle_c.semitone());

    let high_c = middle_c >> 1u8; // Up 1 octave
    println!("Middle C >> 1 octave = High C ({})", high_c.semitone());

    let low_c = middle_c << 1u8; // Down 1 octave
    println!("Middle C << 1 octave = Low C ({})", low_c.semitone());

    println!();

    // Multi-octave shifts
    println!("🎹 Multi-Octave Shifts:");
    let c0 = Note::new(12); // C1
    println!("Starting note: C1 ({})", c0.semitone());

    let c3 = c0 >> 2u8; // Up 2 octaves
    println!("C1 >> 2 octaves = C3 ({})", c3.semitone());

    let c5 = c3 >> 2u8; // Up another 2 octaves
    println!("C3 >> 2 octaves = C5 ({})", c5.semitone());

    let back_to_c3 = c5 << 2u8; // Down 2 octaves
    println!("C5 << 2 octaves = back to C3 ({})", back_to_c3.semitone());

    println!();

    // Vocal range demonstrations
    println!("🎤 Vocal Range Examples:");

    // Soprano range
    let soprano_high_c = Note::new(84); // C7
    let soprano_middle_c = soprano_high_c << 1u8; // Down to C6
    println!(
        "Soprano: High C7({}) >> down 1 octave = C6({})",
        soprano_high_c.semitone(),
        soprano_middle_c.semitone()
    );

    // Bass range
    let tenor_c = Note::new(48); // C4 (Tenor C)
    let bass_c = tenor_c << 1u8; // Down to C3 (Bass C)
    println!(
        "Bass: Tenor C4({}) >> down 1 octave = Bass C3({})",
        tenor_c.semitone(),
        bass_c.semitone()
    );

    println!();

    // Piano range exploration
    println!("🎹 Piano Range Exploration:");
    let piano_middle_c = Note::new(60); // Middle C

    // Go up the piano
    let treble_range = piano_middle_c >> 2u8; // Up 2 octaves
    println!(
        "Piano Middle C({}) >> up 2 octaves = Treble range ({})",
        piano_middle_c.semitone(),
        treble_range.semitone()
    );

    // Go down the piano
    let bass_range = piano_middle_c << 2u8; // Down 2 octaves
    println!(
        "Piano Middle C({}) >> down 2 octaves = Bass range ({})",
        piano_middle_c.semitone(),
        bass_range.semitone()
    );

    println!();

    // Symmetry demonstration
    println!("⚖️  Symmetry Demonstration:");
    let test_notes = [24, 36, 48, 60, 72];

    for &note_value in &test_notes {
        let note = Note::new(note_value);
        let up_then_down = (note >> 1u8) << 1u8;
        let down_then_up = (note << 1u8) >> 1u8;

        println!(
            "Note {}: up then down = {}, down then up = {}",
            note_value,
            up_then_down.semitone(),
            down_then_up.semitone()
        );
    }

    println!();

    // Saturation behavior
    println!("🔒 Saturation Behavior:");

    // Test upward saturation
    let high_note = Note::new(250);
    let saturated_up = high_note >> 1u8;
    println!(
        "High note ({}) >> up 1 octave = {} (saturated at 255)",
        high_note.semitone(),
        saturated_up.semitone()
    );

    // Test downward saturation
    let low_note = Note::new(5);
    let saturated_down = low_note << 1u8;
    println!(
        "Low note ({}) >> down 1 octave = {} (saturated at 0)",
        low_note.semitone(),
        saturated_down.semitone()
    );

    println!();

    // Chord voicing across octaves
    println!("🎵 Chord Voicing Across Octaves:");
    let c_major_notes = [60, 64, 67]; // C4, E4, G4 (C major chord)

    println!("Original C Major chord:");
    for &note_val in &c_major_notes {
        print!("{} ", note_val);
    }
    println!();

    println!("Moved up 1 octave:");
    for &note_val in &c_major_notes {
        let note = Note::new(note_val);
        let shifted = note >> 1u8;
        print!("{} ", shifted.semitone());
    }
    println!();

    println!("Moved down 1 octave:");
    for &note_val in &c_major_notes {
        let note = Note::new(note_val);
        let shifted = note << 1u8;
        print!("{} ", shifted.semitone());
    }
    println!();

    // Musical intervals preservation
    println!("\n🎼 Musical Intervals Preserved:");
    let note1 = Note::new(60); // C4
    let note2 = Note::new(64); // E4 (major third above C)

    let interval_original = note2.semitone() - note1.semitone();

    let note1_up = note1 >> 1u8;
    let note2_up = note2 >> 1u8;
    let interval_shifted = note2_up.semitone() - note1_up.semitone();

    println!("Original interval: {} semitones", interval_original);
    println!("After octave shift: {} semitones", interval_shifted);
    println!(
        "Interval preserved: {}",
        interval_original == interval_shifted
    );

    println!("\n🎹 Note octave shifting completed successfully!");
    println!("   These operations enable efficient voice leading and chord inversions.");
}
