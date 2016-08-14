
pub struct Terminal {
    // TODO: Fill in this
}

impl Terminal {
    pub fn new(row_count: u32, col_count: u32) -> Terminal {
        // Create a terminal object
        let term = Terminal {
            
        };

        // Resize the terminal
        term.resize(row_count, col_count);

        // TODO: Clear buffers

        // TODO: Set typeahead

        // TODO: fterm_rawclear();

        // TODO: move(0, 0);
    }

    pub fn resize(&mut self, row_count: u32, col_count: u32) {
        // TODO: Check row_count and col_count are in the pre-defined ranges

        // TODO: Reallocate buffers

        // TODO: Clear new exposed buffer after resized

        // TODO: Redraw

        // TODO: Set new cursor position (x, y)
    }
}
