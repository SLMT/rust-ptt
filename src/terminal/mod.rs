
mod byte_matrix;

use std::mem;
use byte_matrix::ByteMatrix;

// Macros
macro_rules! ranged {
    ($x: expr, $min: expr, $max: expr) => (
        if $min < $x {
            if $max > $x { $x }
            else { $max }
        } else { $min }
    );
}

// Special Characters
const CHAR_ERASE: u8 = ' ' as u8;
const ATTR_ERASE: u8 = 0x07;

const CHAR_TAB: u8 = '\t' as u8;
const CHAR_BACK: u8 = '\b' as u8;
const CHAR_ESC: u8 = '\x1b' as u8;
const CHAR_RETURN: u8 = '\r' as u8;
const CHAR_NEWLINE: u8 = '\n' as u8;

// The Structure of Terminal
pub struct Terminal {
    // TODO: Fill in this

    // Buffers
    char_map: ByteMatrix, // character map
    attr_map: ByteMatrix, // attribute map

    // The size of the terminal
    rows: usize, cols: usize,

    // Cursor Pointers
    x: usize, y: usize,

    // Is dirty ?
    dirty: bool,
}

impl Terminal {
    pub fn new(row_count: usize, col_count: usize) -> Terminal {
        // Create a terminal object
        let mut term = Terminal {
            char_map: ByteMatrix::new(row_count, col_count),
            attr_map: ByteMatrix::new(row_count, col_count),

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

    pub fn resize(&mut self, row_count: usize, col_count: usize) {
        // TODO: Check row_count and col_count are in the pre-defined ranges

        // TODO: Reallocate buffers

        // TODO: Clear new exposed buffer after resized

        // TODO: Redraw

        // TODO: Set new cursor position (x, y)
    }

    /// Move the cursor
    /// (The original implementation in C is named as `move`, but `move` is a keyword in Rust)
    pub fn move_cur(&mut self, x: usize, y: usize) {
        self.x = ranged!(x, 0, self.cols - 1);
        self.y = ranged!(y, 0, self.rows - 1);
    }

    /// Clear from the current line to the target line
    pub fn clrtoln(&mut self, line: usize) {
        if line <= self.y {
            return;
        }
        let line1 = self.y;
        self.clrregion(line1, line - 1);
    }

    /// Clear from line 1 to line 2
    pub fn clrregion(&mut self, mut line1: usize, mut line2: usize) {
        // Swap if line1 behind line2
        if line1 > line2 {
            mem::swap(&mut line1, &mut line2);
        }

        // Make sure they are in the range
        line1 = ranged!(line1, 0, self.rows - 1);
        line2 = ranged!(line2, 0, self.rows - 1);

        // Clear the buffer
        for row in line1 .. (line2 + 1) {
            self.char_map.set_row(row, CHAR_ERASE);
            self.attr_map.set_row(row, ATTR_ERASE);
        }

        // Mark as dirty
        self.mark_dirty();
    }

    /// Output a string
    pub fn outs(&mut self, s: &str) {
        // TODO: Implement this
    }

    /// Output a character
    pub fn outc(&mut self, ch: u8) {
        // TODO: Ignore 0x00 and 0xFF (Invalid)
        if ch == 0x00 || ch == 0x00 {
            return;
        }

        // Mark as dirty
        self.mark_dirty();

        // TODO: Process escape commands

        // TODO: Process the char
        match ch {
            CHAR_ESC => {
                // TODO: Implement it
            },
            CHAR_TAB => {
                // TODO: Implement it
            },
            CHAR_BACK => {
                // TODO: Implement it
            },
            CHAR_RETURN || CHAR_NEWLINE => {
                // TODO: Implement it
            },
            _ => {
                // TODO: Implement it (two cases: control characters or normal character)
            }
        }
    }

    // TODO: Implement other APIs

    /// Mark as dirty
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}
