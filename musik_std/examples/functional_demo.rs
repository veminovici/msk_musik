//! Demonstration of functional programming approach in Semitone pitch class preservation testing.

use musik_std::Semitone;

fn main() {
    println!("🎼 Functional Programming Demo: Pitch Class Preservation\n");

    // Functional approach for upward octave shifts (Shr)
    println!("📈 Upward Octave Shifts (Shr operator):");
    println!("Using functional programming constructs...\n");

    let shr_results: Vec<_> = (0..12u8)
        .map(Semitone::new)
        .enumerate()
        .flat_map(|(pitch_class, original)| {
            (1..=3u8).map(move |octaves| {
                let shifted = original >> octaves;
                let shifted_value = u8::from(shifted);
                let preserved_pitch_class = shifted_value % 12;
                (
                    pitch_class as u8,
                    octaves,
                    shifted_value,
                    preserved_pitch_class,
                )
            })
        })
        .collect();

    // Display results using functional iteration
    shr_results
        .iter()
        .take(12) // Show first octave for demonstration
        .for_each(|(original_pc, octaves, final_value, preserved_pc)| {
            println!(
                "PC {} >> {} octaves = semitone {} (PC {} preserved: {})",
                original_pc,
                octaves,
                final_value,
                preserved_pc,
                original_pc == preserved_pc
            );
        });

    // Functional approach for downward octave shifts (Shl)
    println!("\n📉 Downward Octave Shifts (Shl operator):");
    println!("Using functional programming constructs...\n");

    let base_octave = 5u8;
    let shl_results: Vec<_> = (0..12u8)
        .map(|pc| (pc, Semitone::new(pc + (base_octave * 12))))
        .flat_map(|(pitch_class, original)| {
            (1..=3u8).map(move |octaves| {
                let shifted = original << octaves;
                let shifted_value = u8::from(shifted);
                let preserved_pitch_class = shifted_value % 12;
                (pitch_class, octaves, shifted_value, preserved_pitch_class)
            })
        })
        .collect();

    shl_results
        .iter()
        .take(12)
        .for_each(|(original_pc, octaves, final_value, preserved_pc)| {
            println!(
                "PC {} << {} octaves = semitone {} (PC {} preserved: {})",
                original_pc,
                octaves,
                final_value,
                preserved_pc,
                original_pc == preserved_pc
            );
        });

    // Functional verification: all pitch classes preserved
    println!("\n🔍 Functional Verification:");
    let all_preserved_shr = shr_results
        .iter()
        .all(|(original, _, _, preserved)| original == preserved);

    let all_preserved_shl = shl_results
        .iter()
        .all(|(original, _, _, preserved)| original == preserved);

    println!("✅ All Shr pitch classes preserved: {}", all_preserved_shr);
    println!("✅ All Shl pitch classes preserved: {}", all_preserved_shl);

    // Functional analysis: group by octave shift amount
    println!("\n📊 Analysis by Octave Shift Amount:");
    (1..=3u8).for_each(|target_octaves| {
        let shr_count = shr_results
            .iter()
            .filter(|(_, octaves, _, _)| *octaves == target_octaves)
            .count();

        let shl_count = shl_results
            .iter()
            .filter(|(_, octaves, _, _)| *octaves == target_octaves)
            .count();

        println!(
            "  {} octave shifts: {} Shr operations, {} Shl operations",
            target_octaves, shr_count, shl_count
        );
    });

    // Functional pattern: chain operations
    println!("\n🔗 Chained Functional Operations:");
    let chained_result = (0..12u8)
        .map(Semitone::new)
        .map(|s| s >> 2u8) // Up 2 octaves
        .map(|s| s << 1u8) // Down 1 octave
        .enumerate()
        .filter(|(pc, _)| *pc % 3 == 0) // Every 3rd pitch class
        .map(|(pc, semitone)| (pc, u8::from(semitone), u8::from(semitone) % 12))
        .collect::<Vec<_>>();

    chained_result
        .iter()
        .for_each(|(original_pc, final_value, preserved_pc)| {
            println!(
                "  Chain result: PC {} → semitone {} (PC {})",
                original_pc, final_value, preserved_pc
            );
        });

    println!("\n🎼 Functional programming demonstration completed!");
    println!("   Benefits: Immutable data flow, composable operations, expressive intent");
}
