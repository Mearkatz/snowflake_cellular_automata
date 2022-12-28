use std::{fmt::Display, time::{Duration, Instant}};

use ndarray::prelude::*;
use rand::{seq::SliceRandom, Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    Flying,
    Frozen,
}

impl Cell {
    fn is_frozen(&self) -> bool {
        &Self::Frozen == self
    }

    fn is_flying(&self) -> bool {
        &Self::Flying == self
    }

    fn is_empty(&self) -> bool {
        &Self::Empty == self
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Flying => write!(f, "~"),
            Cell::Frozen => write!(f, "*"),
        }
    }
}

fn main() {
    let timer = Instant::now();

    const WIDTH: usize = 5;
    const HEIGHT: usize = 5;
    const STARTING_FLYING_CELLS: usize = 2;
    let frame_rate: u64 = 8; // Frames expected per second
    let frame_time = frame_rate_to_frame_time(frame_rate);  // Time to sleep between frames to maintain framerate    

    // Stores whether each cell is frozen or not
    let mut cells: Array2<Cell> = Array2::from_elem((HEIGHT, WIDTH), Cell::Empty);

    // Set middle cell to frozen
    // so that other cells will freeze when they touch it
    cells[[HEIGHT / 2, WIDTH / 2]] = Cell::Frozen;

    let mut rng = rand::thread_rng();
    for _ in 0..STARTING_FLYING_CELLS {
        let y = rng.gen_range(0..HEIGHT);
        let x = rng.gen_range(0..WIDTH);
        cells[[y, x]] = Cell::Flying;
    }

    /*
    SIMULATION LOOP
    */
    let indices_of_cells = indices_of_2d_array(HEIGHT, WIDTH);

    let mut flying_cells_seen = STARTING_FLYING_CELLS;
    while flying_cells_seen != 0 {
        // Show cells as they are
        std::thread::sleep(frame_time);
        clearscreen::clear().unwrap();
        show_cells(cells.view());        
        
        flying_cells_seen = 0; // Reset each frame

        // Update each cell one at a time (if they're flying)
        for cell_index in indices_of_cells.iter() {
            let (y, x) = *cell_index;
            let cell = cells[*cell_index];
            if let Cell::Flying = cell {
                // Mark that we've seen a flying cell
                flying_cells_seen += 1;

                // Freeze check:
                // If any neighbor to the North/East/South/West are frozen, become frozen too.
                // Handling edges:
                // - If a neighbor is on the left edge, it is considered its own neighbor.
                //      - The worst that can happen as a result of this is that a cell moves to where it already is, which is fine.
                let left_index = [y, x.saturating_sub(1)];
                let right_index = [y, (x + 1) % WIDTH];
                let up_index = [y.saturating_sub(1), x];
                let down_index = [(y + 1) % HEIGHT, x];

                let left: Cell = cells[left_index];
                let right = cells[right_index];
                let up: Cell = cells[up_index];
                let down: Cell = cells[down_index];

                // Check if any of these neighbors are frozen, if they are: freeze.
                if [left, right, up, down].into_iter().any(|c| c.is_frozen()) {
                    cells[*cell_index] = Cell::Frozen;
                } else {
                    // 1. filter neighbors_indices by which are empty
                    let empty_nbor_indices: Vec<[usize; 2]> = [
                        (left, left_index),
                        (right, right_index),
                        (up, up_index),
                        (down, down_index),
                    ]
                    .into_iter()
                    .filter_map(|(current_cell, cell_index)| {
                        if current_cell.is_empty() {
                            Some(cell_index)
                        } else {
                            None
                        }
                    })
                    .collect();

                    // 2. If that vec is non-empty, pick one randomly and move there
                    // Pick a new position to move to (as an index into `cells`)
                    if let Some(index_to_move_to) = empty_nbor_indices.choose(&mut rng) {
                        // Set old position to empty
                        cells[*cell_index] = Cell::Empty;

                        // Set new position to flying
                        cells[*index_to_move_to] = Cell::Flying;
                    }
                }
            }
        }
    }

    // ===================
    // SHOW FINAL RESULTS
    // ===================
    clearscreen::clear().unwrap();
    show_cells(cells.view());
    println!("FINAL STATE OF BOARD.");
    println!("Finished in {:?}", timer.elapsed());
    
}

// Converts a frame_rate into a frame_time, meaning how long each individual frame lasts in microseconds
fn frame_rate_to_frame_time(frame_rate: u64) -> Duration {
    // Store number of microseconds per second
    let micros_per_sec: u64 = 1_000_000;

    Duration::from_micros(
        micros_per_sec / frame_rate
    )
}

fn indices_of_2d_array(height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut indices = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            indices.push((y, x));
        }
    }
    indices
}

fn show_cells(arr: ArrayView2<Cell>) {
    for row in arr.rows() {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}
