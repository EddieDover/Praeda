use praeda::*;
use std::collections::HashMap; // Used in test_item_struct

/// Helper to create a basic generator with standard configuration
fn create_test_generator() -> PraedaGenerator {
    let mut generator = PraedaGenerator::new();

    // Quality tiers
    generator.set_quality_data("common", 100);
    generator.set_quality_data("uncommon", 60);
    generator.set_quality_data("rare", 30);

    // Item types
    generator.set_item_type("weapon", 1);
    generator.set_item_type("armor", 1);

    // Subtypes
    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_item_subtype("weapon", "axe", 1);
    generator.set_item_subtype("armor", "head", 1);

    // Attributes
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_attribute(
        "armor",
        "",
        ItemAttribute::new(
            "defense",
            5.0,
            1.0,
            10.0,
            true,
        ),
    );

    // Item names
    generator.set_item(
        "weapon",
        "sword",
        vec!["longsword", "shortsword"],
    );
    generator.set_item(
        "weapon",
        "axe",
        vec!["battleaxe"],
    );
    generator.set_item(
        "armor",
        "head",
        vec!["helm", "crown"],
    );

    // Affixes
    generator.set_affix_attribute(
        "weapon",
        "",
        true,
        "sharp",
        ItemAttribute::new(
            "damage",
            5.0,
            0.0,
            0.0,
            false,
        ),
    );

    generator.set_affix_attribute(
        "weapon",
        "",
        false,
        "of fire",
        ItemAttribute::new(
            "damage",
            3.0,
            0.0,
            0.0,
            false,
        ),
    );

    generator
}

#[test]
fn test_generator_creation() {
    let generator = PraedaGenerator::new();
    assert_eq!(generator.get_quality_data().len(), 0);
    assert_eq!(generator.get_item_types().len(), 0);
}

#[test]
fn test_set_quality_data() {
    let mut generator = PraedaGenerator::new();
    generator.set_quality_data("common", 100);
    generator.set_quality_data("rare", 10);

    assert!(generator.has_quality("common"));
    assert!(generator.has_quality("rare"));
    assert!(!generator.has_quality("epic"));
}

#[test]
fn test_set_item_type() {
    let mut generator = PraedaGenerator::new();
    generator.set_item_type("weapon", 50);
    generator.set_item_type("armor", 50);

    assert!(generator.has_item_type("weapon"));
    assert!(generator.has_item_type("armor"));
    assert!(!generator.has_item_type("shield"));
}

#[test]
fn test_set_item_subtype() {
    let mut generator = PraedaGenerator::new();
    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 50);

    assert!(generator.has_item_subtype("weapon", "sword"));
    assert!(!generator.has_item_subtype("weapon", "bow"));
}

#[test]
fn test_empty_string_overrides_always_match() {
    let generator = PraedaGenerator::new();
    assert!(generator.has_quality(""));
    assert!(generator.has_item_type(""));
    assert!(generator.has_item_subtype("", ""));
}

#[test]
fn test_single_item_generation() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.5,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "test")?;

    assert_eq!(items.len(), 1);
    let item = &items[0];

    // Verify item has required fields
    assert!(!item.get_name().is_empty());
    assert!(!item.get_quality().is_empty());
    assert!(!item.get_type().is_empty());
    assert!(item.has_attribute("level"));

    Ok(())
}

#[test]
fn test_multiple_items_generation() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "bulk")?;

    assert_eq!(items.len(), 100);

    // Verify all items are valid
    for item in items {
        assert!(!item.get_name().is_empty());
        assert!(!item.get_quality().is_empty());
    }

    Ok(())
}

#[test]
fn test_quality_override() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "rare",
        "",
        "",
    );

    let items = generator.generate_loot(&options, &overrides, "quality_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_quality(), "rare");

    Ok(())
}

#[test]
fn test_type_override() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "",
        "weapon",
        "",
    );

    let items = generator.generate_loot(&options, &overrides, "type_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_type(), "weapon");

    Ok(())
}

#[test]
fn test_subtype_override() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "",
        "weapon",
        "sword",
    );

    let items = generator.generate_loot(&options, &overrides, "subtype_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_subtype(), "sword");

    Ok(())
}

#[test]
fn test_linear_vs_exponential_scaling() -> Result<()> {
    let mut gen1 = create_test_generator();
    let mut gen2 = create_test_generator();

    let linear_opts = GeneratorOptions {
        number_of_items: 10,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 1.0, // Set to 1.0 to ensure optional attributes are applied
        linear: true,
        scaling_factor: 1.5,
    };

    let exp_opts = GeneratorOptions {
        number_of_items: 10,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 1.0, // Set to 1.0 to ensure optional attributes are applied
        linear: false,
        scaling_factor: 1.5,
    };

    let linear_items = gen1.generate_loot(&linear_opts, &GeneratorOverrides::empty(), "linear")?;
    let exp_items = gen2.generate_loot(&exp_opts, &GeneratorOverrides::empty(), "exp")?;

    // Both should generate items
    assert_eq!(linear_items.len(), 10);
    assert_eq!(exp_items.len(), 10);

    // Both should have level attribute (required)
    assert!(linear_items[0].has_attribute("level"));
    assert!(exp_items[0].has_attribute("level"));

    Ok(())
}

#[test]
fn test_json_serialization() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "json_test")?;
    let json_str = serde_json::to_string(&items)?;

    // Should be valid JSON
    let _: Vec<Item> = serde_json::from_str(&json_str)?;

    Ok(())
}


