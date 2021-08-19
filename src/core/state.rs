use ggez::error::{GameError, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::Color;
use super::grid::Grid;
use ggez::*;
use glam::*;

pub struct MainState {
  grid: Grid
}

impl MainState {
  pub fn new() -> Self {
    Self {
      grid: Grid::new(IVec2::from([16, 9]) * 3)
    }
  }
}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
    const TARGET_FPS: u32 = 5;
    while timer::check_update_time(ctx, TARGET_FPS) {
      self.grid.update(ctx)?;
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

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    let pos = Vec2::from([x, y]);
    self.grid.handle_mouse_click(ctx, button, pos);
  }

  fn key_down_event(
    &mut self,
    ctx: &mut Context,
    keycode: KeyCode,
    keymod: KeyMods,
    repeat: bool,
  ) {
    self.grid.handle_key_press(ctx, keycode, keymod, repeat);
  }

  fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
    //
  }
}
