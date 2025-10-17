use musik_std::prelude::*;

fn main() {
    println!("🎼 ChordFormula Demo - Bit-Packed Chord Representations");
    println!("========================================================\n");

    // Basic triads
    println!("📝 Basic Triads:");
    let major = ChordFormula::major_triad();
    let minor = ChordFormula::minor_triad();
    let diminished = ChordFormula::diminished_triad();
    let augmented = ChordFormula::augmented_triad();

    println!("  Major Triad: {}", major);
    println!("  Minor Triad: {}", minor);
    println!("  Diminished Triad: {}", diminished);
    println!("  Augmented Triad: {}", augmented);

    // Suspended chords
    println!("\n🔄 Suspended Chords:");
    let sus2 = ChordFormula::sus2();
    let sus4 = ChordFormula::sus4();

    println!("  Sus2: {}", sus2);
    println!("  Sus4: {}", sus4);

    // Seventh chords
    println!("\n7️⃣ Seventh Chords:");
    let maj7 = ChordFormula::major_seventh();
    let min7 = ChordFormula::minor_seventh();
    let dom7 = ChordFormula::dominant_seventh();
    let min_maj7 = ChordFormula::minor_major_seventh();
    let half_dim7 = ChordFormula::half_diminished_seventh();
    let full_dim7 = ChordFormula::fully_diminished_seventh();

    println!("  Major 7th: {}", maj7);
    println!("  Minor 7th: {}", min7);
    println!("  Dominant 7th: {}", dom7);
    println!("  Minor Major 7th: {}", min_maj7);
    println!("  Half-Diminished 7th: {}", half_dim7);
    println!("  Fully Diminished 7th: {}", full_dim7);

    // Extended jazz chords (9ths)
    println!("\n🎷 Extended Jazz Chords (9ths):");
    let maj9 = ChordFormula::major_ninth();
    let min9 = ChordFormula::minor_ninth();
    let dom9 = ChordFormula::dominant_ninth();
    let dom7_flat9 = ChordFormula::dominant_seventh_flat_ninth();
    let dom7_sharp9 = ChordFormula::dominant_seventh_sharp_ninth();

    println!("  Major 9th: {}", maj9);
    println!("  Minor 9th: {}", min9);
    println!("  Dominant 9th: {}", dom9);
    println!("  Dominant 7♭9: {}", dom7_flat9);
    println!("  Dominant 7♯9: {}", dom7_sharp9);

    // Extended jazz chords (11ths)
    println!("\n🎺 Extended Jazz Chords (11ths):");
    let maj11 = ChordFormula::major_eleventh();
    let min11 = ChordFormula::minor_eleventh();
    let dom11 = ChordFormula::dominant_eleventh();
    let dom7_sharp11 = ChordFormula::dominant_seventh_sharp_eleventh();

    println!("  Major 11th: {}", maj11);
    println!("  Minor 11th: {}", min11);
    println!("  Dominant 11th: {}", dom11);
    println!("  Dominant 7♯11: {}", dom7_sharp11);

    // Extended jazz chords (13ths)
    println!("\n🎸 Extended Jazz Chords (13ths):");
    let maj13 = ChordFormula::major_thirteenth();
    let min13 = ChordFormula::minor_thirteenth();
    let dom13 = ChordFormula::dominant_thirteenth();
    let dom13_flat9 = ChordFormula::dominant_thirteenth_flat_ninth();
    let dom13_sharp11 = ChordFormula::dominant_thirteenth_sharp_eleventh();

    println!("  Major 13th: {}", maj13);
    println!("  Minor 13th: {}", min13);
    println!("  Dominant 13th: {}", dom13);
    println!("  Dominant 13♭9: {}", dom13_flat9);
    println!("  Dominant 13♯11: {}", dom13_sharp11);

    // Add chords
    println!("\n➕ Add Chords:");
    let add9 = ChordFormula::add_ninth();
    let min_add9 = ChordFormula::minor_add_ninth();
    let sixth = ChordFormula::sixth();
    let min6 = ChordFormula::minor_sixth();
    let six_nine = ChordFormula::six_nine();
    let min_six_nine = ChordFormula::minor_six_nine();

    println!("  Add 9: {}", add9);
    println!("  Minor Add 9: {}", min_add9);
    println!("  6th: {}", sixth);
    println!("  Minor 6th: {}", min6);
    println!("  6/9: {}", six_nine);
    println!("  Minor 6/9: {}", min_six_nine);

    // Altered dominant chords
    println!("\n🔀 Altered Dominant Chords:");
    let altered = ChordFormula::altered_dominant();
    let dom7_sharp5 = ChordFormula::dominant_seventh_sharp_fifth();
    let dom7_flat5 = ChordFormula::dominant_seventh_flat_fifth();

    println!("  Altered Dominant: {}", altered);
    println!("  Dominant 7♯5: {}", dom7_sharp5);
    println!("  Dominant 7♭5: {}", dom7_flat5);

    // Detailed analysis
    println!("\n🔍 Detailed Chord Analysis:");

    println!("\n  C Minor 7 Analysis:");
    let cm7 = ChordFormula::minor_seventh();
    let degrees = cm7.degrees();
    for (degree, alteration) in &degrees {
        let alteration_str = match alteration {
            DegreeAlteration::None => "natural".to_string(),
            DegreeAlteration::Flat => "flat".to_string(),
            DegreeAlteration::Sharp => "sharp".to_string(),
        };
        println!("    {}° degree: {}", degree, alteration_str);
    }

    println!("\n  Dom7♯9 Breakdown:");
    let dom7_s9 = ChordFormula::dominant_seventh_sharp_ninth();
    println!(
        "    Has root: {}",
        dom7_s9.has_degree(1, DegreeAlteration::None)
    );
    println!(
        "    Has major 3rd: {}",
        dom7_s9.has_degree(3, DegreeAlteration::None)
    );
    println!(
        "    Has perfect 5th: {}",
        dom7_s9.has_degree(5, DegreeAlteration::None)
    );
    println!(
        "    Has flat 7th: {}",
        dom7_s9.has_degree(7, DegreeAlteration::Flat)
    );
    println!(
        "    Has sharp 9th: {}",
        dom7_s9.has_degree(9, DegreeAlteration::Sharp)
    );
    println!(
        "    Has natural 9th: {}",
        dom7_s9.has_degree(9, DegreeAlteration::None)
    );

    // Bit manipulation examples
    println!("\n🔢 Bit Manipulation:");
    println!("  Major triad bits: 0b{:032b}", major.bits());
    println!("  Minor triad bits: 0b{:032b}", minor.bits());

    // Custom chord construction
    println!("\n🏗️ Custom Chord Construction:");
    let custom = ChordFormula::empty()
        .with_degree(1, DegreeAlteration::None) // Root
        .with_degree(3, DegreeAlteration::None) // Major 3rd
        .with_degree(5, DegreeAlteration::Sharp) // Augmented 5th
        .with_degree(7, DegreeAlteration::None) // Major 7th
        .with_degree(9, DegreeAlteration::Sharp); // Sharp 9th

    println!("  Custom chord (Maj7♯5♯9): {}", custom);

    // Chord combinations using union
    println!("\n🔗 Chord Combinations:");
    let base_triad = ChordFormula::major_triad();
    let add_seventh = ChordFormula::empty().with_degree(7, DegreeAlteration::None);
    let add_ninth = ChordFormula::empty().with_degree(9, DegreeAlteration::None);

    let maj7_from_union = base_triad.union(add_seventh);
    let maj9_from_union = maj7_from_union.union(add_ninth);

    println!("  Major triad: {}", base_triad);
    println!("  + Major 7th: {}", maj7_from_union);
    println!("  + Major 9th: {}", maj9_from_union);

    // Jazz chord progressions
    println!("\n🎵 Jazz Chord Progressions:");
    let chord_progression = [
        ("Cm7", ChordFormula::minor_seventh()),
        ("F7", ChordFormula::dominant_seventh()),
        ("BbMaj7", ChordFormula::major_seventh()),
        ("EbMaj7", ChordFormula::major_seventh()),
        ("Am7b5", ChordFormula::half_diminished_seventh()),
        ("D7alt", ChordFormula::altered_dominant()),
        ("Gm", ChordFormula::minor_triad()),
    ];

    println!("  ii-V-I-VI-ii-V-i progression in Bb major/G minor:");
    for (name, formula) in &chord_progression {
        println!("    {}: {}", name, formula);
    }

    // Memory efficiency demonstration
    println!("\n💾 Memory Efficiency:");
    println!(
        "  ChordFormula size: {} bytes",
        std::mem::size_of::<ChordFormula>()
    );
    println!("  Can represent up to 15 degrees with alterations in just 4 bytes!");

    let complex_jazz_chord = ChordFormula::dominant_thirteenth_sharp_eleventh();
    println!(
        "  Complex chord (Dom13♯11) fits in: {} bytes",
        std::mem::size_of_val(&complex_jazz_chord)
    );
    println!(
        "  Degrees in Dom13♯11: {}",
        complex_jazz_chord.degrees().len()
    );

    println!("\n✅ ChordFormula provides efficient, comprehensive chord representation!");
}
