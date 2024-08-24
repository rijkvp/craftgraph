use gamedata::{GameData, RecipeItem, RecipeItems};

mod gamedata;
mod graph;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: {} <zip file> <item id>", args[0]);
        return Ok(());
    }

    let game_data = GameData::load(&args[1])?;

    let items = RecipeItems::single(RecipeItem::Item(args[2].clone()));
    let craft_graph = graph::calculate_craft_graph(game_data, items);
    println!("Crafting graph for {}:\n{}", args[2], craft_graph);

    Ok(())
}