#[test]
fn test_affixes_applied_to_items() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 50,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 1.0, // Always apply affixes
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "affix_test")?;

    let mut has_prefix = false;
    let mut has_suffix = false;

    for item in items {
        if !item.get_prefix().get_name().is_empty() {
            has_prefix = true;
        }
        if !item.get_suffix().get_name().is_empty() {
            has_suffix = true;
        }
    }

    // With high affix chance and enough items, should see some affixes
    assert!(has_prefix || has_suffix);

    Ok(())
}

#[test]
fn test_get_prefixes_and_suffixes() {
    let generator = create_test_generator();

    let prefixes = generator.get_prefixes("weapon", "");
    let suffixes = generator.get_suffixes("weapon", "");

    assert_eq!(prefixes.len(), 1);
    assert_eq!(suffixes.len(), 1);
    assert_eq!(prefixes[0].get_name(), "sharp");
    assert_eq!(suffixes[0].get_name(), "of fire");
}

#[test]
fn test_item_attribute_struct() {
    let mut attr = ItemAttribute::new(
        "health",
        100.0,
        0.0,
        200.0,
        true,
    );

    assert_eq!(attr.get_name(), "health");
    assert_eq!(attr.get_initial_value(), 100.0);
    assert!(attr.get_required());

    attr.set_initial_value(150.0);
    assert_eq!(attr.get_initial_value(), 150.0);
}

#[test]
fn test_item_struct() {
    let item = Item::new(
        "sword",
        "rare",
        "weapon",
        "sword",
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    assert_eq!(item.get_name(), "sword");
    assert_eq!(item.get_quality(), "rare");
    assert_eq!(item.get_type(), "weapon");
    assert_eq!(item.get_subtype(), "sword");
}

#[test]
fn test_affix_struct() {
    let attr = ItemAttribute::new(
        "damage",
        10.0,
        0.0,
        0.0,
        false,
    );

    let affix = Affix::new("sharp", vec![attr]);

    assert_eq!(affix.get_name(), "sharp");
    assert_eq!(affix.get_attributes().len(), 1);
    assert_eq!(affix.get_attributes()[0].get_name(), "damage");
}

#[test]
fn test_generator_options_defaults() {
    let opts = GeneratorOptions::default();

    assert_eq!(opts.number_of_items, 1);
    assert_eq!(opts.base_level, 1.0);
    assert_eq!(opts.level_variance, 1.0);
    assert_eq!(opts.affix_chance, 0.25);
    assert!(opts.linear);
    assert_eq!(opts.scaling_factor, 1.0);
}

#[test]
fn test_generator_overrides_empty() {
    let overrides = GeneratorOverrides::empty();

    assert_eq!(overrides.get_quality_override(), "");
    assert_eq!(overrides.get_type_override(), "");
    assert_eq!(overrides.get_subtype_override(), "");
}

#[test]
fn test_loot_retrieval() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions::default();
    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "retrieval_test")?;

    let retrieved = generator.get_loot("retrieval_test");
    assert_eq!(retrieved.len(), items.len());

    let json = generator.get_loot_json("retrieval_test")?;
    assert!(!json.is_empty());

    Ok(())
}

#[test]
fn test_nonexistent_loot_retrieval() {
    let generator = PraedaGenerator::new();

    let items = generator.get_loot("nonexistent");
    assert_eq!(items.len(), 0);
}

#[test]
fn test_has_attribute() {
    let generator = create_test_generator();

    assert!(generator.has_attribute("weapon", "", "damage"));
    assert!(generator.has_attribute("armor", "", "defense"));
    assert!(!generator.has_attribute("weapon", "", "nonexistent"));
}

#[test]
fn test_empty_quality_data_handles_gracefully() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Should fail gracefully when trying to generate with no qualities
    let options = GeneratorOptions::default();
    let result = generator.generate_loot(&options, &GeneratorOverrides::empty(), "empty");

    // It should fail since there's no quality data
    assert!(result.is_err());

    Ok(())
}


#[test]
fn test_quality_distribution() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup with very unbalanced weights
    generator.set_quality_data("common", 1000);
    generator.set_quality_data("rare", 1);

    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );
    generator.set_item(
        "weapon",
        "sword",
        vec!["sword"],
    );

    let options = GeneratorOptions {
        number_of_items: 100,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "distribution")?;

    let common_count = items.iter().filter(|i| i.get_quality() == "common").count();
    let rare_count = items.iter().filter(|i| i.get_quality() == "rare").count();

    // Most items should be common (1000:1 ratio)
    assert!(common_count > rare_count * 5);

    Ok(())
}

#[test]
fn test_quality_weights_respect_ratios() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup with balanced weights: 50% common, 30% uncommon, 20% rare
    generator.set_quality_data("common", 50);
    generator.set_quality_data("uncommon", 30);
    generator.set_quality_data("rare", 20);

    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );
    generator.set_item(
        "weapon",
        "sword",
        vec!["sword"],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "weight_test")?;

    let common_count = items.iter().filter(|i| i.get_quality() == "common").count() as f64;
    let uncommon_count = items.iter().filter(|i| i.get_quality() == "uncommon").count() as f64;
    let rare_count = items.iter().filter(|i| i.get_quality() == "rare").count() as f64;
    let total = items.len() as f64;

    let common_pct = common_count / total;
    let uncommon_pct = uncommon_count / total;
    let rare_pct = rare_count / total;

    // Allow 10% deviation from expected percentages
    assert!((common_pct - 0.50).abs() < 0.10, "common: expected 50%, got {}", common_pct * 100.0);
    assert!((uncommon_pct - 0.30).abs() < 0.10, "uncommon: expected 30%, got {}", uncommon_pct * 100.0);
    assert!((rare_pct - 0.20).abs() < 0.10, "rare: expected 20%, got {}", rare_pct * 100.0);

    Ok(())
}

