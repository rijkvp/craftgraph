use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
};
use zip::ZipArchive;

const RECIPES_DIR: &str = "data/minecraft/recipe/";
const TAGS_DIR: &str = "data/minecraft/tags/";

const fn default_one() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub struct RecipeResult {
    #[serde(default = "default_one")]
    pub count: u32,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum RecipeItem {
    Item(String),
    Tag(String),
}

impl std::fmt::Display for RecipeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeItem::Item(id) => write!(f, "{}", id),
            RecipeItem::Tag(id) => write!(f, "#{}", id),
        }
    }
}

// NOTE: Automatic implementation of PartialEq and Eq for RecipeItem might not be correct in all cases!
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[allow(unused)]
pub enum RecipeItems {
    Single(RecipeItem),
    Multiple(Vec<RecipeItem>),
}

impl RecipeItems {
    pub fn single(item: RecipeItem) -> Self {
        RecipeItems::Single(item)
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &RecipeItem> + '_> {
        match self {
            RecipeItems::Single(item) => Box::new(std::iter::once(item)),
            RecipeItems::Multiple(items) => Box::new(items.into_iter()),
        }
    }
}

impl std::fmt::Display for RecipeItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeItems::Single(item) => write!(f, "{}", item),
            RecipeItems::Multiple(items) => {
                write!(f, "[")?;
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(unused)]
pub enum Recipe {
    #[serde(rename = "minecraft:crafting_shaped")]
    Shaped {
        category: String,
        group: Option<String>,
        key: BTreeMap<String, RecipeItems>,
        pattern: Vec<String>,
        result: RecipeResult,
    },
    #[serde(rename = "minecraft:crafting_shapeless")]
    Shapeless {
        category: String,
        group: Option<String>,
        ingredients: Vec<RecipeItems>,
        result: RecipeResult,
    },
    #[serde(rename = "minecraft:stonecutting")]
    Stonecutting {
        ingredient: RecipeItems,
        result: RecipeResult,
    },
    #[serde(rename = "minecraft:smelting")]
    Smelting {
        category: String,
        cookingtime: u32,
        experience: f32,
        ingredient: RecipeItems,
        result: RecipeResult,
    },
    // Unused types
    #[serde(rename = "minecraft:crafting_special_armordye")]
    ArmorDye,
    #[serde(rename = "minecraft:crafting_special_bannerduplicate")]
    BannerDuplicate,
    #[serde(rename = "minecraft:blasting")]
    Blasting,
    #[serde(rename = "minecraft:crafting_special_bookcloning")]
    BookCloning,
    #[serde(rename = "minecraft:campfire_cooking")]
    CampfireCooking,
    #[serde(rename = "minecraft:crafting_decorated_pot")]
    DecoratedPot,
    #[serde(rename = "minecraft:crafting_special_firework_rocket")]
    FireforkRocket,
    #[serde(rename = "minecraft:crafting_special_firework_star")]
    FireforkStar,
    #[serde(rename = "minecraft:crafting_special_firework_star_fade")]
    FireforkStarFade,
    #[serde(rename = "minecraft:crafting_special_mapcloning")]
    MapCloning,
    #[serde(rename = "minecraft:crafting_special_mapextending")]
    MapExtending,
    #[serde(rename = "minecraft:crafting_special_repairitem")]
    RepairItem,
    #[serde(rename = "minecraft:crafting_special_shielddecoration")]
    ShieldDecoration,
    #[serde(rename = "minecraft:crafting_special_shulkerboxcoloring")]
    ShulkerBoxColoring,
    #[serde(rename = "minecraft:crafting_special_suspiciousstew")]
    SuspiciousStew,
    #[serde(rename = "minecraft:smoking")]
    Smoking,
    #[serde(rename = "minecraft:smithing_trim")]
    SmithingTrim,
    #[serde(rename = "minecraft:smithing_transform")]
    SmithingTrransform,
    #[serde(rename = "minecraft:crafting_special_tippedarrow")]
    TrippedArrow,
}

impl Recipe {
    pub fn get_result(&self) -> Option<&RecipeResult> {
        match self {
            Recipe::Shaped { result, .. } => Some(result),
            Recipe::Shapeless { result, .. } => Some(result),
            Recipe::Stonecutting { result, .. } => Some(result),
            Recipe::Smelting { result, .. } => Some(result),
            _ => None,
        }
    }

    pub fn get_ingredients(&self) -> Vec<(&RecipeItems, u32)> {
        match self {
            Recipe::Shaped { key, pattern, .. } => {
                let mut ingredients = Vec::new();
                for row in pattern {
                    for c in row.chars() {
                        if let Some(items) = key.get(&c.to_string()) {
                            if let Some((_, count)) =
                                ingredients.iter_mut().find(|(i, _)| *i == items)
                            {
                                *count += 1;
                            } else {
                                ingredients.push((items, 1));
                            }
                        }
                    }
                }
                ingredients
            }
            Recipe::Shapeless { ingredients, .. } => ingredients.iter().map(|i| (i, 1)).collect(),
            Recipe::Stonecutting { ingredient, .. } => vec![(ingredient, 1)],
            Recipe::Smelting { ingredient, .. } => vec![(ingredient, 1)],
            _ => Vec::new(),
        }
    }

