//! Combined demonstration of Shr and Shl traits for bidirectional octave shifting.

use musik_std::Semitone;

fn main() {
    println!("🎼 Combined Shr (>>) and Shl (<<) Octave Shifting Demo\n");

    // Example 1: Bidirectional octave transposition
    println!("🔄 Bidirectional Octave Transposition:");
    let middle_c = Semitone::new(60); // C4 (middle C)
    println!("Starting point: C4 (semitone {})", u8::from(middle_c));

    let c5 = middle_c >> 1u8; // Up 1 octave
    println!("C5 = C4 >> 1:  (semitone {})", u8::from(c5));

    let c6 = c5 >> 1u8; // Up another octave
    println!("C6 = C5 >> 1:  (semitone {})", u8::from(c6));

    let back_to_c5 = c6 << 1u8; // Down 1 octave
    println!("C5 = C6 << 1:  (semitone {})", u8::from(back_to_c5));

    let back_to_c4 = back_to_c5 << 1u8; // Down 1 octave
    println!("C4 = C5 << 1:  (semitone {})", u8::from(back_to_c4));

    assert_eq!(back_to_c4, middle_c);
    println!("✅ Perfect round trip: C4 >> 2 << 2 = C4\n");

    // Example 2: Musical range exploration
    println!("🎹 Musical Range Exploration:");
    let soprano_high_c = Semitone::new(84); // C6
    println!("Soprano high C6 (semitone {})", u8::from(soprano_high_c));

    // Transpose down through vocal ranges
    let alto_range = soprano_high_c << 1u8; // C5
    let tenor_range = soprano_high_c << 2u8; // C4
    let bass_range = soprano_high_c << 3u8; // C3
    let contrabass = soprano_high_c << 4u8; // C2

    println!(
        "Alto C5:        (semitone {}) = C6 << 1",
        u8::from(alto_range)
    );
    println!(
        "Tenor C4:       (semitone {}) = C6 << 2",
        u8::from(tenor_range)
    );
    println!(
        "Bass C3:        (semitone {}) = C6 << 3",
        u8::from(bass_range)
    );
    println!(
        "Contrabass C2:  (semitone {}) = C6 << 4",
        u8::from(contrabass)
    );

    // Now transpose back up
    let back_to_soprano = bass_range >> 3u8; // C3 >> 3 = C6

    println!(
        "Back to soprano: (semitone {}) = C3 >> 3",
        u8::from(back_to_soprano)
    );
    assert_eq!(back_to_soprano, soprano_high_c);
    println!("✅ Range exploration successful!\n");

    // Example 3: Instrument tuning and octave doubling
    println!("🎸 Instrument Tuning & Octave Doubling:");
    let guitar_low_e = Semitone::new(40); // E2 (6th string)
    println!("Guitar low E2 (semitone {})", u8::from(guitar_low_e));

    let guitar_high_e = guitar_low_e >> 2u8; // E4 (1st string, 2 octaves up)
    println!(
        "Guitar high E4:  (semitone {}) = E2 >> 2",
        u8::from(guitar_high_e)
    );

    let octave_down = guitar_high_e << 1u8; // E3 (1 octave down)
    let two_octaves_down = guitar_high_e << 2u8; // E2 (2 octaves down)

    println!(
        "One octave down: (semitone {}) = E4 << 1",
        u8::from(octave_down)
    );
    println!(
        "Two octaves down:(semitone {}) = E4 << 2",
        u8::from(two_octaves_down)
    );

    assert_eq!(two_octaves_down, guitar_low_e);
    println!("✅ Guitar string octave relationships verified!\n");

    // Example 4: Pitch class consistency check
    println!("🎵 Pitch Class Consistency Across Octaves:");
    let f_sharp_notes = [
        Semitone::new(6),  // F#-1
        Semitone::new(18), // F#0
        Semitone::new(30), // F#1
        Semitone::new(42), // F#2
        Semitone::new(54), // F#3
        Semitone::new(66), // F#4
        Semitone::new(78), // F#5
    ];

    println!("F# notes across octaves:");
    for (i, &note) in f_sharp_notes.iter().enumerate() {
        let note_value = u8::from(note);
        let octave = i as i8 - 1; // -1, 0, 1, 2, 3, 4, 5
        println!(
            "  F#{}: semitone {}, pitch class {}",
            octave,
            note_value,
            note_value % 12
        );
        assert_eq!(note_value % 12, 6); // All should have pitch class 6
    }

    // Test bidirectional movement preserves pitch class
    let base_f_sharp = f_sharp_notes[3]; // F#2
    let up_down = (base_f_sharp >> 2u8) << 2u8; // Up 2, down 2
    let down_up = (base_f_sharp << 1u8) >> 1u8; // Down 1, up 1

    assert_eq!(up_down, base_f_sharp);
    assert_eq!(down_up, base_f_sharp);
    println!("✅ Pitch class F# preserved across all operations!\n");

    // Example 5: Musical interval relationships
    println!("🎶 Musical Interval Relationships:");
    let c3 = Semitone::new(48);
    println!("C3 (semitone {})", u8::from(c3));

    // Build intervals using octave shifts
    let c4 = c3 >> 1u8; // Perfect octave up
    let c5 = c4 >> 1u8; // Another octave up
    let c2 = c3 << 1u8; // Perfect octave down
    let c1 = c2 << 1u8; // Another octave down

    println!("Perfect octave relationships:");
    println!("  C1: {} = C3 << 2", u8::from(c1));
    println!("  C2: {} = C3 << 1", u8::from(c2));
    println!("  C3: {} (reference)", u8::from(c3));
    println!("  C4: {} = C3 >> 1", u8::from(c4));
    println!("  C5: {} = C3 >> 2", u8::from(c5));

    // Verify mathematical relationships
    assert_eq!(u8::from(c4) - u8::from(c3), 12); // Octave = 12 semitones
    assert_eq!(u8::from(c5) - u8::from(c4), 12); // Octave = 12 semitones
    assert_eq!(u8::from(c3) - u8::from(c2), 12); // Octave = 12 semitones
    assert_eq!(u8::from(c2) - u8::from(c1), 12); // Octave = 12 semitones
    println!("✅ All octave intervals are exactly 12 semitones!\n");

    println!("🎼 Combined Shr/Shl demonstration completed successfully!");
    println!("   >> operator: moves semitones UP by octaves");
    println!("   << operator: moves semitones DOWN by octaves");
    println!("   Both preserve pitch class and provide safe saturation!");
}
