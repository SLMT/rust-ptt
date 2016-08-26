
use std::mem;

// Macros
macro_rules! ranged {
    ($x: expr, $min: expr, $max: expr) => (
        if $min < $x {
            if $max > $x { $x }
            else { $max }
        } else { $min }
    );
}

// The Structure of Terminal
pub struct Terminal {
    // TODO: Fill in this

    // Buffers
    cmaps: Vec<Vec<u8>>, // character map
    amaps: Vec<Vec<u8>>, // attribute map

    // The size of the terminal
    rows: u32, cols: u32,

    // Cursor Pointers
    x: u32, y: u32,

    // Is dirty ?
    dirty: bool,
}

impl Terminal {
    pub fn new(row_count: u32, col_count: u32) -> Terminal {
        // Create a terminal object
        let mut term = Terminal {
            cmaps: generate_map(row_count, col_count),
            amaps: generate_map(row_count, col_count),

            rows: row_count, cols: col_count,
            x: 0, y: 0,
            dirty: false,

            // TODO: There are some other variables
        };

        // Resize the terminal
        // ???: Why should we generate twice ?
        term.resize(row_count, col_count);

        // TODO: Clear buffers
        // ???: Why do we need to clear it ?

        // TODO: Set typeahead

        // TODO: fterm_rawclear();

        // Reset the cursor
        term.move_cur(0, 0);

        term
    }

    pub fn resize(&mut self, row_count: u32, col_count: u32) {
        // TODO: Check row_count and col_count are in the pre-defined ranges

        // TODO: Reallocate buffers

        // TODO: Clear new exposed buffer after resized

        // TODO: Redraw

        // TODO: Set new cursor position (x, y)
    }

    /// Move the cursor
    /// (The original implementation in C is named as `move`, but `move` is a keyword in Rust)
    pub fn move_cur(&mut self, x: u32, y: u32) {
        self.x = ranged!(x, 0, self.cols - 1);
        self.y = ranged!(y, 0, self.rows - 1);
    }

    /// Clear from the current line to the target line
    pub fn clrtoln(&mut self, line: u32) {
        if line <= self.y {
            return;
        }
        let line1 = self.y;
        self.clrregion(line1, line - 1);
    }

    /// Clear from line 1 to line 2
    pub fn clrregion(&mut self, mut line1: u32, mut line2: u32) {
        // Swap if line1 behind line2
        if line1 > line2 {
            mem::swap(&mut line1, &mut line2);
        }

        // Make sure they are in the range
        line1 = ranged!(line1, 0, self.rows - 1);
        line2 = ranged!(line2, 0, self.rows - 1);

        // TODO: Clear the buffer

        // Mark as dirty
        self.mark_dirty();
    }

    /// Output a string
    pub fn outs(&mut self, s: &str) {
        // TODO: Implement this
    }

    // TODO: Implement other APIs

    /// Mark as dirty
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

/// Generate a map with the given size
fn generate_map(lines: u32, chars_in_lines: u32) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for _ in 0 .. lines {
        map.push(vec![0; chars_in_lines as usize]);
    }
    map
}
