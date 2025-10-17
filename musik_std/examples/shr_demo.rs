//! Demonstration of the Shr trait implementation for Semitone octave shifting.

use musik_std::Semitone;

fn main() {
    println!("🎼 Semitone Octave Shifting with Shr Operator Demo\n");

    // Example 1: Basic octave shifting
    println!("📌 Basic Octave Shifting:");
    let c_minus_1 = Semitone::new(0); // C-1 (MIDI note 0)
    println!("C-1 (semitone {})", u8::from(c_minus_1));

    let c0 = c_minus_1 >> 1u8; // C0 (1 octave up)
    println!("C0 (semitone {}) = C-1 >> 1 octave", u8::from(c0));

    let c4 = c_minus_1 >> 5u8; // C4 (5 octaves up - middle C)
    println!(
        "C4 (semitone {}) = C-1 >> 5 octaves (middle C)",
        u8::from(c4)
    );

    // Example 2: Piano range demonstration
    println!("\n🎹 Piano Range Demonstration:");
    let a0 = Semitone::new(21); // A0 (lowest A on piano)
    println!("A0 (semitone {})", u8::from(a0));

    let a4 = a0 >> 4u8; // A4 (440 Hz tuning note)
    println!("A4 (semitone {}) = A0 >> 4 octaves (440 Hz)", u8::from(a4));

    let a7 = a0 >> 7u8; // A7 (high A)
    println!("A7 (semitone {}) = A0 >> 7 octaves", u8::from(a7));

    // Example 3: Guitar string transposition
    println!("\n🎸 Guitar String Transposition:");
    let low_e = Semitone::new(40); // E2 (6th string)
    println!("Low E (semitone {})", u8::from(low_e));

    let high_e = low_e >> 2u8; // E4 (1st string, 2 octaves up)
    println!(
        "High E (semitone {}) = Low E >> 2 octaves",
        u8::from(high_e)
    );

    // Example 4: Pitch class preservation
    println!("\n🎵 Pitch Class Preservation:");
    let f_sharp = Semitone::new(6); // F#-1
    println!(
        "F#-1 (semitone {}, pitch class {})",
        u8::from(f_sharp),
        u8::from(f_sharp) % 12
    );

    let f_sharp_3 = f_sharp >> 4u8; // F#3 (4 octaves up)
    println!(
        "F#3 (semitone {}, pitch class {}) = F#-1 >> 4 octaves",
        u8::from(f_sharp_3),
        u8::from(f_sharp_3) % 12
    );

    // Verify pitch class is preserved
    assert_eq!(u8::from(f_sharp) % 12, u8::from(f_sharp_3) % 12);
    println!("✅ Pitch class preserved across octave shifts!");

    // Example 5: Boundary behavior
    println!("\n⚠️  Boundary Behavior (Saturation):");
    let high_note = Semitone::new(200);
    println!("High note (semitone {})", u8::from(high_note));

    let saturated = high_note >> 10u8; // Would overflow, so saturates at 255
    println!(
        "Saturated (semitone {}) = High note >> 10 octaves (saturated at u8::MAX)",
        u8::from(saturated)
    );

    println!("\n🎼 All examples completed successfully!");
}
