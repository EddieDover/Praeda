#include "praeda.hpp"
#include <iostream>

int main() {
    try {
        std::cout << "=== Praeda C++ FFI Test ===" << std::endl << std::endl;

        // Create generator
        std::cout << "Creating generator..." << std::endl;
        auto gen = praeda::Generator::create();
        std::cout << "✓ Generator created successfully" << std::endl << std::endl;

        // Test 1: Programmatic Configuration
        std::cout << "--- Test 1: Programmatic Configuration ---" << std::endl;

        std::cout << "Setting qualities..." << std::endl;
        gen->set_quality_data("common", 100);
        gen->set_quality_data("uncommon", 60);
        gen->set_quality_data("rare", 30);
        std::cout << "✓ Qualities set" << std::endl;

        std::cout << "Setting item types..." << std::endl;
        gen->set_item_type("weapon", 2);
        gen->set_item_type("armor", 1);
        std::cout << "✓ Item types set" << std::endl;

        std::cout << "Setting item subtypes..." << std::endl;
        gen->set_item_subtype("weapon", "sword", 3);
        gen->set_item_subtype("weapon", "axe", 2);
        gen->set_item_subtype("armor", "chest", 1);
        std::cout << "✓ Item subtypes set" << std::endl;

        std::cout << "Setting attributes..." << std::endl;
        praeda::ItemAttribute damage_attr("damage", 15.0, 5.0, 30.0, true);
        gen->set_attribute("weapon", "", damage_attr);
        praeda::ItemAttribute defense_attr("defense", 10.0, 2.0, 20.0, true);
        gen->set_attribute("armor", "", defense_attr);
        std::cout << "✓ Attributes set" << std::endl;

        std::cout << "Setting item names..." << std::endl;
        gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});
        gen->set_item_names("weapon", "axe", {"battleaxe"});
        gen->set_item_names("armor", "chest", {"plate_armor", "leather_armor"});
        std::cout << "✓ Item names set" << std::endl << std::endl;

        // Test 2: Query Methods
        std::cout << "--- Test 2: Query Methods ---" << std::endl;

        bool has_common = gen->has_quality("common");
        std::cout << "Has quality 'common': " << (has_common ? "true" : "false") << std::endl;

        bool has_epic = gen->has_quality("epic");
        std::cout << "Has quality 'epic': " << (has_epic ? "true" : "false") << std::endl << std::endl;

        // Test 3: Load Configuration from TOML
        std::cout << "--- Test 3: Load Configuration from TOML ---" << std::endl;

        std::string toml_config = R"(
[quality_data]
common = 100
uncommon = 60
rare = 30
legendary = 5

[[item_types]]
item_type = "weapon"
weight = 2
[item_types.subtypes]
sword = 3
axe = 2

[[item_types]]
item_type = "armor"
weight = 1
[item_types.subtypes]
chest = 1

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword", "shortsword", "bastard_sword"]

[[item_list]]
item_type = "weapon"
subtype = "axe"
names = ["battleaxe", "hand_axe"]

