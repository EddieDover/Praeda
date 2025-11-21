use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an item type with subtypes and weight
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemType {
    pub item_type: String,
    pub subtypes: HashMap<String, i32>,
    pub weight: i32,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ItemType {
    pub fn new(item_type: &str, subtypes: HashMap<String, i32>, weight: i32) -> Self {
        ItemType {
            item_type: item_type.to_string(),
            subtypes,
            weight,
            metadata: HashMap::new(),
        }
    }

    pub fn set_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_type(&self) -> &str {
        &self.item_type
    }

    pub fn add_subtype(&mut self, subtype: &str, weight: i32) {
        self.subtypes.insert(subtype.to_string(), weight);
    }

    pub fn get_subtypes(&self) -> &HashMap<String, i32> {
        &self.subtypes
    }

    pub fn has_subtype(&self, subtype: &str) -> bool {
        self.subtypes.contains_key(subtype)
    }

    pub fn set_weight(&mut self, weight: i32) {
        self.weight = weight;
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    pub fn get_all_metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }

    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

/// Represents item data (names for specific types/subtypes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemData {
    pub item_type: String,
    pub subtype: String,
    pub names: Vec<String>,
    /// Per-item metadata: maps item name to metadata properties
    #[serde(default)]
    pub item_metadata: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl ItemData {
    pub fn new(item_type: &str, subtype: &str, names: Vec<String>) -> Self {
        ItemData {
            item_type: item_type.to_string(),
            subtype: subtype.to_string(),
            names,
            item_metadata: HashMap::new(),
        }
    }

    pub fn set_item_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_item_type(&self) -> &str {
        &self.item_type
    }

    pub fn set_subtype(&mut self, subtype: String) {
        self.subtype = subtype;
    }

    pub fn get_subtype(&self) -> &str {
        &self.subtype
    }

    pub fn add_name(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn get_names(&self) -> &[String] {
        &self.names
    }

    /// Set metadata for a specific item name
    pub fn set_item_metadata(&mut self, item_name: String, key: String, value: serde_json::Value) {
        self.item_metadata
            .entry(item_name)
            .or_default()
            .insert(key, value);
    }

    /// Get metadata for a specific item name and key
    pub fn get_item_metadata(&self, item_name: &str, key: &str) -> Option<&serde_json::Value> {
        self.item_metadata
            .get(item_name)
            .and_then(|metadata| metadata.get(key))
    }

    /// Get all metadata for a specific item name
    pub fn get_item_all_metadata(&self, item_name: &str) -> Option<&HashMap<String, serde_json::Value>> {
        self.item_metadata.get(item_name)
    }

    /// Check if an item has metadata
    pub fn has_item_metadata(&self, item_name: &str, key: &str) -> bool {
        self.item_metadata
            .get(item_name)
            .map(|m| m.contains_key(key))
            .unwrap_or(false)
    }
}

/// Represents a single attribute or stat on an item.
///
/// Attributes are custom properties that can be attached to items during generation.
/// Examples: damage, defense, health, mana, armor class, critical strike chance, etc.
///
/// # Fields
///
/// * `name` - Attribute name (e.g., "damage", "defense")
/// * `initial_value` - Base value for this attribute
/// * `min` - Minimum possible value after scaling
/// * `max` - Maximum possible value after scaling
/// * `required` - If true, this attribute is always applied; if false, it depends on chance
/// * `scaling_factor` - Multiplier applied per level (linear: adds, exponential: multiplies)
/// * `chance` - Probability (0.0-1.0) of being included if not required
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemAttribute {
    pub name: String,
    pub initial_value: f64,
    pub min: f64,
    pub max: f64,
    pub required: bool,
    #[serde(default)]
    pub scaling_factor: f64,
    #[serde(default)]
    pub chance: f64,
}

impl ItemAttribute {
    /// Creates a new item attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the attribute
    /// * `initial_value` - Starting value before level scaling
    /// * `min` - Minimum value after scaling
    /// * `max` - Maximum value after scaling
    /// * `required` - If true, always applied; if false, chance-based
    pub fn new(
        name: &str,
        initial_value: f64,
        min: f64,
        max: f64,
        required: bool,
    ) -> Self {
        ItemAttribute {
            name: name.to_string(),
            initial_value,
            min,
            max,
            required,
            scaling_factor: 1.0,
            chance: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_initial_value(&mut self, initial_value: f64) {
        // LCOV_EXCL_START - Rare path: setting bounds when both are zero
        if self.min == 0.0 && self.max == 0.0 {
            self.min = initial_value;
            self.max = initial_value;
        }
        // LCOV_EXCL_END
        self.initial_value = initial_value;
    }

    pub fn get_initial_value(&self) -> f64 {
        self.initial_value
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn get_min(&self) -> f64 {
        self.min
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }

    pub fn get_max(&self) -> f64 {
        self.max
    }

    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }

    pub fn get_required(&self) -> bool {
        self.required
    }

    /// Generate a scaled value based on level, scaling factor, and linear/exponential progression
    pub fn generate_value(&mut self, new_level: f64, linear: bool, scaling_factor: f64) {
        if self.min == 0.0 && self.max == 0.0 && self.initial_value != 0.0 {
            self.min = self.initial_value;
            self.max = self.initial_value;
        }

        if self.initial_value == 0.0 && !linear {
            self.initial_value = 1.0;
        }

        if linear {
            self.initial_value += new_level * scaling_factor;
        } else {
            self.initial_value *= scaling_factor.powf(new_level);
        }

        if self.initial_value < 0.0 {
            self.initial_value = 0.0;
        }
    }
}

/// Represents a prefix or suffix affix.
///
/// Affixes are optional name modifiers that can be applied to items (e.g., "Flaming", "of Strength").
/// Each affix is associated with a set of attributes that are applied to the item when the affix is selected.
///
/// # Example
///
/// A "Flaming" prefix might add extra fire damage to a weapon, implemented as an affix with a
/// single damage attribute.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Affix {
    pub name: String,
    pub attributes: Vec<ItemAttribute>,
}

impl Affix {
    pub fn new(name: &str, attributes: Vec<ItemAttribute>) -> Self {
        Affix { name: name.to_string(), attributes }
    }

    pub fn empty() -> Self {
        Affix {
            name: String::new(),
            attributes: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_attributes(&self) -> &[ItemAttribute] {
        &self.attributes
    }

    pub fn set_attributes(&mut self, attributes: Vec<ItemAttribute>) {
        self.attributes = attributes;
    }

    pub fn set_attribute(&mut self, new_attribute: ItemAttribute) {
        if let Some(pos) = self
            .attributes
            .iter()
            .position(|a| a.get_name() == new_attribute.get_name())
        {
            self.attributes[pos] = new_attribute;
        } else {
            self.attributes.push(new_attribute);
        }
    }
}

/// Represents a complete generated item.
///
/// An `Item` is the output of the loot generation process. It contains all the information
/// about a single generated item, including its identity, quality tier, type, affixes, and stats.
///
/// # Fields
///
/// * `name` - Display name of the item (e.g., "Iron Sword", "Leather Armor")
/// * `quality` - Quality/rarity tier (e.g., "common", "rare", "legendary")
/// * `item_type` - Category type (e.g., "weapon", "armor")
/// * `subtype` - Specific subtype (e.g., "sword", "plate armor")
/// * `prefix` - Prefix affix applied to this item (empty if none)
/// * `suffix` - Suffix affix applied to this item (empty if none)
/// * `attributes` - Map of attribute names to their values (damage, defense, etc.)
/// * `metadata` - Additional metadata (application-specific data)
///
/// # Example
///
/// A generated item might look like:
/// ```text
/// name: "Flaming Iron Sword"
/// quality: "rare"
/// item_type: "weapon"
/// subtype: "sword"
/// prefix: Affix { name: "Flaming", attributes: [damage: 5.0] }
/// suffix: Affix { name: "of Strength", attributes: [strength: 3.0] }
/// attributes: { "damage": 15.0, "crit_chance": 0.1 }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub quality: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub subtype: String,
    pub prefix: Affix,
    pub suffix: Affix,
    pub attributes: HashMap<String, ItemAttribute>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Item {
    /// Creates a new item with the specified properties.
    ///
    /// This is typically called internally by [`PraedaGenerator`](crate::generator::PraedaGenerator)
    /// during loot generation, but can also be used to manually construct items.
    pub fn new(
        name: &str,
        quality: &str,
        item_type: &str,
        subtype: &str,
        prefix: Affix,
        suffix: Affix,
        attributes: HashMap<String, ItemAttribute>,
    ) -> Self {
        Item {
            name: name.to_string(),
            quality: quality.to_string(),
            item_type: item_type.to_string(),
            subtype: subtype.to_string(),
            prefix,
            suffix,
            attributes,
            metadata: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Item {
            name: String::new(),
            quality: String::new(),
            item_type: String::new(),
            subtype: String::new(),
            prefix: Affix::empty(),
            suffix: Affix::empty(),
            attributes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_quality(&mut self, quality: String) {
        self.quality = quality;
    }

    pub fn get_quality(&self) -> &str {
        &self.quality
    }

    pub fn set_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_type(&self) -> &str {
        &self.item_type
    }

    pub fn set_subtype(&mut self, subtype: String) {
        self.subtype = subtype;
    }

    pub fn get_subtype(&self) -> &str {
        &self.subtype
    }

    pub fn set_prefix(&mut self, prefix: Affix) {
        self.prefix = prefix;
    }

    pub fn get_prefix(&self) -> &Affix {
        &self.prefix
    }

    pub fn get_prefix_mut(&mut self) -> &mut Affix {
        &mut self.prefix
    }

    pub fn set_suffix(&mut self, suffix: Affix) {
        self.suffix = suffix;
    }

    pub fn get_suffix(&self) -> &Affix {
        &self.suffix
    }

    #[cfg(not(tarpaulin_include))]
    pub fn get_suffix_mut(&mut self) -> &mut Affix {
        &mut self.suffix
    }

    #[cfg(not(tarpaulin_include))]
    pub fn set_attributes(&mut self, attributes: HashMap<String, ItemAttribute>) {
        self.attributes = attributes;
    }

    pub fn get_attributes(&self) -> &HashMap<String, ItemAttribute> {
        &self.attributes
    }

    pub fn set_attribute(&mut self, name: &str, attr: ItemAttribute) {
        self.attributes.insert(name.to_string(), attr);
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&ItemAttribute> {
        self.attributes.get(name)
    }

    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut ItemAttribute> {
        self.attributes.get_mut(name)
    }

    pub fn set_metadata(&mut self, key: &str, value: serde_json::Value) {
        self.metadata.insert(key.to_string(), value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    pub fn get_all_metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }

    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

/// Options controlling loot generation behavior.
///
/// These parameters define how items are generated, including how many items to create,
/// what level range they should be, and how attributes scale.
///
/// # Fields
///
/// * `number_of_items` - How many items to generate
/// * `base_level` - Starting level for items (used for attribute scaling)
/// * `level_variance` - Range around base_level (actual level = base ± variance)
/// * `affix_chance` - Probability (0.0-1.0) that an affix is selected for each item
/// * `linear` - If true, attributes scale linearly; if false, exponentially
/// * `scaling_factor` - Multiplier applied per level
///   - Linear: adds `level * scaling_factor` to attribute value
///   - Exponential: multiplies attribute value by `scaling_factor^level`
///
/// # Example
///
/// ```rust,ignore
/// let options = GeneratorOptions {
///     number_of_items: 5,        // Generate 5 items
///     base_level: 10.0,          // Level 10 items
///     level_variance: 2.0,       // Range: level 8-12
///     affix_chance: 0.25,        // 25% chance for affixes
///     linear: true,              // Linear attribute scaling
///     scaling_factor: 1.5,       // Adds 1.5 per level linearly
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratorOptions {
    pub number_of_items: u32,
    pub base_level: f64,
    pub level_variance: f64,
    pub affix_chance: f64,
    pub linear: bool,
    pub scaling_factor: f64,
}

impl GeneratorOptions {
    pub fn new(
        number_of_items: u32,
        base_level: f64,
        level_variance: f64,
        affix_chance: f64,
        linear: bool,
        scaling_factor: f64,
    ) -> Self {
        GeneratorOptions {
            number_of_items,
            base_level,
            level_variance,
            affix_chance,
            linear,
            scaling_factor,
        }
    }

    pub fn is_linear(&self) -> bool {
        self.linear
    }

    pub fn is_exponential(&self) -> bool {
        !self.linear
    }
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            number_of_items: 1,
            base_level: 1.0,
            level_variance: 1.0,
            affix_chance: 0.25,
            linear: true,
            scaling_factor: 1.0,
        }
    }
}

/// Per-generation overrides for loot generation.
///
/// Allow forcing specific item properties during generation instead of random selection.
/// Leave fields empty to use random selection for that property.
///
/// # Fields
///
/// * `quality_override` - If set, forces items to this quality; if empty, quality is random
/// * `type_override` - If set, forces items to this type; if empty, type is random
/// * `subtype_override` - If set, forces items to this subtype; if empty, subtype is random
///
/// # Example
///
/// ```rust,ignore
/// // Generate legendary weapons only
/// let overrides = GeneratorOverrides {
///     quality_override: "legendary".to_string(),
///     type_override: "weapon".to_string(),
///     subtype_override: "".to_string(),  // Random subtype
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratorOverrides {
    pub quality_override: String,
    pub type_override: String,
    pub subtype_override: String,
}

impl GeneratorOverrides {
    pub fn new(
        quality_override: &str,
        type_override: &str,
        subtype_override: &str,
    ) -> Self {
        GeneratorOverrides {
            quality_override: quality_override.to_string(),
            type_override: type_override.to_string(),
            subtype_override: subtype_override.to_string(),
        }
    }

    pub fn empty() -> Self {
        GeneratorOverrides {
            quality_override: String::new(),
            type_override: String::new(),
            subtype_override: String::new(),
        }
    }

    pub fn get_quality_override(&self) -> &str {
        &self.quality_override
    }

    pub fn get_type_override(&self) -> &str {
        &self.type_override
    }

    pub fn get_subtype_override(&self) -> &str {
        &self.subtype_override
    }
}

// ============================================================================
// TOML Intermediate Structures for Deserialization
// ============================================================================

/// Intermediate structure for loading TOML configuration
#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub quality_data: HashMap<String, i32>,
    #[serde(default)]
    pub item_types: Vec<ItemType>,
    #[serde(default)]
    pub item_attributes: Vec<TomlItemAttributes>,
    #[serde(default)]
    pub item_list: Vec<TomlItemList>,
    #[serde(default)]
    pub item_affixes: Vec<TomlItemAffixes>,
}

/// Item attributes for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemAttributes {
    #[serde(default)]
    pub item_type: String,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub attributes: Vec<ItemAttribute>,
}

/// Item list for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemList {
    pub item_type: String,
    pub subtype: String,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub item_metadata: HashMap<String, HashMap<String, serde_json::Value>>,
}

/// Item affixes for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemAffixes {
    #[serde(default)]
    pub item_type: String,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub prefixes: Vec<Affix>,
    #[serde(default)]
    pub suffixes: Vec<Affix>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
