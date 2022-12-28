use ndarray::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Flying,
    Frozen,
}

fn main() {
    const WIDTH: usize = 16;
    const HEIGHT: usize = 8;
    const STARTING_FLYING_CELLS: usize = 5;

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
    loop {
        // Show cells as they are
        show_cells(cells.view());

        // Update each cell one at a time (if they're flying)
        for cell_index in indices_of_cells.iter() {
            let cell = cells[*cell_index];
            if let Cell::Flying = cell {
                println!("Flying cell at {:?}", *cell_index);
            }
        }
    }
    
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
            match cell {
                Cell::Empty => print!(" "),
                Cell::Flying => print!("f"),
                Cell::Frozen => print!("*"),
            }            
        }
        println!(); 
    }
}