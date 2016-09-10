
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

    /// Set all entries in a givne range of a specified row with a given value
    /// XXX: If we could use memset(), we might get better performance
    pub fn set_bytes(&mut self, r: usize, start_c: usize, num: usize, new_val: u8) {
        let start = r * self.cols + start_c;
        for c in 0 .. num {
            self.data[start + c] = new_val;
        }
    }

    /// Set the entries in a row from the given start index to the end
    /// XXX: If we could use memset(), we might get better performance
    pub fn set_to_row_end(&mut self, r: usize, start_c: usize, new_val: u8) {
        let start = r * self.cols;
        for c in start .. self.cols {
            self.data[start + c] = new_val;
        }
    }

    /// Set all entries of a specified row with a given value
    /// XXX: If we could use memset(), we might get better performance
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
