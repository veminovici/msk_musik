//! Simple example demonstrating FormulaDegree usage in musik_std.
//!
//! This example shows basic FormulaDegree usage for extended harmony in jazz chords.

use musik_std::FormulaDegree;

fn main() {
    println!("🎼 FormulaDegree Demo (now in musik_std)");
    println!("=========================================\n");

    // Basic chord tones
    println!("Basic chord tones:");
    let chord_tones = [
        FormulaDegree::natural(1), // Root
        FormulaDegree::natural(3), // Major 3rd
        FormulaDegree::natural(5), // Perfect 5th
        FormulaDegree::natural(7), // Major 7th
    ];

    for degree in &chord_tones {
        println!(
            "  {}: {} semitones",
            degree.symbol(),
            degree.to_semitone_offset().unwrap()
        );
    }

    // Extended harmony
    println!("\nExtended harmony:");
    let extensions = [
        FormulaDegree::natural(9),  // 9th
        FormulaDegree::natural(11), // 11th
        FormulaDegree::natural(13), // 13th
    ];

    for degree in &extensions {
        println!(
            "  {}: {} semitones (extended: {})",
            degree.symbol(),
            degree.to_semitone_offset().unwrap(),
            degree.is_extended()
        );
    }

    // Altered tensions
    println!("\nAltered tensions:");
    let alterations = [
        FormulaDegree::flat(9),   // ♭9
        FormulaDegree::sharp(9),  // ♯9
        FormulaDegree::sharp(11), // ♯11
        FormulaDegree::flat(13),  // ♭13
    ];

    for degree in &alterations {
        println!(
            "  {}: {} semitones (tension: {})",
            degree.symbol(),
            degree.to_semitone_offset().unwrap(),
            degree.is_tension()
        );
    }

    println!("\n✅ FormulaDegree successfully moved to musik_std!");
}
