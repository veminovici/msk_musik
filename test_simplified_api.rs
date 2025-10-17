// Test to verify the simplified API with DegreeAlteration::None

use musik_std::{ChordFormula, DegreeAlteration};

fn main() {
    println!("Testing simplified ChordFormula API...");

    // Test creating a chord with the new API
    let major_chord = ChordFormula::empty()
        .with_degree(1, DegreeAlteration::None)  // Root (natural)
        .with_degree(3, DegreeAlteration::None)  // Major 3rd (natural)
        .with_degree(5, DegreeAlteration::None); // Perfect 5th (natural)

    // Test creating a minor chord
    let minor_chord = ChordFormula::empty()
        .with_degree(1, DegreeAlteration::None)  // Root (natural)
        .with_degree(3, DegreeAlteration::Flat)  // Minor 3rd (flat)
        .with_degree(5, DegreeAlteration::None); // Perfect 5th (natural)

    // Test checking degrees
    assert!(major_chord.has_degree(1, DegreeAlteration::None));
    assert!(major_chord.has_degree(3, DegreeAlteration::None));
    assert!(major_chord.has_degree(5, DegreeAlteration::None));
    assert!(!major_chord.has_degree(3, DegreeAlteration::Flat));

    assert!(minor_chord.has_degree(1, DegreeAlteration::None));
    assert!(minor_chord.has_degree(3, DegreeAlteration::Flat));
    assert!(minor_chord.has_degree(5, DegreeAlteration::None));
    assert!(!minor_chord.has_degree(3, DegreeAlteration::None));

    // Test getting degree alterations
    assert_eq!(major_chord.get_degree_alteration(1), Some(DegreeAlteration::None));
    assert_eq!(major_chord.get_degree_alteration(3), Some(DegreeAlteration::None));
    assert_eq!(major_chord.get_degree_alteration(7), None); // Not present

    assert_eq!(minor_chord.get_degree_alteration(3), Some(DegreeAlteration::Flat));

    // Test degrees list
    let major_degrees = major_chord.degrees();
    assert!(major_degrees.contains(&(1, DegreeAlteration::None)));
    assert!(major_degrees.contains(&(3, DegreeAlteration::None)));
    assert!(major_degrees.contains(&(5, DegreeAlteration::None)));

    let minor_degrees = minor_chord.degrees();
    assert!(minor_degrees.contains(&(1, DegreeAlteration::None)));
    assert!(minor_degrees.contains(&(3, DegreeAlteration::Flat)));
    assert!(minor_degrees.contains(&(5, DegreeAlteration::None)));

    // Test built-in chord constructors
    let built_in_major = ChordFormula::major_triad();
    let built_in_minor = ChordFormula::minor_triad();

    assert_eq!(major_chord, built_in_major);
    assert_eq!(minor_chord, built_in_minor);

    println!("✅ All tests passed! API simplification successful.");
    println!("Major chord: {}", major_chord);
    println!("Minor chord: {}", minor_chord);
}