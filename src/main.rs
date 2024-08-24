use gamedata::{GameData, RecipeItem, RecipeItems};

mod gamedata;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: {} <zip file> <item id>", args[0]);
        return Ok(());
    }

    let game_data = GameData::load(&args[1])?;

    let items = RecipeItems::single(RecipeItem::Item(args[2].clone()));
    let ingredients = game_data.get_ingredients_recursively(items);
    println!("Ingredients for {}:\n{:#?}", args[2], ingredients);

    Ok(())
}
