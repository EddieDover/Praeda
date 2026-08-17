use praeda::*;
use std::collections::HashSet;

/// Helper to build a generator with enough variety that random choices are observable.
fn create_seeded_test_generator() -> PraedaGenerator {
    let mut generator = PraedaGenerator::new();

    generator.set_quality_data("common", 100);
    generator.set_quality_data("uncommon", 60);
    generator.set_quality_data("rare", 30);
    generator.set_quality_data("legendary", 5);

    generator.set_item_type("weapon", 3);
    generator.set_item_type("armor", 2);

    generator.set_item_subtype("weapon", "sword", 3);
    generator.set_item_subtype("weapon", "axe", 2);
    generator.set_item_subtype("weapon", "bow", 1);
    generator.set_item_subtype("armor", "head", 2);
    generator.set_item_subtype("armor", "chest", 1);

    generator.set_item(
        "weapon",
        "sword",
        vec!["longsword", "shortsword", "rapier", "sabre"],
    );
    generator.set_item("weapon", "axe", vec!["battleaxe", "hatchet", "cleaver"]);
    generator.set_item("weapon", "bow", vec!["shortbow", "longbow"]);
    generator.set_item("armor", "head", vec!["helm", "crown", "cap"]);
    generator.set_item("armor", "chest", vec!["breastplate", "hauberk"]);

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new("damage", 10.0, 1.0, 20.0, true),
    );
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new("crit_chance", 0.5, 0.0, 5.0, false),
    );
    generator.set_attribute(
        "armor",
        "",
        ItemAttribute::new("defense", 5.0, 1.0, 10.0, true),
    );
    generator.set_attribute(
        "",
        "",
        ItemAttribute::new("level_requirement", 1.0, 0.0, 0.0, true),
    );

    generator.set_affix_attribute(
        "weapon",
        "",
        true,
        "sharp",
        ItemAttribute::new("damage", 5.0, 0.0, 0.0, false),
    );
    generator.set_affix_attribute(
        "weapon",
        "",
        true,
        "heavy",
        ItemAttribute::new("damage", 8.0, 0.0, 0.0, false),
    );
    generator.set_affix_attribute(
        "weapon",
        "",
        false,
        "of flame",
        ItemAttribute::new("fire_damage", 3.0, 0.0, 0.0, false),
    );
    generator.set_affix_attribute(
        "armor",
        "",
        true,
        "sturdy",
        ItemAttribute::new("defense", 4.0, 0.0, 0.0, false),
    );

    generator
}

fn test_options(number_of_items: u32) -> GeneratorOptions {
    GeneratorOptions {
        number_of_items,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.5,
        linear: true,
        scaling_factor: 1.5,
    }
}

/// Full identity of an item, used to compare items across runs.
fn fingerprint(item: &Item) -> String {
    let mut attributes: Vec<String> = item
        .get_attributes()
        .iter()
        .map(|(name, attr)| format!("{}={:?}", name, attr.initial_value))
        .collect();
    attributes.sort();

    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        item.get_name(),
        item.get_quality(),
        item.get_type(),
        item.get_subtype(),
        item.get_prefix().get_name(),
        item.get_suffix().get_name(),
        attributes.join(","),
    )
}

#[test]
fn same_seed_produces_identical_items() {
    let options = test_options(8);

    let mut first_generator = create_seeded_test_generator();
    let mut second_generator = create_seeded_test_generator();

    let first = first_generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "delve", 12345)
        .expect("seeded generation should succeed");
    let second = second_generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "delve", 12345)
        .expect("seeded generation should succeed");

    assert_eq!(first.len(), 8);
    assert_eq!(first, second, "same seed must produce identical items");

    for (a, b) in first.iter().zip(second.iter()) {
        assert_eq!(fingerprint(a), fingerprint(b));
    }
}

#[test]
fn different_seeds_produce_different_items() {
    let options = test_options(4);

    let mut generator = create_seeded_test_generator();
    let baseline: Vec<String> = generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "baseline", 0)
        .expect("seeded generation should succeed")
        .iter()
        .map(fingerprint)
        .collect();

    let mut differing = 0;
    for seed in 1..=20u64 {
        let batch: Vec<String> = generator
            .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "candidate", seed)
            .expect("seeded generation should succeed")
            .iter()
            .map(fingerprint)
            .collect();

        if batch != baseline {
            differing += 1;
        }
    }

    assert!(
        differing >= 18,
        "expected at least 18 of 20 seeds to differ from seed 0, got {}",
        differing
    );
}

