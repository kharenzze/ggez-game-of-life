pub struct Grid {
  dim: GridDimensions,
  matrix: Vec<Vec<Cell>>
}
pub struct Cell {
  state: CellState
}

enum CellState {
  Dead,
  Alive
}

pub struct GridDimensions {
  x: usize,
  y: usize,
}