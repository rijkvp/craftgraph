#![allow(non_snake_case)]

mod gamedata;
mod graph;

use gamedata::{GameData, RecipeItem, RecipeItems};
use std::sync::OnceLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

pub async fn load_gamedata() -> Result<GameData, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("/data.json", &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;
    let data = serde_wasm_bindgen::from_value::<GameData>(json)?;
    Ok(data)
}

pub async fn get_game_data() -> Result<&'static GameData, JsValue> {
    match GAME_DATA.get() {
        Some(data) => Ok(data),
        None => {
            let data = load_gamedata().await?;
            GAME_DATA.set(data).unwrap();
            Ok(GAME_DATA.get().unwrap())
        }
    }
}

#[wasm_bindgen]
pub async fn loadItems() -> Result<JsValue, JsValue> {
    let game_data = get_game_data().await?;
    let items = serde_wasm_bindgen::to_value(&game_data.items)?;
    Ok(items)
}

#[wasm_bindgen]
pub async fn getCraftGraph(itemId: String) -> Result<JsValue, JsValue> {
    let game_data = get_game_data().await?;
    let items = RecipeItems::single(RecipeItem::Item(format!("minecraft:{itemId}")));
    let craft_graph = graph::calculate_craft_graph(&game_data, items);
    let js_value = serde_wasm_bindgen::to_value(&craft_graph)?;
    Ok(js_value)
}
