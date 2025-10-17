use musik_std::prelude::*;

fn main() {
    println!("🔢 ChordFormula Bit Manipulation Deep Dive");
    println!("==========================================\n");

    // Understanding the bit layout
    println!("📊 Bit Layout (2 bits per degree):");
    println!("  0: Not in chord");
    println!("  1: Natural degree");
    println!("  2: Flat degree (♭)");
    println!("  3: Sharp degree (♯)");
    println!();

    println!("  Degree mapping:");
    for degree in 1..=15 {
        let bit_position = (degree - 1) * 2;
        println!(
            "    {}° degree: bits {}-{}",
            degree,
            bit_position,
            bit_position + 1
        );
    }

    // Simple chord analysis
    println!("\n🔬 Simple Chord Bit Analysis:");

    let major_triad = ChordFormula::major_triad();
    println!("  Major Triad (1-3-5): 0b{:032b}", major_triad.bits());

    // Break down the bits
    println!("    Bit breakdown:");
    for degree in 1..=7 {
        let shift = (degree - 1) * 2;
        let value = (major_triad.bits() >> shift) & 0b11;
        let meaning = match value {
            0 => "absent",
            1 => "natural",
            2 => "flat",
            3 => "sharp",
            _ => "unknown",
        };
        println!("      {}° degree: {} ({})", degree, value, meaning);
    }

    // Complex chord analysis
    println!("\n🎷 Complex Jazz Chord Analysis:");

    let dom13_sharp11 = ChordFormula::dominant_thirteenth_sharp_eleventh();
    println!("  Dom13♯11: {}", dom13_sharp11);
    println!("  Bits: 0b{:032b}", dom13_sharp11.bits());
    println!("  Hex: 0x{:08X}", dom13_sharp11.bits());

    println!("    Detailed breakdown:");
    for degree in 1..=15 {
        let shift = (degree - 1) * 2;
        let value = (dom13_sharp11.bits() >> shift) & 0b11;
        if value != 0 {
            let (meaning, symbol) = match value {
                1 => ("natural", ""),
                2 => ("flat", "♭"),
                3 => ("sharp", "♯"),
                _ => ("unknown", "?"),
            };
            println!(
                "      {}° degree: {} ({}{} - {})",
                degree, value, symbol, degree, meaning
            );
        }
    }

    // Memory efficiency demonstration
    println!("\n💾 Memory Efficiency:");
    println!(
        "  ChordFormula size: {} bytes",
        std::mem::size_of::<ChordFormula>()
    );
    println!("  u32 size: {} bytes", std::mem::size_of::<u32>());
    println!("  Can store up to 15 degrees with alterations in just 4 bytes!");

    // Compare different storage methods
    let degrees_vec = dom13_sharp11.degrees();
    println!(
        "  Vec<(u8, Option<DegreeAlteration>)> for same chord: {} bytes (on stack)",
        std::mem::size_of_val(&degrees_vec)
    );
    println!("  Plus heap allocation for {} elements", degrees_vec.len());

    // Bit manipulation operations
    println!("\n⚙️ Bit Manipulation Operations:");

    // Manual chord construction
    let mut bits = 0u32;

    // Add root (degree 1, natural)
    bits |= 1 << 0;
    println!("  After adding root: 0b{:032b}", bits);

    // Add flat 3rd (degree 3, flat)
    bits |= 2 << 4; // degree 3 is at bit position 4-5
    println!("  After adding ♭3rd: 0b{:032b}", bits);

    // Add perfect 5th (degree 5, natural)
    bits |= 1 << 8; // degree 5 is at bit position 8-9
    println!("  After adding 5th: 0b{:032b}", bits);

    // Add flat 7th (degree 7, flat)
    bits |= 2 << 12; // degree 7 is at bit position 12-13
    println!("  After adding ♭7th: 0b{:032b}", bits);

    let manual_minor7 = ChordFormula::new(bits);
    let builtin_minor7 = ChordFormula::minor_seventh();

    println!("  Manual construction: {}", manual_minor7);
    println!("  Built-in method: {}", builtin_minor7);
    println!("  Are they equal? {}", manual_minor7 == builtin_minor7);

    // Chord union operations
    println!("\n🔗 Bit Union Operations:");

    let triad = ChordFormula::major_triad();
    let seventh = ChordFormula::empty().with_degree(7, DegreeAlteration::None);
    let ninth = ChordFormula::empty().with_degree(9, DegreeAlteration::None);

    println!("  Major triad: {} (0b{:032b})", triad, triad.bits());
    println!("  Just 7th: {} (0b{:032b})", seventh, seventh.bits());
    println!("  Just 9th: {} (0b{:032b})", ninth, ninth.bits());

    let maj7 = triad.union(seventh);
    let maj9 = maj7.union(ninth);

    println!("  Union result (Maj7): {} (0b{:032b})", maj7, maj7.bits());
    println!("  Union result (Maj9): {} (0b{:032b})", maj9, maj9.bits());

    // Chord comparison and analysis
    println!("\n🔍 Chord Comparison:");

    let chords = [
        ("Major", ChordFormula::major_triad()),
        ("Minor", ChordFormula::minor_triad()),
        ("Dim", ChordFormula::diminished_triad()),
        ("Aug", ChordFormula::augmented_triad()),
        ("Dom7", ChordFormula::dominant_seventh()),
        ("Maj7", ChordFormula::major_seventh()),
    ];

    for (name, chord) in &chords {
        println!("  {}: {} (0x{:08X})", name, chord, chord.bits());
    }

    // Bitwise operations
    println!("\n🧮 Advanced Bitwise Operations:");

    // Check if any altered degrees exist
    let has_alterations = |chord: ChordFormula| {
        let bits = chord.bits();
        // Check all even bit positions (alterations are encoded as 2 or 3)
        for degree in 1..=15 {
            let shift = (degree - 1) * 2;
            let value = (bits >> shift) & 0b11;
            if value == 2 || value == 3 {
                // flat or sharp
                return true;
            }
        }
        false
    };

    println!(
        "  Major triad has alterations: {}",
        has_alterations(ChordFormula::major_triad())
    );
    println!(
        "  Minor triad has alterations: {}",
        has_alterations(ChordFormula::minor_triad())
    );
    println!(
        "  Dom7♯9 has alterations: {}",
        has_alterations(ChordFormula::dominant_seventh_sharp_ninth())
    );

    // Count degrees in chord
    let count_degrees = |chord: ChordFormula| {
        let bits = chord.bits();
        let mut count = 0;
        for degree in 1..=15 {
            let shift = (degree - 1) * 2;
            let value = (bits >> shift) & 0b11;
            if value != 0 {
                count += 1;
            }
        }
        count
    };

    println!(
        "  Triad degree count: {}",
        count_degrees(ChordFormula::major_triad())
    );
    println!(
        "  Dom13♯11 degree count: {}",
        count_degrees(ChordFormula::dominant_thirteenth_sharp_eleventh())
    );

    // Pattern matching on bit patterns
    println!("\n🎯 Pattern Matching:");

    let analyze_chord_type = |chord: ChordFormula| {
        let has_root = chord.has_degree(1, DegreeAlteration::None);
        let has_major_3rd = chord.has_degree(3, DegreeAlteration::None);
        let has_minor_3rd = chord.has_degree(3, DegreeAlteration::Flat);
        let has_5th = chord.has_degree(5, DegreeAlteration::None);
        let has_7th = chord.has_any_degree(7);

        match (has_root, has_major_3rd, has_minor_3rd, has_5th, has_7th) {
            (true, true, false, true, false) => "Major Triad",
            (true, false, true, true, false) => "Minor Triad",
            (true, true, false, true, true) => "Major-based 7th chord",
            (true, false, true, true, true) => "Minor-based 7th chord",
            _ => "Complex/Other chord",
        }
    };

    let test_chords = [
        ChordFormula::major_triad(),
        ChordFormula::minor_triad(),
        ChordFormula::major_seventh(),
        ChordFormula::minor_seventh(),
        ChordFormula::dominant_seventh(),
        ChordFormula::altered_dominant(),
    ];

    for chord in &test_chords {
        println!("  {}: {}", chord, analyze_chord_type(*chord));
    }

    println!("\n✅ ChordFormula provides powerful bit-level chord manipulation!");
}
