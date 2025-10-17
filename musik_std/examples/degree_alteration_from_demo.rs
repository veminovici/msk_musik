use musik_std::prelude::*;

fn main() {
    println!("🔄 DegreeAlteration From<u8> and From<DegreeAlteration> Demo");
    println!("===========================================================\n");

    // Converting from u8 to DegreeAlteration
    println!("📥 Converting u8 → DegreeAlteration:");
    let flat_from_2 = DegreeAlteration::from(2);
    let sharp_from_3 = DegreeAlteration::from(3);

    println!("  2 → {}", flat_from_2);
    println!("  3 → {}", sharp_from_3);

    // Using the Into trait (automatic conversion)
    println!("\n📥 Using Into trait (automatic conversion):");
    let flat_auto: DegreeAlteration = 2.into();
    let sharp_auto: DegreeAlteration = 3.into();

    println!("  2.into() → {}", flat_auto);
    println!("  3.into() → {}", sharp_auto);

    // Converting from DegreeAlteration to u8
    println!("\n📤 Converting DegreeAlteration → u8:");
    let flat_to_u8 = u8::from(DegreeAlteration::Flat);
    let sharp_to_u8 = u8::from(DegreeAlteration::Sharp);

    println!("  {} → {}", DegreeAlteration::Flat, flat_to_u8);
    println!("  {} → {}", DegreeAlteration::Sharp, sharp_to_u8);

    // Using the Into trait (automatic conversion)
    println!("\n📤 Using Into trait (automatic conversion):");
    let flat_value: u8 = DegreeAlteration::Flat.into();
    let sharp_value: u8 = DegreeAlteration::Sharp.into();

    println!("  {}.into() → {}", DegreeAlteration::Flat, flat_value);
    println!("  {}.into() → {}", DegreeAlteration::Sharp, sharp_value);

    // Round-trip conversions
    println!("\n🔄 Round-trip Conversions:");

    // u8 → DegreeAlteration → u8
    let original_values = [2u8, 3u8];
    for &original in &original_values {
        let alteration = DegreeAlteration::from(original);
        let back_to_u8: u8 = alteration.into();
        println!("  {} → {} → {} ✓", original, alteration, back_to_u8);
        assert_eq!(original, back_to_u8);
    }

    // DegreeAlteration → u8 → DegreeAlteration
    let alterations = [DegreeAlteration::Flat, DegreeAlteration::Sharp];
    for &alteration in &alterations {
        let as_u8: u8 = alteration.into();
        let back_to_alteration = DegreeAlteration::from(as_u8);
        println!("  {} → {} → {} ✓", alteration, as_u8, back_to_alteration);
        assert_eq!(alteration, back_to_alteration);
    }

    // Musical use case example
    println!("\n🎵 Musical Use Case - Scale Degree Encoding:");

    // Example: encoding scale degrees with alterations as single bytes
    let chord_tones = [
        ("Root", 1, DegreeAlteration::None),
        ("Minor 3rd", 3, DegreeAlteration::Flat),
        ("Perfect 5th", 5, DegreeAlteration::None),
        ("Minor 7th", 7, DegreeAlteration::Flat),
    ];

    println!("  Chord: C minor 7 (Cm7)");
    for (name, degree, alteration) in &chord_tones {
        match alteration {
            DegreeAlteration::None => {
                println!("    {} (degree {}): natural (no alteration)", name, degree);
            }
            alt => {
                let encoded: u8 = (*alt).into();
                println!(
                    "    {} (degree {}, alteration {}): encoded as {}",
                    name, degree, alt, encoded
                );
            }
        }
    }

    // Demonstrate error handling with invalid values
    println!("\n⚠️  Error Handling:");
    println!("  Valid values: 2 (♭), 3 (♯)");
    println!("  Invalid values like 0, 1, 4+ will panic!");

    // Show what would happen (commented out to avoid panic)
    // println!("  Trying DegreeAlteration::from(1) would panic with:");
    // println!("  \"Invalid u8 value for DegreeAlteration: 1. Expected 2 (Flat) or 3 (Sharp)\"");

    println!("\n✅ DegreeAlteration now supports bidirectional u8 conversion!");
    println!("   Perfect for serialization, encoding, and data interchange!");
}
