use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};
use zip::ZipArchive;

const RECIPES_DIR: &str = "data/minecraft/recipe/";

fn read_archive(file_name: &str) -> anyhow::Result<ZipArchive<File>> {
    let file = File::open(file_name)?;
    Ok(ZipArchive::new(file)?)
}

const fn default_one() -> u32 {
    1
}

#[derive(Debug, Clone, Deserialize)]
struct RecipeResult {
    #[serde(default = "default_one")]
    count: u32,
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RecipeItem {
    Item(String),
    Tag(String),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum RecipeItems {
    Single(RecipeItem),
    Multiple(Vec<RecipeItem>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
enum Recipe {
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
        ingredient: RecipeItem,
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

fn read_recipes(jar_file_path: &str) -> anyhow::Result<Vec<Recipe>> {
    let path_reader = read_archive(jar_file_path)?;
    let mut file_reader = read_archive(jar_file_path)?;
    let mut recipes = Vec::new();
    for name in path_reader.file_names() {
        if name.starts_with(RECIPES_DIR) && name.ends_with(".json") {
            let file = file_reader.by_name(name)?;
            let recipe: Recipe = serde_json::from_reader(file)?;
            recipes.push(recipe);
        }
    }
    Ok(recipes)
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <zip file>", args[0]);
        return Ok(());
    }

    let recipes = read_recipes(&args[1])?;
    println!("Read {} crafting recipes", recipes.len());

    Ok(())
}