#[test]
fn test_item_type_weights() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup with 2:1 weapon to armor ratio
    generator.set_quality_data("common", 100);
    generator.set_item_type("weapon", 2);
    generator.set_item_type("armor", 1);

    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_item_subtype("armor", "head", 1);

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_item(
        "weapon",
        "sword",
        vec!["sword"],
    );
    generator.set_item(
        "armor",
        "head",
        vec!["helm"],
    );

    let options = GeneratorOptions {
        number_of_items: 300,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "type_weights")?;

    let weapon_count = items.iter().filter(|i| i.get_type() == "weapon").count() as f64;
    let armor_count = items.iter().filter(|i| i.get_type() == "armor").count() as f64;
    let total = items.len() as f64;

    let weapon_pct = weapon_count / total;
    let armor_pct = armor_count / total;

    // Expect roughly 2:1 ratio (66% weapons, 33% armor)
    // Allow 15% deviation
    assert!(weapon_pct > 0.51 && weapon_pct < 0.81, "weapons: expected ~66%, got {}", weapon_pct * 100.0);
    assert!(armor_pct > 0.19 && armor_pct < 0.49, "armor: expected ~33%, got {}", armor_pct * 100.0);

    Ok(())
}

#[test]
fn test_subtype_weights() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup with 3:1 ratio of one-handed to two-handed
    generator.set_quality_data("common", 100);
    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "one-handed", 3);
    generator.set_item_subtype("weapon", "two-handed", 1);

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_item(
        "weapon",
        "one-handed",
        vec!["sword"],
    );
    generator.set_item(
        "weapon",
        "two-handed",
        vec!["claymore"],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "subtype_weights")?;

    let one_handed_count = items.iter().filter(|i| i.get_subtype() == "one-handed").count() as f64;
    let two_handed_count = items.iter().filter(|i| i.get_subtype() == "two-handed").count() as f64;
    let total = items.len() as f64;

    let one_handed_pct = one_handed_count / total;
    let two_handed_pct = two_handed_count / total;

    // Expect roughly 3:1 ratio (75% one-handed, 25% two-handed)
    // Allow 10% deviation (with 1000 items, variance should be small)
    assert!(one_handed_pct > 0.65 && one_handed_pct < 0.85, "one-handed: expected ~75%, got {}", one_handed_pct * 100.0);
    assert!(two_handed_pct > 0.15 && two_handed_pct < 0.35, "two-handed: expected ~25%, got {}", two_handed_pct * 100.0);

    Ok(())
}

/// Test 1: High variance scaling with exponential growth
/// Simulates a game with varied item levels (1-100) and exponential attribute scaling
#[test]
fn test_exponential_scaling_variance() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup qualities with heavy weights toward common
    generator.set_quality_data("common", 1000);
    generator.set_quality_data("uncommon", 300);
    generator.set_quality_data("rare", 100);
    generator.set_quality_data("epic", 20);
    generator.set_quality_data("legendary", 1);

    // Multiple item types with varied weights
    generator.set_item_type("weapon", 5);
    generator.set_item_type("armor", 4);
    generator.set_item_type("accessory", 1);

    // Weapon subtypes
    generator.set_item_subtype("weapon", "sword", 3);
    generator.set_item_subtype("weapon", "axe", 2);
    generator.set_item_subtype("weapon", "bow", 1);

    // Armor subtypes
    generator.set_item_subtype("armor", "chest", 2);
    generator.set_item_subtype("armor", "legs", 2);
    generator.set_item_subtype("armor", "head", 1);

    // Accessory subtypes
    generator.set_item_subtype("accessory", "ring", 1);

    // Set attributes with exponential scaling
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "attack",
            50.0,
            10.0,
            100.0,
            true,
        ),
    );

    generator.set_attribute(
        "armor",
        "",
        ItemAttribute::new(
            "defense",
            30.0,
            5.0,
            60.0,
            true,
        ),
    );

    generator.set_attribute(
        "accessory",
        "",
        ItemAttribute::new(
            "magic",
            20.0,
            5.0,
            50.0,
            true,
        ),
    );

    // Set item names
    generator.set_item(
        "weapon",
        "sword",
        vec!["longsword", "shortsword", "claymore"],
    );
    generator.set_item(
        "weapon",
        "axe",
        vec!["war_axe", "hand_axe"],
    );
    generator.set_item(
        "weapon",
        "bow",
        vec!["longbow"],
    );
    generator.set_item(
        "armor",
        "chest",
        vec!["plate_chest", "leather_chest"],
    );
    generator.set_item(
        "armor",
        "legs",
        vec!["plate_legs", "leather_legs"],
    );
    generator.set_item(
        "armor",
        "head",
        vec!["helmet"],
    );
    generator.set_item(
        "accessory",
        "ring",
        vec!["gold_ring", "silver_ring"],
    );

    // Generate with high variance and exponential scaling
    let options = GeneratorOptions {
        number_of_items: 500,
        base_level: 50.0,
        level_variance: 40.0,
        affix_chance: 0.3,
        linear: false, // Exponential scaling
        scaling_factor: 1.5,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "exp_scaling")?;

    // Verify items were generated
    assert_eq!(items.len(), 500);

    // Verify all items have expected types
    let valid_types: Vec<&str> = vec!["weapon", "armor", "accessory"];
    for item in &items {
        assert!(valid_types.contains(&item.get_type()));
    }

    // Verify quality distribution roughly matches weights (1421 total weight)
    let common_pct = items.iter().filter(|i| i.get_quality() == "common").count() as f64 / 500.0;
    assert!(common_pct > 0.60 && common_pct < 0.75, "common expected ~70%, got {}", common_pct * 100.0);

    Ok(())
}

