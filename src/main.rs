use simple_logger::{SimpleLogger};
use glam::*;
use log::{LevelFilter, info};
use ggez::*;
use pathfind::game::MainState;

const RES_1080: Vec2 = const_vec2!([1920.0, 1080.0]);

const TITLE: &str = "Title";
const AUTHOR: &str = "Author";
const GAME_ID: &str = "GAME_ID";

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level(module_path!(), LevelFilter::Debug)
    .init()
    .unwrap();

  info!("Start!");
  let window_setup = ggez::conf::WindowSetup::default().title(TITLE);
  let res = RES_1080;
  let window_mode = ggez::conf::WindowMode::default()
    .min_dimensions(res.x, res.y)
    .dimensions(res.x, res.y);
  // Make a Context.
  let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
    .window_setup(window_setup)
    .window_mode(window_mode)
    .build()
    .expect("Could not create ggez context!");

  let state = MainState{};

  event::run(ctx, event_loop, state);
}