#[test]
fn batch_items_are_not_identical() {
    let options = test_options(10);

    let mut generator = create_seeded_test_generator();
    let items = generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "batch", 777)
        .expect("seeded generation should succeed");

    assert_eq!(items.len(), 10);

    let distinct: HashSet<String> = items.iter().map(fingerprint).collect();
    assert!(
        distinct.len() > 1,
        "a seeded batch must advance the stream between items, got {} distinct of 10",
        distinct.len()
    );
}

#[test]
fn seeded_is_independent_of_call_order() {
    let options = test_options(5);

    let mut generator = create_seeded_test_generator();
    let expected: Vec<String> = generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "reference", 999)
        .expect("seeded generation should succeed")
        .iter()
        .map(fingerprint)
        .collect();

    let mut interleaved = create_seeded_test_generator();
    for _ in 0..3 {
        let _ = interleaved
            .generate_loot(&options, &GeneratorOverrides::empty(), "noise")
            .expect("unseeded generation should succeed");

        let seeded: Vec<String> = interleaved
            .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "seeded", 999)
            .expect("seeded generation should succeed")
            .iter()
            .map(fingerprint)
            .collect();

        assert_eq!(
            seeded, expected,
            "seeded results must not depend on surrounding unseeded calls"
        );
    }
}

#[test]
fn attribute_values_are_reproducible() {
    let options = test_options(6);

    let mut first_generator = create_seeded_test_generator();
    let mut second_generator = create_seeded_test_generator();

    let first = first_generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "attrs", 424242)
        .expect("seeded generation should succeed");
    let second = second_generator
        .generate_loot_seeded(&options, &GeneratorOverrides::empty(), "attrs", 424242)
        .expect("seeded generation should succeed");

    let mut compared = 0;
    for (a, b) in first.iter().zip(second.iter()) {
        assert_eq!(
            a.get_attributes().len(),
            b.get_attributes().len(),
            "items must carry the same attribute set"
        );

        for (name, attr) in a.get_attributes() {
            let other = b
                .get_attribute(name)
                .unwrap_or_else(|| panic!("attribute {} missing from second run", name));

            assert!(
                attr.initial_value == other.initial_value,
                "attribute {} must be exactly equal, got {} and {}",
                name,
                attr.initial_value,
                other.initial_value
            );
            compared += 1;
        }
    }

    assert!(compared > 0, "expected some attributes to compare");
}

#[test]
fn seeded_json_is_reproducible() {
    let options = test_options(6);

    let mut first_generator = create_seeded_test_generator();
    let mut second_generator = create_seeded_test_generator();

    let first = first_generator
        .generate_loot_seeded_json(&options, &GeneratorOverrides::empty(), "json", 31337)
        .expect("seeded generation should succeed");
    let second = second_generator
        .generate_loot_seeded_json(&options, &GeneratorOverrides::empty(), "json", 31337)
        .expect("seeded generation should succeed");

    assert_eq!(first, second, "seeded JSON must be byte-identical");
}

#[test]
fn unseeded_api_still_works() {
    let options = test_options(30);

    let mut generator = create_seeded_test_generator();

    let items = generator
        .generate_loot(&options, &GeneratorOverrides::empty(), "unseeded")
        .expect("unseeded generation should succeed");

    assert_eq!(items.len(), 30);
    for item in &items {
        assert!(!item.get_name().is_empty(), "item should have a name");
        assert!(!item.get_quality().is_empty(), "item should have a quality");
        assert!(!item.get_type().is_empty(), "item should have a type");
        assert!(
            item.get_attribute("level").is_some(),
            "item should have a level attribute"
        );
    }

    let distinct: HashSet<String> = items.iter().map(fingerprint).collect();
    assert!(
        distinct.len() > 1,
        "unseeded generation should still vary across a large batch"
    );

    assert_eq!(generator.get_loot("unseeded").len(), 30);
}