[[item_list]]
item_type = "armor"
subtype = "chest"
names = ["plate_armor", "leather_armor"]
)";

        std::cout << "Loading TOML configuration..." << std::endl;
        auto gen_toml = praeda::Generator::create();
        gen_toml->load_toml_string(toml_config);
        std::cout << "✓ TOML configuration loaded" << std::endl << std::endl;

        // Test 4: Generate Loot with Native Options from Programmatic Config
        std::cout << "--- Test 4: Loot Generation with Native Options (Programmatic) ---" << std::endl;

        praeda::GenerationOptions options;
        options.number_of_items = 5;
        options.base_level = 15.0;
        options.level_variance = 5.0;
        options.affix_chance = 0.75;
        options.linear = true;
        options.scaling_factor = 1.0;

        std::cout << "Generating 5 items with programmatic config..." << std::endl;
        auto items = gen->generate_loot(options);

        std::cout << "✓ Generated " << items.size() << " items:" << std::endl;
        for (size_t i = 0; i < items.size(); ++i) {
            const auto& item = items[i];
            std::cout << "  " << (i+1) << ". [" << item.quality
                      << "] " << item.type
                      << " / " << item.subtype
                      << " - " << item.name << std::endl;

            // Display attributes from native Item object
            if (!item.attributes.empty()) {
                std::cout << "      Attributes:" << std::endl;
                for (const auto& [key, attr] : item.attributes) {
                    std::cout << "        - " << attr.name << ": " << attr.initial_value
                              << " [" << attr.min << "-" << attr.max << "]" << std::endl;
                }
            }
        }
        std::cout << std::endl;

        // Test 5: Generate Loot from TOML Configuration
        std::cout << "--- Test 5: Loot Generation with Native Options (TOML) ---" << std::endl;

        praeda::GenerationOptions toml_options;
        toml_options.number_of_items = 3;
        toml_options.base_level = 10.0;
        toml_options.level_variance = 2.0;
        toml_options.affix_chance = 0.5;
        toml_options.linear = true;
        toml_options.scaling_factor = 1.0;

        std::cout << "Generating 3 items with TOML config..." << std::endl;
        auto toml_items = gen_toml->generate_loot(toml_options);

        std::cout << "✓ Generated " << toml_items.size() << " items from TOML:" << std::endl;
        for (size_t i = 0; i < toml_items.size(); ++i) {
            const auto& item = toml_items[i];
            std::cout << "  " << (i+1) << ". [" << item.quality
                      << "] " << item.type
                      << " / " << item.subtype
                      << " - " << item.name << std::endl;

            // Display attributes from native Item object
            if (!item.attributes.empty()) {
                std::cout << "      Attributes:" << std::endl;
                for (const auto& [key, attr] : item.attributes) {
                    std::cout << "        - " << attr.name << ": " << attr.initial_value
                              << " [" << attr.min << "-" << attr.max << "]" << std::endl;
                }
            }
        }
        std::cout << std::endl;

        // Test 6: Deterministic Loot Generation
        std::cout << "--- Test 6: Deterministic Loot Generation ---" << std::endl;

        const uint64_t seed = 20260817ULL;

        std::cout << "Generating 5 items twice from seed " << seed << "..." << std::endl;
        auto first_run = gen->generate_loot_seeded(options, seed);
        auto second_run = gen->generate_loot_seeded(options, seed);

        bool identical = first_run.size() == second_run.size();
        for (size_t i = 0; identical && i < first_run.size(); ++i) {
            identical = first_run[i].name == second_run[i].name
                     && first_run[i].quality == second_run[i].quality
                     && first_run[i].type == second_run[i].type
                     && first_run[i].subtype == second_run[i].subtype;
        }

        for (size_t i = 0; i < first_run.size(); ++i) {
            std::cout << "  " << (i+1) << ". [" << first_run[i].quality
                      << "] " << first_run[i].type
                      << " / " << first_run[i].subtype
                      << " - " << first_run[i].name << std::endl;
        }

        if (!identical) {
            std::cerr << "✗ Seeded generation returned different items for the same seed" << std::endl;
            return 1;
        }
        std::cout << "✓ Both runs produced identical items" << std::endl;

        auto other_seed = gen->generate_loot_seeded(options, seed + 1);
        std::cout << "✓ Seed " << (seed + 1) << " produced its own independent batch of "
                  << other_seed.size() << " items" << std::endl;
        std::cout << std::endl;

        // Test 7: Generator Info
        std::cout << "--- Test 7: Generator Info ---" << std::endl;
        std::string version = gen->info();
        std::cout << "Library version: " << version << std::endl;
        std::cout << "✓ Generator info retrieved successfully" << std::endl;
        std::cout << std::endl;

        std::cout << "=== All Tests Passed! ===" << std::endl;
        return 0;

    } catch (const praeda::Exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }
}