/// Test 2: Minimal setup - single type, single subtype, single quality
/// Verifies library works with minimal configuration
#[test]
fn test_minimal_single_item_generation() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Absolute minimum setup
    generator.set_quality_data("normal", 1);
    generator.set_item_type("tool", 1);
    generator.set_item_subtype("tool", "pickaxe", 1);

    generator.set_attribute(
        "tool",
        "",
        ItemAttribute::new(
            "durability",
            50.0,
            10.0,
            100.0,
            true,
        ),
    );

    generator.set_item(
        "tool",
        "pickaxe",
        vec!["pickaxe"],
    );

    let options = GeneratorOptions {
        number_of_items: 10,
        base_level: 1.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "minimal")?;

    // All items should be identical (same quality, type, subtype, name)
    assert_eq!(items.len(), 10);
    for item in &items {
        assert_eq!(item.get_quality(), "normal");
        assert_eq!(item.get_type(), "tool");
        assert_eq!(item.get_subtype(), "pickaxe");
        assert_eq!(item.get_name(), "pickaxe");
    }

    Ok(())
}

/// Test 3: Extremely skewed weights (1000:1 ratio)
/// Tests that the algorithm handles extreme weight disparities
#[test]
fn test_extreme_weight_skew() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Setup with extreme skew toward common
    generator.set_quality_data("common", 1000);
    generator.set_quality_data("legendary", 1);

    generator.set_item_type("weapon", 1000);
    generator.set_item_type("special", 1);

    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_item_subtype("special", "artifact", 1);

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_attribute(
        "special",
        "",
        ItemAttribute::new(
            "power",
            100.0,
            50.0,
            150.0,
            true,
        ),
    );

    generator.set_item(
        "weapon",
        "sword",
        vec!["sword"],
    );
    generator.set_item(
        "special",
        "artifact",
        vec!["artifact"],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "skew")?;

    // With 1000:1 weight, expect almost all to be the heavy weight item
    let common_count = items.iter().filter(|i| i.get_quality() == "common").count();
    let common_pct = common_count as f64 / 1000.0;

    // Should be >98% common (with 1000:1 ratio, expected rate is ~99.9%)
    assert!(common_pct > 0.98, "common expected >98%, got {}", common_pct * 100.0);

    Ok(())
}

/// Test 4: Many item types (10+) with varied weights
/// Tests performance and correctness with complex item hierarchies
#[test]
fn test_many_item_types() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    generator.set_quality_data("common", 100);
    generator.set_quality_data("rare", 10);

    // 10 different weapon types with varied weights
    let weapon_types = vec![
        ("sword", 50),
        ("axe", 40),
        ("mace", 30),
        ("bow", 20),
        ("staff", 15),
        ("spear", 10),
        ("dagger", 8),
        ("flail", 5),
        ("wand", 3),
        ("club", 2),
    ];

    generator.set_item_type("weapon", 1);

    for (subtype, weight) in &weapon_types {
        generator.set_item_subtype("weapon", subtype, *weight);
        let names = [format!("{}1", subtype), format!("{}2", subtype)];
        let names_refs: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
        generator.set_item("weapon", subtype, names_refs);
    }

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            25.0,
            5.0,
            50.0,
            true,
        ),
    );

    let options = GeneratorOptions {
        number_of_items: 500,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.2,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "many_types")?;

    assert_eq!(items.len(), 500);

    // Verify sword is most common (weight 50 out of 183 total)
    let sword_count = items.iter().filter(|i| i.get_subtype() == "sword").count();
    let sword_pct = sword_count as f64 / 500.0;
    let expected_sword_pct = 50.0 / 183.0;

    // Allow 8% deviation
    assert!(
        (sword_pct - expected_sword_pct).abs() < 0.08,
        "sword expected ~{}%, got {}%",
        expected_sword_pct * 100.0,
        sword_pct * 100.0
    );

    // Verify rarest item exists and is rare
    let club_count = items.iter().filter(|i| i.get_subtype() == "club").count();
    let club_pct = club_count as f64 / 500.0;
    assert!(club_pct < 0.08, "club expected <8%, got {}", club_pct * 100.0);

    Ok(())
}

