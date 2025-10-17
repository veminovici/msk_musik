//! Demonstration of the Shl trait implementation for Semitone octave shifting.

use musik_std::Semitone;

fn main() {
    println!("🎼 Semitone Octave Shifting with Shl Operator Demo\n");

    // Example 1: Basic octave shifting (downward)
    println!("📌 Basic Octave Shifting (Down):");
    let c4 = Semitone::new(60); // C4 (middle C)
    println!("C4 (semitone {})", u8::from(c4));

    let c3 = c4 << 1u8; // C3 (1 octave down)
    println!("C3 (semitone {}) = C4 << 1 octave", u8::from(c3));

    let c_minus_1 = c4 << 5u8; // C-1 (5 octaves down)
    println!("C-1 (semitone {}) = C4 << 5 octaves", u8::from(c_minus_1));

    // Example 2: Piano range demonstration (downward)
    println!("\n🎹 Piano Range Demonstration (Downward):");
    let c8 = Semitone::new(108); // C8 (highest C on piano)
    println!("C8 (semitone {})", u8::from(c8));

    let c4_from_c8 = c8 << 4u8; // C4 (4 octaves down from C8)
    println!("C4 (semitone {}) = C8 << 4 octaves", u8::from(c4_from_c8));

    let c1 = c8 << 7u8; // C1 (7 octaves down)
    println!("C1 (semitone {}) = C8 << 7 octaves", u8::from(c1));

    // Example 3: Vocal range transposition
    println!("\n🎤 Vocal Range Transposition:");
    let soprano_c6 = Semitone::new(84); // Soprano high C
    println!("Soprano C6 (semitone {})", u8::from(soprano_c6));

    let alto_c5 = soprano_c6 << 1u8; // Alto range (1 octave down)
    println!(
        "Alto C5 (semitone {}) = Soprano C6 << 1 octave",
        u8::from(alto_c5)
    );

    let tenor_c4 = soprano_c6 << 2u8; // Tenor range (2 octaves down)
    println!(
        "Tenor C4 (semitone {}) = Soprano C6 << 2 octaves",
        u8::from(tenor_c4)
    );

    let bass_c3 = soprano_c6 << 3u8; // Bass range (3 octaves down)
    println!(
        "Bass C3 (semitone {}) = Soprano C6 << 3 octaves",
        u8::from(bass_c3)
    );

    // Example 4: Pitch class preservation
    println!("\n🎵 Pitch Class Preservation:");
    let f_sharp4 = Semitone::new(66); // F#4
    println!(
        "F#4 (semitone {}, pitch class {})",
        u8::from(f_sharp4),
        u8::from(f_sharp4) % 12
    );

    let f_sharp2 = f_sharp4 << 2u8; // F#2 (2 octaves down)
    println!(
        "F#2 (semitone {}, pitch class {}) = F#4 << 2 octaves",
        u8::from(f_sharp2),
        u8::from(f_sharp2) % 12
    );

    // Verify pitch class is preserved
    assert_eq!(u8::from(f_sharp4) % 12, u8::from(f_sharp2) % 12);
    println!("✅ Pitch class preserved across downward octave shifts!");

    // Example 5: Symmetry with Shr
    println!("\n🔄 Symmetry Test (Shl ↔ Shr):");
    let middle_c = Semitone::new(60);
    println!("Original middle C (semitone {})", u8::from(middle_c));

    let down_then_up = (middle_c << 2u8) >> 2u8; // Down 2 octaves, then up 2 octaves
    println!("Down 2, then up 2 (semitone {})", u8::from(down_then_up));
    assert_eq!(down_then_up, middle_c);

    let up_then_down = (middle_c >> 1u8) << 1u8; // Up 1 octave, then down 1 octave
    println!("Up 1, then down 1 (semitone {})", u8::from(up_then_down));
    assert_eq!(up_then_down, middle_c);

    println!("✅ Shl and Shr operations are symmetric!");

    // Example 6: Boundary behavior
    println!("\n⚠️  Boundary Behavior (Saturation):");
    let low_note = Semitone::new(6);
    println!("Low note (semitone {})", u8::from(low_note));

    let saturated = low_note << 5u8; // Would underflow, so saturates at 0
    println!(
        "Saturated (semitone {}) = Low note << 5 octaves (saturated at 0)",
        u8::from(saturated)
    );

    println!("\n🎼 All examples completed successfully!");
}
