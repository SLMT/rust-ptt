
pub struct ByteMatrix {
    // Size
    rows: usize,
    cols: usize,

    // Data
    data: Vec<u8>,
}

impl ByteMatrix {

    /// Create a matrix with given number of rows and columns
    pub fn new(rows: usize, cols: usize) -> ByteMatrix {
        let len = rows * cols;
        let matrix: Vec<u8> = vec![0; len];

        ByteMatrix {
            rows: rows,
            cols: cols,
            data: matrix,
        }
    }

    pub fn set(&mut self, r: usize, c: usize, new_val: u8) {
        let cols = self.cols;
        self.data[r * cols + c] = new_val;
    }

    /// Set all entries of a specified row with the given value
    /// TODO: If we could use memset(), we might get better performance
    pub fn set_row(&mut self, r: usize, new_val: u8) {
        let cols = self.cols;
        let start = r * cols;
        for c in 0 .. cols {
            self.data[start + c] = new_val;
        }
    }

    pub fn get(&self, r: usize, c: usize) -> u8 {
        let cols = self.cols;
        self.data[r * cols + c]
    }
}
