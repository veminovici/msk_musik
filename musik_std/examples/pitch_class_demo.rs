use musik_std::{PitchClass, C, C_SHARP, D_FLAT, F_SHARP, G_FLAT, G, A_SHARP, B_FLAT};

fn main() {
    println!("=== PitchClass Demonstration ===\n");

    // Basic creation and display
    println!("1. Basic PitchClass creation:");
    let c = PitchClass::new(0);
    let c_sharp = PitchClass::new(1);
    let d = PitchClass::new(2);

    println!("   PitchClass::new(0) = {}", c);
    println!("   PitchClass::new(1) = {}", c_sharp);
    println!("   PitchClass::new(2) = {}", d);

    // Wrapping behavior
    println!("\n2. Wrapping behavior (values > 11):");
    let wrapped_c = PitchClass::new(12); // Should wrap to C (0)
    let wrapped_d = PitchClass::new(14); // Should wrap to D (2)

    println!("   PitchClass::new(12) = {} (wraps to C)", wrapped_c);
    println!("   PitchClass::new(14) = {} (wraps to D)", wrapped_d);

    // Using constants
    println!("\n3. Using predefined constants:");
    println!("   C = {}", C);
    println!("   C_SHARP = {}", C_SHARP);
    println!("   D_FLAT = {} (same as C#)", D_FLAT);
    println!("   F_SHARP = {}", F_SHARP);
    println!("   G_FLAT = {} (same as F#)", G_FLAT);

    // All pitch classes
    println!("\n4. All 12 pitch classes:");
    print!("   ");
    for pc in PitchClass::all() {
        print!("{} ", pc);
    }
    println!();

    // Conversion to/from u8
    println!("\n8. Conversion to/from u8:");
    let f_sharp = F_SHARP;
    let value: u8 = f_sharp.into();
    let back_to_pitch_class: PitchClass = value.into();

    println!("   F# as u8: {}", value);
    println!("   {} back to PitchClass: {}", value, back_to_pitch_class);

    // Enharmonic equivalents
    println!("\n9. Enharmonic equivalents:");
    println!("   C# == Db: {}", C_SHARP == D_FLAT);
    println!("   F# == Gb: {}", F_SHARP == G_FLAT);
    println!("   A# == Bb: {}", A_SHARP == B_FLAT);
}
