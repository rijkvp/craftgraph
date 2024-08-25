use crate::gamedata::{GameData, Recipe, RecipeItems};
use num_rational::Ratio;
use std::collections::VecDeque;

pub struct CraftGraph {
    nodes: Vec<(RecipeItems, Ratio<u32>)>,
    edges: Vec<(usize, usize, usize)>,
    recipes: Vec<Recipe>,
}

impl std::fmt::Display for CraftGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NODES:")?;
        for (idx, (items, ratio)) in self.nodes.iter().enumerate() {
            writeln!(f, "{idx}: {items} x{ratio}")?;
        }
        writeln!(f, "EDGES:")?;
        for (idx, parent_idx, recipe_idx) in &self.edges {
            writeln!(f, "{idx} -> {parent_idx} ({})", self.recipes[*recipe_idx])?;
        }
        Ok(())
    }
}

pub fn calculate_craft_graph(game_data: &GameData, start: RecipeItems) -> CraftGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut recipes = Vec::new();

    let mut queue = VecDeque::new(); // items, ratio, parent_idx
    nodes.push((start.clone(), Ratio::ONE));
    queue.push_back((&start, Ratio::ONE, 0));

    while let Some((items, parent_ratio, parent_idx)) = queue.pop_front() {
        eprintln!("[NODE] {} (x{})", items, parent_ratio);
        for item in items.iter() {
            for recipe in game_data.get_recipes_for_item(item) {
                recipes.push(recipe.clone());
                let recipe_idx = recipes.len() - 1;

                let recipe_result = recipe.get_result().unwrap();

                for (ingredient, item_ratio) in recipe.get_ingredients() {
                    // Check if this ingredient is already in the graph
                    if nodes.iter().any(|(i, _)| i == ingredient) {
                        continue;
                    }

                    // Calculate new ratio and insert as new node
                    let new_ratio = Ratio::new(item_ratio, recipe_result.count) * parent_ratio;
                    eprintln!("[NEW NODE] {} (x{})", ingredient, parent_ratio);
                    nodes.push((ingredient.clone(), new_ratio));

                    // Add edge from new node to parent
                    let node_idx = nodes.len() - 1;
                    edges.push((node_idx, parent_idx, recipe_idx));
                    eprintln!(
                        "[EDGE] {ingredient} (x{item_ratio}) -> {item} (x{})  (ratio={new_ratio})",
                        recipe_result.count
                    );

                    // Add new node to the queue to processes it in BFS order
                    queue.push_back((ingredient, new_ratio, node_idx));
                }
            }
        }
    }
    CraftGraph {
        nodes,
        edges,
        recipes,
    }
}
