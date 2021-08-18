use ggez::error::{GameError, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::Color;
use ggez::*;
use super::grid::Grid;
use glam::*;

pub struct MainState {
  grid: Grid
}

impl MainState {
  pub fn new() -> Self {
    Self {
      grid: Grid::new(IVec2::from([80, 45]))
    }
  }
}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
    const TARGET_FPS: u32 = 60;
    while timer::check_update_time(ctx, TARGET_FPS) {
      //update objs
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
    graphics::clear(ctx, Color::BLACK);
    self.grid.draw(ctx)?;
    graphics::present(ctx)?;
    timer::yield_now();
    Ok(())
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
    let pos = Vec2::from([x, y]);
    //
  }

  fn key_down_event(
    &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    _keymod: KeyMods,
    _repeat: bool,
  ) {
    //
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
    //
  }
}
