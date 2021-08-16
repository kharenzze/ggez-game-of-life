pub struct Grid {
  dim: GridDimensions,
  matrix: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub struct Cell {
  state: CellState,
}

impl Default for Cell {
  fn default() -> Self {
    Self {
      state: CellState::Dead,
    }
  }
}

#[derive(Debug)]
enum CellState {
  Dead,
  Alive,
}

#[derive(Default, Debug)]
pub struct GridDimensions {
  x: usize,
  y: usize,
}
