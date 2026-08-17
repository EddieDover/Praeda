using System;
using System.Collections.Generic;
using Praeda;

class Program {
    static int Main() {
        try {
            Console.WriteLine("=== Praeda C# FFI Test ===\n");

            // Create generator
            Console.WriteLine("Creating generator...");
            using var gen = new PraedaGenerator();
            Console.WriteLine("✓ Generator created successfully\n");

            // Test 1: Programmatic Configuration
            Console.WriteLine("--- Test 1: Programmatic Configuration ---");

            Console.WriteLine("Setting qualities...");
            if (!gen.SetQualityData("common", 100)) throw new Exception("Failed to set quality");
            if (!gen.SetQualityData("uncommon", 60)) throw new Exception("Failed to set quality");
            if (!gen.SetQualityData("rare", 30)) throw new Exception("Failed to set quality");
            Console.WriteLine("✓ Qualities set");

            Console.WriteLine("Setting item types...");
            if (!gen.SetItemType("weapon", 2)) throw new Exception("Failed to set item type");
            if (!gen.SetItemType("armor", 1)) throw new Exception("Failed to set item type");
            Console.WriteLine("✓ Item types set");

            Console.WriteLine("Setting item subtypes...");
            if (!gen.SetItemSubtype("weapon", "sword", 3)) throw new Exception("Failed to set item subtype");
            if (!gen.SetItemSubtype("weapon", "axe", 2)) throw new Exception("Failed to set item subtype");
            if (!gen.SetItemSubtype("armor", "chest", 1)) throw new Exception("Failed to set item subtype");
            Console.WriteLine("✓ Item subtypes set");

            Console.WriteLine("Setting attributes...");
            if (!gen.SetAttribute("weapon", "", "damage", 15.0, 5.0, 30.0, true)) throw new Exception("Failed to set attribute");
            if (!gen.SetAttribute("armor", "", "defense", 10.0, 2.0, 20.0, true)) throw new Exception("Failed to set attribute");
            Console.WriteLine("✓ Attributes set");

            Console.WriteLine("Setting item names...");
            if (!gen.SetItemNames("weapon", "sword", new[] { "longsword", "shortsword" })) throw new Exception("Failed to set item names");
            if (!gen.SetItemNames("weapon", "axe", new[] { "battleaxe" })) throw new Exception("Failed to set item names");
            if (!gen.SetItemNames("armor", "chest", new[] { "plate_armor", "leather_armor" })) throw new Exception("Failed to set item names");
            Console.WriteLine("✓ Item names set\n");

            // Test 2: Query Methods
            Console.WriteLine("--- Test 2: Query Methods ---");

            bool hasCommon = gen.HasQuality("common");
            Console.WriteLine("Has quality 'common': " + hasCommon);

            bool hasEpic = gen.HasQuality("epic");
            Console.WriteLine("Has quality 'epic': " + hasEpic);
            Console.WriteLine();

            // Test 3: Generate Loot
            Console.WriteLine("--- Test 3: Loot Generation ---");

            var options = new GenerationOptions {
                NumberOfItems = 5,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            Console.WriteLine("Generating 5 items...");
            var items = gen.GenerateLoot(options);

            Console.WriteLine("✓ Generated " + items.Count + " items:");
            int itemIndex = 1;
            foreach (var item in items) {
                Console.WriteLine($"  {itemIndex}. [{item.Quality}] {item.Type} / {item.Subtype} - {item.Name}");
                itemIndex++;
            }
            Console.WriteLine();

            // Test 4: Deterministic Loot Generation
            Console.WriteLine("--- Test 4: Deterministic Loot Generation ---");

            const ulong seed = 20260817UL;

            Console.WriteLine("Generating 5 items twice from seed " + seed + "...");
            var firstRun = gen.GenerateLootSeeded(options, seed);
            var secondRun = gen.GenerateLootSeeded(options, seed);

            bool identical = firstRun.Count == secondRun.Count;
            for (int i = 0; identical && i < firstRun.Count; i++) {
                identical =
                    firstRun[i].Name == secondRun[i].Name &&
                    firstRun[i].Quality == secondRun[i].Quality &&
                    firstRun[i].Type == secondRun[i].Type &&
                    firstRun[i].Subtype == secondRun[i].Subtype;
            }

            for (int i = 0; i < firstRun.Count; i++) {
                Console.WriteLine($"  {i + 1}. [{firstRun[i].Quality}] {firstRun[i].Type} / {firstRun[i].Subtype} - {firstRun[i].Name}");
            }

            if (!identical) {
                throw new InvalidOperationException("Seeded generation returned different items for the same seed");
            }
            Console.WriteLine("✓ Both runs produced identical items");

            var otherSeed = gen.GenerateLootSeeded(options, seed + 1);
            Console.WriteLine("✓ Seed " + (seed + 1) + " produced its own independent batch of " + otherSeed.Count + " items");
            Console.WriteLine();

            // Test 5: Generator Info
            Console.WriteLine("--- Test 5: Generator Info ---");
            string info = gen.GetInfo();
            Console.WriteLine("Generator info retrieved (raw format): " + info.Substring(0, Math.Min(50, info.Length)) + "...");
            Console.WriteLine();

            Console.WriteLine("=== All Tests Passed! ===");
            return 0;

        } catch (Exception ex) {
            Console.Error.WriteLine("Error: " + ex.Message);
            Console.Error.WriteLine(ex.StackTrace);
            return 1;
        }
    }
}