    pub fn get_kind(&self) -> &str {
        match self {
            Recipe::Shaped { .. } => "Shaped",
            Recipe::Shapeless { .. } => "Shapeless",
            Recipe::Stonecutting { .. } => "Stonecutting",
            Recipe::Smelting { .. } => "Smelting",
            _ => "Unsupported",
        }
    }
}

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ingredients = self.get_ingredients();
        for (idx, (item, count)) in ingredients.iter().enumerate() {
            if idx > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} x{}", item, count)?;
        }
        let result = self.get_result().unwrap();
        write!(f, " -> {} x{}", result.id, result.count)?;
        write!(f, " ({})", self.get_kind())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tag {
    values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub items: Vec<String>,
    recipes: Vec<Recipe>,
    tags: HashMap<String, Tag>,
}

fn read_archive(file_name: &str) -> anyhow::Result<ZipArchive<File>> {
    let file = File::open(file_name)?;
    Ok(ZipArchive::new(file)?)
}

impl GameData {
    pub fn load(jar_file_path: &str) -> anyhow::Result<GameData> {
        let path_reader = read_archive(jar_file_path)?;
        let mut file_reader = read_archive(jar_file_path)?;
        let mut recipes = Vec::new();
        let mut tags = HashMap::new();
        for name in path_reader.file_names() {
            if name.ends_with(".json") {
                if name.starts_with(TAGS_DIR) {
                    let file = file_reader.by_name(name)?;
                    let tag: Tag = serde_json::from_reader(file)?;
                    let tag_name = name
                        .split('/')
                        .last()
                        .context("invalid filename")?
                        .trim_end_matches(".json");
                    tags.insert(format!("minecraft:{}", tag_name), tag);
                } else if name.starts_with(RECIPES_DIR) {
                    let file = file_reader.by_name(name)?;
                    let recipe: Recipe = serde_json::from_reader(file)?;
                    recipes.push(recipe);
                }
            }
        }
        let items = export_items(&recipes, &tags);
        Ok(Self {
            items,
            recipes,
            tags,
        })
    }

    pub fn resolve_tag<'a>(&'a self, tag_name: &str) -> Result<Vec<&'a str>> {
        resolve_tag_inner(tag_name, &self.tags)
    }

    fn resolve_item<'a>(&'a self, item: &'a RecipeItem) -> Vec<&'a str> {
        match item {
            RecipeItem::Item(id) => vec![&id],
            RecipeItem::Tag(id) => self.resolve_tag(id).expect("Failed to resolve tag"),
        }
    }

    pub fn get_recipes_for_item(&self, item: &RecipeItem) -> Vec<&Recipe> {
        let item_ids = self.resolve_item(item);
        let mut recipes = Vec::new();
        for recipe in &self.recipes {
            if let Some(result) = recipe.get_result() {
                if item_ids.contains(&result.id.as_str()) {
                    recipes.push(recipe);
                }
            }
        }
        recipes
    }
}

fn resolve_tag_inner<'a>(tag_name: &str, tags: &'a HashMap<String, Tag>) -> Result<Vec<&'a str>> {
    let mut item_ids = Vec::new();
    let tag = tags
        .get(tag_name)
        .context(anyhow!("Tag '{tag_name}' not found"))?;
    for value in &tag.values {
        // Tags are nested by adding a '#' in front of the tag name
        if value.starts_with('#') {
            item_ids.extend(resolve_tag_inner(&value[1..], tags)?);
        } else {
            item_ids.push(value);
        }
    }
    Ok(item_ids)
}

fn export_items(recipes: &Vec<Recipe>, tags: &HashMap<String, Tag>) -> Vec<String> {
    let mut item_ids: HashSet<&str> = HashSet::new();
    for recipe in recipes {
        for (recipe_items, _) in recipe.get_ingredients() {
            for item in recipe_items.iter() {
                match item {
                    RecipeItem::Item(id) => {
                        item_ids.insert(id);
                    }
                    RecipeItem::Tag(id) => {
                        let tag_items =
                            resolve_tag_inner(&id, tags).expect("Failed to resolve tag");
                        for tag_item in tag_items {
                            item_ids.insert(tag_item);
                        }
                    }
                }
            }
        }
        if let Some(recipe_result) = recipe.get_result() {
            item_ids.insert(&recipe_result.id);
        }
    }
    // split of the "minecraft:" prefix
    let mut item_names: Vec<String> = item_ids.into_iter().map(|s| s[10..].to_string()).collect();
    item_names.sort();
    item_names
}