/// Test 5: Full RPG scenario - weapons, armor, accessories with different distributions
/// Tests realistic game loot generation
#[test]
fn test_full_rpg_loot_scenario() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    // Quality tiers following typical game distribution
    generator.set_quality_data("common", 500);
    generator.set_quality_data("uncommon", 250);
    generator.set_quality_data("rare", 100);
    generator.set_quality_data("epic", 30);
    generator.set_quality_data("legendary", 5);

    // Item types with realistic proportions
    generator.set_item_type("weapon", 4);
    generator.set_item_type("armor", 3);
    generator.set_item_type("accessory", 2);
    generator.set_item_type("consumable", 1);

    // Weapon subtypes
    let weapon_subtypes = vec![
        ("sword", 3),
        ("axe", 2),
        ("bow", 2),
        ("staff", 1),
    ];
    for (subtype, weight) in &weapon_subtypes {
        generator.set_item_subtype("weapon", subtype, *weight);
        generator.set_item("weapon", subtype, vec![subtype]);
    }

    // Armor subtypes
    let armor_subtypes = vec![
        ("chest", 2),
        ("legs", 2),
        ("head", 1),
        ("feet", 1),
        ("hands", 1),
    ];
    for (subtype, weight) in &armor_subtypes {
        generator.set_item_subtype("armor", subtype, *weight);
        generator.set_item("armor", subtype, vec![subtype]);
    }

    // Accessory subtypes
    generator.set_item_subtype("accessory", "ring", 1);
    generator.set_item("accessory", "ring", vec!["ring"]);

    generator.set_item_subtype("accessory", "amulet", 1);
    generator.set_item("accessory", "amulet", vec!["amulet"]);

    // Consumable subtypes
    generator.set_item_subtype("consumable", "potion", 1);
    generator.set_item("consumable", "potion", vec!["potion"]);

    // Add attributes to all types
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            30.0,
            10.0,
            60.0,
            true,
        ),
    );

    generator.set_attribute(
        "armor",
        "",
        ItemAttribute::new(
            "defense",
            20.0,
            5.0,
            40.0,
            true,
        ),
    );

    generator.set_attribute(
        "accessory",
        "",
        ItemAttribute::new(
            "bonus",
            10.0,
            2.0,
            20.0,
            true,
        ),
    );

    generator.set_attribute(
        "consumable",
        "",
        ItemAttribute::new(
            "effect",
            5.0,
            1.0,
            10.0,
            true,
        ),
    );

    // Generate with affix chance
    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 20.0,
        level_variance: 10.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "rpg_loot")?;

    assert_eq!(items.len(), 1000);

    // Verify distribution of item types (4:3:2:1 ratio = 40:30:20:10)
    let weapon_count = items.iter().filter(|i| i.get_type() == "weapon").count() as f64 / 1000.0;
    let armor_count = items.iter().filter(|i| i.get_type() == "armor").count() as f64 / 1000.0;
    let accessory_count = items.iter().filter(|i| i.get_type() == "accessory").count() as f64 / 1000.0;
    let consumable_count = items.iter().filter(|i| i.get_type() == "consumable").count() as f64 / 1000.0;

    // Allow 8% deviation
    assert!(weapon_count > 0.32 && weapon_count < 0.48, "weapons expected ~40%, got {}", weapon_count * 100.0);
    assert!(armor_count > 0.22 && armor_count < 0.38, "armor expected ~30%, got {}", armor_count * 100.0);
    assert!(accessory_count > 0.12 && accessory_count < 0.28, "accessories expected ~20%, got {}", accessory_count * 100.0);
    assert!(consumable_count > 0.02 && consumable_count < 0.18, "consumables expected ~10%, got {}", consumable_count * 100.0);

    // Verify all items have valid attributes
    for item in &items {
        let attrs = item.get_attributes();
        assert!(!attrs.is_empty(), "item should have attributes");
    }

    Ok(())
}

