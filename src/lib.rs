//! # Praeda - Procedural Loot Generator
//!
//! A high-performance procedural loot generator library designed for game development.
//! Generate randomized items with qualities, affixes, and attributes using configurable rules.
//!
//! ## Features
//!
//! - **Procedural Item Generation**: Create unique items with random qualities and properties
//! - **Affix System**: Apply prefixes and suffixes to items
//! - **Attribute System**: Define custom attributes (damage, defense, health, etc.)
//! - **Quality/Rarity Tiers**: Configure weighted quality levels (common, rare, legendary, etc.)
//! - **Flexible Configuration**: Use TOML files or programmatic API
//! - **FFI Bindings**: Call from C++, C#, and other languages
//!
//! ## Quick Start (Rust Library)
//!
//! ```rust
//! use praeda::{PraedaGenerator, GeneratorOptions, GeneratorOverrides};
//!
//! let mut generator = PraedaGenerator::new();
//!
//! // Configure qualities (rarity tiers)
//! generator.set_quality_data("common", 100);
//! generator.set_quality_data("rare", 30);
//! generator.set_quality_data("legendary", 5);
//!
//! // Configure item types
//! generator.set_item_type("weapon", 2);
//! generator.set_item_subtype("weapon", "sword", 3);
//! generator.set_item_subtype("weapon", "axe", 2);
//!
//! // Generate items with options
//! let options = GeneratorOptions {
//!     number_of_items: 5,
//!     base_level: 10.0,
//!     level_variance: 2.0,
//!     affix_chance: 0.25,
//!     linear: true,
//!     scaling_factor: 1.5,
//! };
//!
//! let items = generator.generate_loot(&options, &GeneratorOverrides::empty(), "main")?;
//! for item in items {
//!     println!("Generated: {} ({})", item.name, item.quality);
//! }
//! # Ok::<_, Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Configuration
//!
//! ### Using TOML Files
//!
//! Define a TOML file with your loot configuration:
//!
//! ```toml
//! [quality_data]
//! common = 100
//! rare = 30
//! legendary = 5
//!
//! [[item_types]]
//! item_type = "weapon"
//! weight = 2
//! [item_types.subtypes]
//! sword = 3
//! axe = 2
//! ```
//!
//! Then load it:
//!
//! ```rust,ignore
//! let mut generator = PraedaGenerator::new();
//! generator.load_data_from_file("loot.toml")?;
//! ```
//!
//! ### Programmatic Configuration
//!
//! Use the [`PraedaGenerator`] API to configure qualities, item types, affixes, and attributes.
//!
//! ## Core Types
//!
//! - [`PraedaGenerator`] - Main generator for creating loot
//! - [`Item`] - A generated item with quality, type, affixes, and attributes
//! - [`Affix`] - Prefix or suffix applied to items
//! - [`ItemAttribute`] - Custom attribute on an item
//! - [`GeneratorOptions`] - Options controlling generation behavior
//! - [`PraedaError`] - Error type for operation failures
//!
//! ## FFI Usage (C, C++, C#)
//!
//! This library provides C-compatible FFI bindings for non-Rust languages.
//! See the `ffi` module or the [FFI documentation](https://github.com/edover/praeda/blob/master/FFI.md)
//! for language-specific examples and detailed API reference.

pub mod models;
pub mod generator;
pub mod error;
pub mod ffi;

pub use models::*;
pub use generator::*;
pub use error::*;
