//! Example demonstrating Note arithmetic operations with Add and Sub traits.
//!
//! This example shows how to use the + and - operators with Note and Semitone
//! to perform musical interval operations, transpositions, and harmonic calculations.

use musik_std::{Note, Semitone};

fn main() {
    println!("🎼 Note Arithmetic Operations Demo");
    println!("==================================\n");

    // Basic interval additions
    println!("📈 Basic Interval Additions:");
    let c = Note::new(0); // C
    println!("Starting note: C ({})", c.semitone());

    let major_third = Semitone::new(4);
    let e = c + major_third;
    println!("C + Major 3rd = E ({})", e.semitone());

    let perfect_fifth = Semitone::new(7);
    let g = c + perfect_fifth;
    println!("C + Perfect 5th = G ({})", g.semitone());

    let octave = Semitone::new(12);
    let high_c = c + octave;
    println!("C + Octave = high C ({})", high_c.semitone());

    println!();

    // Basic interval subtractions
    println!("📉 Basic Interval Subtractions:");
    let a = Note::new(9); // A
    println!("Starting note: A ({})", a.semitone());

    let minor_third = Semitone::new(3);
    let f_sharp = a - minor_third;
    println!("A - Minor 3rd = F# ({})", f_sharp.semitone());

    let perfect_fourth = Semitone::new(5);
    let e_from_a = a - perfect_fourth;
    println!("A - Perfect 4th = E ({})", e_from_a.semitone());

    println!();

    // Chord construction
    println!("🎵 Chord Construction:");
    let root = Note::new(0); // C major chord
    let third = root + Semitone::new(4); // E
    let fifth = root + Semitone::new(7); // G
    println!(
        "C Major Chord: C({}) - E({}) - G({})",
        root.semitone(),
        third.semitone(),
        fifth.semitone()
    );

    let d_minor_root = Note::new(2); // D minor chord
    let d_minor_third = d_minor_root + Semitone::new(3); // F (minor third)
    let d_minor_fifth = d_minor_root + Semitone::new(7); // A
    println!(
        "D Minor Chord: D({}) - F({}) - A({})",
        d_minor_root.semitone(),
        d_minor_third.semitone(),
        d_minor_fifth.semitone()
    );

    println!();

    // Transposition
    println!("🔄 Transposition:");
    let melody_notes = [0, 2, 4, 5, 7, 9, 11]; // C major scale
    let transpose_up = Semitone::new(2); // Transpose up a whole tone

    println!("Original melody (C major scale):");
    for &note_val in &melody_notes {
        print!("{} ", note_val);
    }
    println!();

    println!("Transposed up by 2 semitones (D major scale):");
    for &note_val in &melody_notes {
        let original = Note::new(note_val);
        let transposed = original + transpose_up;
        print!("{} ", transposed.semitone());
    }
    println!("\n");

    // Interval calculations
    println!("📏 Interval Calculations:");
    let note1 = Note::new(0); // C
    let note2 = Note::new(7); // G

    // Calculate the interval from C to G
    // Note: This is a simplified example - real interval calculation
    // would need to handle wrapping and direction
    println!(
        "From C({}) to G({}) is 7 semitones",
        note1.semitone(),
        note2.semitone()
    );

    // Going backwards from G to C
    let back_to_c = note2 - Semitone::new(7);
    println!("G - 7 semitones = C({})", back_to_c.semitone());

    println!();

    // Saturation behavior
    println!("🔒 Saturation Behavior:");
    let low_note = Note::new(0);
    let large_subtraction = Semitone::new(50);
    let result_low = low_note - large_subtraction;
    println!(
        "Note({}) - {} semitones = {} (saturates at 0)",
        low_note.semitone(),
        u8::from(large_subtraction),
        result_low.semitone()
    );

    let high_note = Note::new(250);
    let large_addition = Semitone::new(50);
    let result_high = high_note + large_addition;
    println!(
        "Note({}) + {} semitones = {} (saturates at 255)",
        high_note.semitone(),
        u8::from(large_addition),
        result_high.semitone()
    );

    println!();

    // Musical scales with arithmetic
    println!("🎵 Scale Construction with Arithmetic:");
    let root_note = Note::new(5); // F
    let major_scale_intervals = [0, 2, 4, 5, 7, 9, 11, 12]; // Major scale pattern

    println!("F Major Scale:");
    for &interval in &major_scale_intervals {
        let scale_note = root_note + Semitone::new(interval);
        print!("{} ", scale_note.semitone());
    }
    println!();

    println!("\n🎹 Note arithmetic operations completed successfully!");
    println!("   These operations enable harmonic analysis, transposition, and chord building.");
}