/// Test 6: Linear vs exponential scaling comparison
/// Generates items with same base but different scaling to verify scaling factor effect
#[test]
fn test_linear_vs_exponential_scaling_comparison() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    generator.set_quality_data("standard", 1);
    generator.set_item_type("gem", 1);
    generator.set_item_subtype("gem", "emerald", 1);

    generator.set_attribute(
        "gem",
        "",
        ItemAttribute::new(
            "value",
            100.0,
            50.0,
            200.0,
            true,
        ),
    );

    generator.set_item(
        "gem",
        "emerald",
        vec!["emerald"],
    );

    // Generate with linear scaling
    let options_linear = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items_linear = generator.generate_loot(&options_linear, &GeneratorOverrides::empty(), "linear")?;

    // Generate with exponential scaling
    let options_exp = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.0,
        linear: false,
        scaling_factor: 1.5,
    };

    let items_exp = generator.generate_loot(&options_exp, &GeneratorOverrides::empty(), "exp")?;

    // Calculate average attribute values
    let linear_avg = items_linear
        .iter()
        .map(|i| {
            i.get_attributes()
                .get("value")
                .map(|a| a.get_initial_value())
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        / 100.0;

    let exp_avg = items_exp
        .iter()
        .map(|i| {
            i.get_attributes()
                .get("value")
                .map(|a| a.get_initial_value())
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        / 100.0;

    // Exponential scaling should produce higher average values
    assert!(
        exp_avg > linear_avg,
        "exponential avg {} should be > linear avg {}",
        exp_avg,
        linear_avg
    );

    Ok(())
}

/// Test 7: Override cascade - test all three override types together
/// Verifies overrides work correctly when multiple are specified
#[test]
fn test_override_cascade() -> Result<()> {
    let mut generator = PraedaGenerator::new();

    generator.set_quality_data("common", 1);
    generator.set_quality_data("rare", 100);

    generator.set_item_type("weapon", 1);
    generator.set_item_type("armor", 100);

    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_item_subtype("weapon", "axe", 100);

    generator.set_item_subtype("armor", "chest", 1);
    generator.set_item_subtype("armor", "legs", 100);

    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_attribute(
        "armor",
        "",
        ItemAttribute::new(
            "defense",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    generator.set_item("weapon", "sword", vec!["sword"]);
    generator.set_item("weapon", "axe", vec!["axe"]);
    generator.set_item("armor", "chest", vec!["chest"]);
    generator.set_item("armor", "legs", vec!["legs"]);

    // Override all three: force rare sword
    let overrides = GeneratorOverrides::new(
        "rare",
        "weapon",
        "sword",
    );

    let options = GeneratorOptions {
        number_of_items: 50,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator.generate_loot(&options, &overrides, "overrides")?;

    // All items must be rare swords
    for item in &items {
        assert_eq!(item.get_quality(), "rare");
        assert_eq!(item.get_type(), "weapon");
        assert_eq!(item.get_subtype(), "sword");
        assert_eq!(item.get_name(), "sword");
    }

    Ok(())
}

// ============================================================================
// FILE I/O AND SERIALIZATION TESTS
// ============================================================================


#[test]
fn test_load_toml_data() -> Result<()> {
    let mut generator = PraedaGenerator::new();
    let toml_path = "examples/test_data.toml";

    generator.load_data_from_file(toml_path)?;

    // Verify TOML was loaded
    assert!(!generator.get_quality_data().is_empty());
    assert!(!generator.get_item_types().is_empty());

    Ok(())
}

#[test]
fn test_generate_loot_json() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 5,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let json_str = generator.generate_loot_json(&options, &GeneratorOverrides::empty(), "json_gen")?;

    // Verify it's valid JSON and can be parsed
    let _: Vec<Item> = serde_json::from_str(&json_str)?;

    Ok(())
}

// ============================================================================
// MODEL STRUCT TESTS - SETTERS AND MUTATORS
// ============================================================================

#[test]
fn test_item_type_setters() {
    let mut item_type = ItemType::new("weapon", HashMap::new(), 1);

    item_type.set_type("armor".to_string());
    assert_eq!(item_type.get_type(), "armor");

    item_type.set_weight(5);
    assert_eq!(item_type.get_weight(), 5);
}

#[test]
fn test_item_attribute_setters() {
    let mut attr = ItemAttribute::new(
        "damage",
        10.0,
        1.0,
        20.0,
        false,
    );

    attr.set_name("health".to_string());
    assert_eq!(attr.get_name(), "health");

    attr.set_min(5.0);
    assert_eq!(attr.get_min(), 5.0);

    attr.set_max(50.0);
    assert_eq!(attr.get_max(), 50.0);

    attr.set_required(true);
    assert!(attr.get_required());
}

#[test]
fn test_item_empty() {
    let item = Item::empty();

    assert_eq!(item.get_name(), "");
    assert_eq!(item.get_quality(), "");
    assert_eq!(item.get_type(), "");
    assert_eq!(item.get_subtype(), "");
    assert_eq!(item.get_attributes().len(), 0);
}

#[test]
fn test_item_setters() {
    let mut item = Item::empty();

    item.set_name("sword".to_string());
    assert_eq!(item.get_name(), "sword");

    item.set_quality("rare".to_string());
    assert_eq!(item.get_quality(), "rare");

    item.set_type("weapon".to_string());
    assert_eq!(item.get_type(), "weapon");

    item.set_subtype("one-handed".to_string());
    assert_eq!(item.get_subtype(), "one-handed");
}

#[test]
fn test_item_prefix_suffix_mut() {
    let mut item = Item::empty();

    let prefix = Affix::new("sharp", vec![]);
    item.set_prefix(prefix);
    assert_eq!(item.get_prefix().get_name(), "sharp");

    // Test get_prefix_mut
    item.get_prefix_mut().set_name("super_sharp".to_string());
    assert_eq!(item.get_prefix().get_name(), "super_sharp");

    let suffix = Affix::new("of fire", vec![]);
    item.set_suffix(suffix);
    assert_eq!(item.get_suffix().get_name(), "of fire");
}

#[test]
fn test_item_attribute_access() {
    let mut item = Item::empty();

    let attr = ItemAttribute::new(
        "damage",
        10.0,
        1.0,
        20.0,
        true,
    );

    item.set_attribute("damage", attr);

    // Test has_attribute
    assert!(item.has_attribute("damage"));
    assert!(!item.has_attribute("nonexistent"));

    // Test get_attribute
    assert!(item.get_attribute("damage").is_some());
    assert!(item.get_attribute("nonexistent").is_none());

    // Test get_attribute_mut
    if let Some(attr_mut) = item.get_attribute_mut("damage") {
        attr_mut.set_initial_value(15.0);
    }
    assert_eq!(
        item.get_attribute("damage").unwrap().get_initial_value(),
        15.0
    );
}

#[test]
fn test_affix_setters() {
    let mut affix = Affix::empty();

    affix.set_name("fire".to_string());
    assert_eq!(affix.get_name(), "fire");

    let attr = ItemAttribute::new("damage", 5.0, 0.0, 10.0, false);
    let attrs = vec![attr];
    affix.set_attributes(attrs);
    assert_eq!(affix.get_attributes().len(), 1);
}

#[test]
fn test_affix_set_attribute() {
    let mut affix = Affix::new("fire", vec![]);

    let attr = ItemAttribute::new("damage", 5.0, 0.0, 10.0, false);
    affix.set_attribute(attr);
    assert_eq!(affix.get_attributes().len(), 1);

    // Setting same attribute again should replace it
    let attr2 = ItemAttribute::new("damage", 10.0, 0.0, 20.0, false);
    affix.set_attribute(attr2);
    assert_eq!(affix.get_attributes().len(), 1);
    assert_eq!(affix.get_attributes()[0].get_initial_value(), 10.0);
}

// ============================================================================
// GENERATOR OPTIONS AND OVERRIDES TESTS
// ============================================================================

#[test]
fn test_generator_options_new() {
    let opts = GeneratorOptions::new(
        10,
        5.0,
        2.0,
        0.5,
        false,
        1.5,
    );

    assert_eq!(opts.number_of_items, 10);
    assert_eq!(opts.base_level, 5.0);
    assert_eq!(opts.level_variance, 2.0);
    assert_eq!(opts.affix_chance, 0.5);
    assert!(!opts.is_linear());
    assert!(opts.is_exponential());
    assert_eq!(opts.scaling_factor, 1.5);
}

#[test]
fn test_generator_options_is_linear() {
    let linear_opts = GeneratorOptions::new(1, 1.0, 1.0, 0.25, true, 1.0);
    assert!(linear_opts.is_linear());
    assert!(!linear_opts.is_exponential());

    let exp_opts = GeneratorOptions::new(1, 1.0, 1.0, 0.25, false, 1.0);
    assert!(!exp_opts.is_linear());
    assert!(exp_opts.is_exponential());
}

#[test]
fn test_generator_overrides_new() {
    let overrides = GeneratorOverrides::new(
        "rare",
        "weapon",
        "sword",
    );

    assert_eq!(overrides.get_quality_override(), "rare");
    assert_eq!(overrides.get_type_override(), "weapon");
    assert_eq!(overrides.get_subtype_override(), "sword");
}

#[test]
fn test_generator_default() {
    let generator = PraedaGenerator::default();
    assert_eq!(generator.get_quality_data().len(), 0);
    assert_eq!(generator.get_item_types().len(), 0);
}

// ============================================================================
// ATTRIBUTE SCALING TESTS
// ============================================================================

#[test]
fn test_generate_value_linear_with_zero_bounds() {
    let mut attr = ItemAttribute::new("damage", 10.0, 0.0, 0.0, true);

    // Should set min/max to initial_value when both are 0
    attr.generate_value(5.0, true, 1.0);

    assert_eq!(attr.get_min(), 10.0);
    assert_eq!(attr.get_max(), 10.0);
}

#[test]
fn test_generate_value_exponential_zero_initial() {
    let mut attr = ItemAttribute::new("damage", 0.0, 0.0, 0.0, true);

    // Should set initial_value to 1.0 for exponential when 0
    attr.generate_value(5.0, false, 1.5);

    assert_eq!(attr.get_initial_value(), 1.5_f64.powf(5.0));
    assert!(attr.get_initial_value() > 0.0);
}

#[test]
fn test_generate_value_clamps_negative() {
    let mut attr = ItemAttribute::new("damage", 5.0, 0.0, 10.0, true);

    // Linear with negative scaling should clamp to 0
    attr.generate_value(10.0, true, -1.0);

    assert_eq!(attr.get_initial_value(), 0.0);
}

#[test]
fn test_attribute_generate_value_exponential() {
    let mut attr = ItemAttribute::new("damage", 10.0, 1.0, 100.0, true);

    attr.generate_value(5.0, false, 1.5);

    let expected = 10.0 * (1.5_f64.powf(5.0));
    assert!((attr.get_initial_value() - expected).abs() < 0.01);
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_item_type_has_subtype() {
    let mut item_type = ItemType::new("weapon", HashMap::new(), 1);

    // Add a subtype
    item_type.add_subtype("sword", 1);

    // Should have the subtype we added
    assert!(item_type.has_subtype("sword"));
    assert!(!item_type.has_subtype("nonexistent"));
}

#[test]
fn test_item_data_struct() {
    let item_data = ItemData::new(
        "weapon",
        "sword",
        vec!["longsword".to_string(), "shortsword".to_string()],
    );

    assert_eq!(item_data.get_item_type(), "weapon");
    assert_eq!(item_data.get_subtype(), "sword");
    assert_eq!(item_data.get_names().len(), 2);
}

#[test]
fn test_item_data_mutators() {
    let mut item_data = ItemData::new(
        "weapon",
        "sword",
        vec![],
    );

    item_data.set_item_type("armor".to_string());
    assert_eq!(item_data.get_item_type(), "armor");

    item_data.set_subtype("chest".to_string());
    assert_eq!(item_data.get_subtype(), "chest");

    item_data.add_name("chestplate".to_string());
    assert_eq!(item_data.get_names().len(), 1);
    assert_eq!(item_data.get_names()[0], "chestplate");
}

#[test]
fn test_attribute_updating_same_attribute() {
    let mut generator = PraedaGenerator::new();

    generator.set_quality_data("common", 100);
    generator.set_item_type("weapon", 1);

    // Set attribute first time
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    // Set same attribute again - should add to initial_value
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            5.0,
            1.0,
            20.0,
            true,
        ),
    );

    assert!(generator.has_attribute("weapon", "", "damage"));
}

#[test]
fn test_get_loot_json() -> Result<()> {
    let mut generator = create_test_generator();

    let options = GeneratorOptions::default();
    generator.generate_loot(&options, &GeneratorOverrides::empty(), "json_test")?;

    let json = generator.get_loot_json("json_test")?;
    assert!(!json.is_empty());

    // Verify it's valid JSON
    let _: Vec<Item> = serde_json::from_str(&json)?;

    Ok(())
}

#[test]
fn test_error_handling_invalid_toml() {
    let mut generator = PraedaGenerator::new();
    let invalid_toml = "[invalid TOML syntax ===";

    let result = generator.load_data(invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_weighted_random_select_with_single_item() -> Result<()> {
    let mut generator = PraedaGenerator::new();
    generator.set_quality_data("only_one", 1);
    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 1);
    generator.set_item("weapon", "sword", vec!["sword"]);
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    let options = GeneratorOptions::default();
    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "single")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_quality(), "only_one");

    Ok(())
}

#[test]
fn test_set_item_type_updates_existing() {
    let mut generator = PraedaGenerator::new();

    // Add an item type with weight 1
    generator.set_item_type("weapon", 1);
    assert_eq!(generator.get_item_type("weapon").unwrap().get_weight(), 1);

    // Update the same type with weight 5 - tests the rare "type already exists" path
    generator.set_item_type("weapon", 5);
    assert_eq!(generator.get_item_type("weapon").unwrap().get_weight(), 5);
}

#[test]
fn test_set_item_subtype_new_type() {
    let mut generator = PraedaGenerator::new();

    // Add subtype to non-existent type - creates new item type with single subtype
    generator.set_item_subtype("armor", "chest", 2);

    // Verify type was created
    assert!(generator.has_item_type("armor"));
    assert!(generator.has_item_subtype("armor", "chest"));
}

#[test]
fn test_has_item_subtype_nonexistent_type() {
    let mut generator = PraedaGenerator::new();
    generator.set_item_type("weapon", 1);

    // Check subtype for non-existent weapon-sword combination - rare path
    assert!(!generator.has_item_subtype("weapon", "nonexistent"));
}

#[test]
fn test_set_initial_value_bounds_from_zero() {
    let mut attr = ItemAttribute::new(
        "test",
        50.0,
        0.0,
        0.0,
        true,
    );

    // Both min and max are 0.0, set_initial_value should set them
    assert_eq!(attr.get_min(), 0.0);
    assert_eq!(attr.get_max(), 0.0);

    attr.set_initial_value(25.0);

    // After setting initial value, min/max should be set to initial value
    assert_eq!(attr.get_min(), 25.0);
    assert_eq!(attr.get_max(), 25.0);
    assert_eq!(attr.get_initial_value(), 25.0);
}

#[test]
fn test_has_attribute_missing_attributes() {
    let mut generator = PraedaGenerator::new();

    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 1);

    // Type and subtype exist, but no attributes set - tests the rare path where attributes aren't found
    assert!(!generator.has_attribute("weapon", "sword", "damage"));
}

#[test]
fn test_get_prefixes_missing() {
    let generator = PraedaGenerator::new();

    // No affixes defined - tests the rare path in get_prefixes
    let prefixes = generator.get_prefixes("weapon", "");
    assert_eq!(prefixes.len(), 0);
}

#[test]
fn test_get_suffixes_missing() {
    let generator = PraedaGenerator::new();

    // No affixes defined - tests the rare path in get_suffixes
    let suffixes = generator.get_suffixes("weapon", "");
    assert_eq!(suffixes.len(), 0);
}

#[test]
fn test_subtype_metadata_set_and_get() {
    let mut generator = PraedaGenerator::new();

    generator.set_subtype_metadata(
        "weapon",
        "one-handed",
        "is_two_handed",
        serde_json::json!(false),
    );

    let metadata = generator.get_subtype_metadata("weapon", "one-handed", "is_two_handed");
    assert!(metadata.is_some());
    assert_eq!(metadata.unwrap(), &serde_json::json!(false));
}

#[test]
fn test_get_all_subtype_metadata() {
    let mut generator = PraedaGenerator::new();

    generator.set_subtype_metadata(
        "weapon",
        "two-handed",
        "is_two_handed",
        serde_json::json!(true),
    );
    generator.set_subtype_metadata(
        "weapon",
        "two-handed",
        "weight",
        serde_json::json!(15),
    );

    let all_metadata = generator.get_all_subtype_metadata("weapon", "two-handed");
    assert!(all_metadata.is_some());

    let metadata = all_metadata.unwrap();
    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata.get("is_two_handed").unwrap(), &serde_json::json!(true));
    assert_eq!(metadata.get("weight").unwrap(), &serde_json::json!(15));
}

#[test]
fn test_item_metadata_set_and_get() {
    let mut item = Item::new(
        "test_sword",
        "common",
        "weapon",
        "one-handed",
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    item.set_metadata("is_magical", serde_json::json!(true));

    assert!(item.has_metadata("is_magical"));
    assert_eq!(item.get_metadata("is_magical"), Some(&serde_json::json!(true)));
}

#[test]
fn test_item_metadata_get_all() {
    let mut item = Item::new(
        "test_axe",
        "rare",
        "weapon",
        "two-handed",
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    item.set_metadata("is_two_handed", serde_json::json!(true));
    item.set_metadata("weight", serde_json::json!(20));

    let all_metadata = item.get_all_metadata();
    assert_eq!(all_metadata.len(), 2);
    assert_eq!(all_metadata.get("is_two_handed").unwrap(), &serde_json::json!(true));
    assert_eq!(all_metadata.get("weight").unwrap(), &serde_json::json!(20));
}

#[test]
fn test_generated_item_contains_subtype_metadata() {
    let mut generator = PraedaGenerator::new();

    // Setup quality data
    generator.set_quality_data("common", 100);

    // Setup item type and subtype
    generator.set_item_type("weapon", 1);
    generator.set_item_subtype("weapon", "sword", 1);

    // Set metadata for the subtype
    generator.set_subtype_metadata(
        "weapon",
        "sword",
        "is_magical",
        serde_json::json!(false),
    );

    // Setup attributes
    generator.set_attribute(
        "weapon",
        "",
        ItemAttribute::new(
            "damage",
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    // Setup item names
    generator.set_item(
        "weapon",
        "sword",
        vec!["longsword"],
    );

    // Generate item
    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 2.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = generator
        .generate_loot(&options, &GeneratorOverrides::empty(), "test")
        .unwrap();

    assert_eq!(items.len(), 1);
    let item = &items[0];

    // Verify the metadata was attached to the generated item
    assert!(item.has_metadata("is_magical"));
    assert_eq!(item.get_metadata("is_magical"), Some(&serde_json::json!(false)));
}

#[test]
fn test_load_metadata_from_toml() {
    let toml_str = r#"
[quality_data]
common = 100

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "damage"
initial_value = 10.0
min = 1.0
max = 20.0
required = true

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword"]

[[item_affixes]]
item_type = "weapon"
subtype = "sword"
[item_affixes.metadata]
is_legendary = true
rarity_multiplier = 1.5
    "#;

    let mut generator = PraedaGenerator::new();
    generator.load_data(toml_str).unwrap();

    // Verify metadata was loaded
    let metadata = generator.get_subtype_metadata("weapon", "sword", "is_legendary");
    assert!(metadata.is_some());
    assert_eq!(metadata.unwrap(), &serde_json::json!(true));

    let multiplier = generator.get_subtype_metadata("weapon", "sword", "rarity_multiplier");
    assert!(multiplier.is_some());
    assert_eq!(multiplier.unwrap(), &serde_json::json!(1.5));
}

