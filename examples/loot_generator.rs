use praeda::*;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate random loot items using Praeda", long_about = None)]
struct Args {
    /// Path to TOML configuration file with item types, attributes, and affixes (required unless --no-toml is used)
    #[arg(short = 'i', long)]
    input: Option<String>,

    /// Path where generated items will be saved as JSON
    #[arg(short = 'o', long)]
    output: String,

    /// Number of items to generate
    #[arg(short = 'n', long)]
    num_items: u32,

    /// Average item level (default: 10.0)
    #[arg(short = 'b', long, default_value = "10.0")]
    base_level: f64,

    /// Range around base level (default: 5.0)
    #[arg(short = 'v', long, default_value = "5.0")]
    level_variance: f64,

    /// Probability of applying affixes 0.0-1.0 (default: 0.75)
    #[arg(short = 'a', long, default_value = "0.75")]
    affix_chance: f64,

    /// Use exponential scaling instead of linear
    #[arg(long = "exponential")]
    exponential: bool,

    /// Multiplier for attribute scaling (default: 1.0)
    #[arg(short = 's', long, default_value = "1.0")]
    scaling_factor: f64,

    /// Use programmatic item generation instead of loading from TOML
    #[arg(long="no-toml", default_value = "false")]
    no_toml: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate that input is provided when not using --no-toml
    if !args.no_toml && args.input.is_none() {
        eprintln!("Error: --input is required unless --no-toml is specified");
        std::process::exit(1);
    }

