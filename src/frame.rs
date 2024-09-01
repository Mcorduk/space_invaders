use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<& 'static str>>;
// [ 
//   [' ', 'P', ' ', ' ', ' '],  // Column 0, has 5 rows in it
//   [' ', ' ', 'E', ' ', ' '],  // Column 1
//   [' ', ' ', ' ', '*', ' '],  // Column 2
//   [' ', ' ', ' ', ' ', ' '],  // Column 3
//   [' ', ' ', ' ', ' ', ' ']   // Column 4
// ]



pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        
        for _ in 0..NUM_ROWS{
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw (&self, frame: &mut Frame);
}
