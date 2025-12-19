use yew::Renderer;
use crate::app::App;

mod app;
mod components;
mod types;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  Renderer::<App>::new().render();
}