    let mut generator = PraedaGenerator::new();
    if !args.no_toml {
        let input_path = args.input.as_ref().unwrap();
        eprintln!("Loading configuration from {}...", input_path);
        generator.load_data_from_file(input_path)?;
    } else {
        // Quality tiers
        generator.set_quality_data("common", 100);
        generator.set_quality_data("uncommon", 60);
        generator.set_quality_data("rare", 30);
        generator.set_quality_data("epic", 9);
        generator.set_quality_data("legendary", 1);

        // Item types
        generator.set_item_type("weapon", 1);
        generator.set_item_type("armor", 1);

        // Weapon subtypes
        generator.set_item_subtype("weapon", "one-handed", 1);
        generator.set_item_subtype("weapon", "two-handed", 1);

        // Armor subtypes
        generator.set_item_subtype("armor", "chest", 1);
        generator.set_item_subtype("armor", "feet", 1);
        generator.set_item_subtype("armor", "hands", 1);
        generator.set_item_subtype("armor", "head", 1);
        generator.set_item_subtype("armor", "legs", 1);
        generator.set_item_subtype("armor", "shoulders", 1);
        generator.set_item_subtype("armor", "waist", 1);
        generator.set_item_subtype("armor", "wrists", 1);

        // Global attributes
        generator.set_attribute("strength_requirement", "",
            ItemAttribute::new("strength_requirement", 0.0, 0.0, 100.0, false)
        );
        generator.set_attribute("dexterity_requirement", "",
            ItemAttribute::new("dexterity_requirement", 0.0, 0.0, 100.0, false)
        );
        generator.set_attribute("intelligence_requirement", "",
            ItemAttribute::new("intelligence_requirement", 0.0, 0.0, 100.0, false)
        );

        // Weapon attributes
        generator.set_attribute("level_requirement", "weapon",
            ItemAttribute::new("level_requirement", 0.0, 0.0, 100.0, false)
        );
        generator.set_attribute("durability", "weapon",
            ItemAttribute::new("durability", 16.0, 1.0, 16.0, true)
        );
        generator.set_attribute("attack_damage", "weapon",
            ItemAttribute::new("attack_damage", 1.0, 1.0, 5.0, true)
        );
        generator.set_attribute("attack_speed", "weapon",
            ItemAttribute::new("attack_speed", 0.0, 0.0, 100.0, true)
        );
        generator.set_attribute("critical_chance", "weapon",
            ItemAttribute::new("critical_chance", 0.0, 0.0, 100.0, false)
        );
        generator.set_attribute("critical_damage", "weapon",
            ItemAttribute::new("critical_damage", 0.0, 0.0, 100.0, false)
        );

        // Armor item names
        generator.set_item("armor", "chest", vec!["chestplate", "tunic"]);
        generator.set_item("armor", "feet", vec!["boots", "shoes"]);
        generator.set_item("armor", "hands", vec!["gauntlets", "gloves"]);
        generator.set_item("armor", "head", vec!["helm", "hood"]);
        generator.set_item("armor", "legs", vec!["legplates", "leggings"]);
        generator.set_item("armor", "shoulders", vec!["shoulderplates", "pauldrons"]);
        generator.set_item("armor", "waist", vec!["belt", "girdle"]);
        generator.set_item("armor", "wrists", vec!["bracers", "vambraces"]);

        // Weapon item names
        generator.set_item("weapon", "one-handed", vec!["sword", "axe", "mace", "dagger"]);
        generator.set_item("weapon", "two-handed", vec!["sword", "axe", "mace", "staff"]);

        // Armor affixes
        // Prefixes
        generator.set_prefix_attribute("armor", "", "heavy",
            ItemAttribute::new("durability", 10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("armor", "", "light",
            ItemAttribute::new("durability", -10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("armor", "", "light",
            ItemAttribute::new("strength_requirement", -10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("armor", "", "strong",
            ItemAttribute::new("strength_requirement", 10.0, 0.0, 0.0, false)
        );
        // Suffixes
        generator.set_suffix_attribute("armor", "", "of the bear",
            ItemAttribute::new("strength_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("armor", "", "of the eagle",
            ItemAttribute::new("intelligence_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("armor", "", "of the wolf",
            ItemAttribute::new("dexterity_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("armor", "", "of the lion",
            ItemAttribute::new("strength_requirement", 5.0, 0.0, 0.0, false)
        );

        // Weapon affixes
        // Prefixes
        generator.set_prefix_attribute("weapon", "", "sharp",
            ItemAttribute::new("attack_damage", 10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("weapon", "", "dull",
            ItemAttribute::new("attack_damage", -10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("weapon", "", "heavy",
            ItemAttribute::new("attack_speed", -10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("weapon", "", "light",
            ItemAttribute::new("attack_speed", 10.0, 0.0, 0.0, false)
        );
        generator.set_prefix_attribute("weapon", "", "strong",
            ItemAttribute::new("strength_requirement", 10.0, 0.0, 0.0, false)
        );
        // Suffixes
        generator.set_suffix_attribute("weapon", "", "of the bear",
            ItemAttribute::new("strength_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("weapon", "", "of the eagle",
            ItemAttribute::new("intelligence_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("weapon", "", "of the wolf",
            ItemAttribute::new("dexterity_requirement", 10.0, 0.0, 0.0, false)
        );
        generator.set_suffix_attribute("weapon", "", "of the lion",
            ItemAttribute::new("strength_requirement", 5.0, 0.0, 0.0, false)
        );
    }

    // Generate loot with user-specified options
    let linear = !args.exponential;  // Default is linear unless --exponential flag is set

    eprintln!("Generating {} items...", args.num_items);
    eprintln!("  Base Level: {}", args.base_level);
    eprintln!("  Level Variance: {}", args.level_variance);
    eprintln!("  Affix Chance: {}", args.affix_chance);
    eprintln!("  Scaling Mode: {}", if linear { "linear" } else { "exponential" });
    eprintln!("  Scaling Factor: {}", args.scaling_factor);

    let options = GeneratorOptions {
        number_of_items: args.num_items,
        base_level: args.base_level,
        level_variance: args.level_variance,
        affix_chance: args.affix_chance,
        linear,
        scaling_factor: args.scaling_factor,
    };

    let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "cli")?;

    // Save output to JSON
    eprintln!("Saving {} items to {}...", items.len(), args.output);
    let output_json = serde_json::to_string_pretty(&items)?;
    fs::write(&args.output, output_json)
        .expect("Failed to write output file");

    println!("✅ Successfully generated {} items and saved to {}", items.len(), args.output);
    Ok(())
}
