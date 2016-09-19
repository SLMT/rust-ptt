
mod byte_matrix;

use std::mem;

use self::byte_matrix::ByteMatrix;

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
const ATTR_DEFAULT: u8 = ATTR_ERASE;

const CHAR_TAB: u8 = '\t' as u8;
const CHAR_BACK: u8 = 0x08 as u8;
const CHAR_ESC: u8 = '\x1b' as u8;
const CHAR_RETURN: u8 = '\r' as u8;
const CHAR_NEWLINE: u8 = '\n' as u8;

// The Structure of Terminal
pub struct Terminal {
    // TODO: Fill in this

    // Buffers
    char_map: ByteMatrix, // character map
    attr_map: ByteMatrix, // attribute map

    // Attribute
    attribute: u8,

    // The size of the terminal
    rows: usize, cols: usize,

    // Cursor Pointers
    x: usize, y: usize,

    // Is dirty ?
    dirty: bool,

    // Escape commands buffer
    commands: Vec<u8>
}

impl Terminal {
    pub fn new(row_count: usize, col_count: usize) -> Terminal {
        // Create a terminal object
        let mut term = Terminal {
            char_map: ByteMatrix::new(row_count, col_count),
            attr_map: ByteMatrix::new(row_count, col_count),

            attribute: ATTR_DEFAULT,
            rows: row_count, cols: col_count,
            x: 0, y: 0,
            dirty: false,

            commands: Vec::new()

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

    /// Clear to the end of the line
    pub fn clrtoeol(&mut self) {
        // TODO: Do we need to check x, y here ?
        let x = self.x;
        let y = self.y;
        self.char_map.set_to_row_end(y, x, CHAR_ERASE);
        self.attr_map.set_to_row_end(y, x, ATTR_ERASE);
        self.mark_dirty();
    }

    /// Output a string
    pub fn outs(&mut self, s: &str) {
        for c in s.chars() {
            outc(c);
        }
    }

    /// Output a character
    pub fn outc(&mut self, ch: u8) {
        // TODO: Ignore 0x00 and 0xFF (Invalid)
        if ch == 0x00 || ch == 0xFF {
            return;
        }

        // Mark as dirty
        self.mark_dirty();

        // Process escape commands
        if !self.commands.is_empty() {
            // TODO: Implement it
        }

        // TODO: Process the char
        match ch {
            CHAR_ESC => {
                // Start escaped commands
                self.commands.push(ch);
            },
            CHAR_TAB => {
                // Tab: Move by 8 chars
                let mut new_x = self.x;
                if new_x % 8 == 0 {
                    new_x += 8;
                } else {
                    new_x += (8 - (new_x % 8));
                }
                new_x = ranged!(new_x, 0, self.cols - 1);

                // Erase the characters
                if new_x > self.x {
                    let cur_row = self.y;
                    let start_col = self.x;
                    let num = new_x - start_col;
                    let attr = self.attribute;

                    self.char_map.set_bytes(cur_row, start_col, num, CHAR_ERASE);
                    self.attr_map.set_bytes(cur_row, start_col, num, attr);
                }

                // Assign new x
                self.x = new_x;
            },
            CHAR_BACK => {
                // Move back by 1
                self.x = ranged!(self.x - 1, 0, self.cols - 1);
            },
            CHAR_RETURN | CHAR_NEWLINE => {
                // New line: only move the cursor to the next line
                self.clrtoeol();
                self.x = 0;
                self.y += 1;

                // XXX: The implementation here is slightly different from the one of pfterm.c in pttbbs
                if self.y >= self.rows {
                    self.y = self.rows - 1;
                }
            },
            _ if (ch as char).is_control() => {
                // Non-control characters: save the character

                // TODO: Check if self.x is the range (necessary ?)

                let mut col = self.x;
                let mut row = self.y;
                let attr = self.attribute;
                self.char_map.set(row, col, ch);
                self.attr_map.set(row, col, attr);

                // Move the cursor
                col += 1;
                if col >= self.cols {
                    self.x = 0;

                    // XXX: The implementation here is slightly different from the one of pfterm.c in pttbbs
                    row += 1;
                    if row < self.rows {
                        self.y = row;
                    }
                } else {
                    self.x = col;
                }
            },
            _ => {
                // If it is other control character, do nothing
            }
        }
    }

    fn process_command() {
        // TODO: Implement this method
    }

    // TODO: Implement other APIs

    /// Mark as dirty
    fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}
