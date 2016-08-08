/// # Telnet Simplified Implementation
///
/// The code here is based on the implementation in [original PTT](https://github.com/ptt/pttbbs),
///  which is written by piaip@csie.ntu.edu.tw in C. You can find the original implementation in
/// the `common\sys\telnet.c` file of PTT's repository.
///

use std::net::TcpStream;
use std::io::Write;

// Telnet Commands (only implement necessary commands)
const IAC: u8 = 255;    // interpret as command:
const DONT: u8 = 254;   // you are not to use option
const DO: u8 = 253;     // please, you use option
const WONT: u8 = 252;   // I won't use option
const WILL: u8 = 251;   // I will use option
const SB: u8 = 250;     // interpret as subnegotiation
const AYT: u8 = 246;    // are you there
const SE: u8 = 240;     // end sub negotiation

// Telnet Options (only provide necessary options)
const OPT_BINARY: u8 = 0;   // 8-bit data path
const OPT_ECHO: u8 = 1;     // echo
const OPT_SGA: u8 = 3;      // suppress go ahead
const OPT_TTYPE: u8 = 24;   // terminal type
const OPT_NAWS: u8 = 31;    // window size

// Telnet Sub-option Qualifiers (only provide necessary qualifiers)
const QUAL_SEND: u8 = 1;    // send option

enum ProcessState {
    None,
    WaitForCommand,
    SubNegotiation
}

pub struct TelnetConnection {
    // Callbacks
    cb_resize_term: Option<fn(u32, u32)>,

    // Processing States
    process_state: ProcessState,
    process_buffer: Vec<u8>
}

impl TelnetConnection {
    pub fn new(stream: &mut TcpStream, cb_resize_term: Option<fn(u32, u32)>) -> TelnetConnection {
        // Send initial message
        let init_msg: &[u8] = &[
            IAC, DO, OPT_TTYPE,
            IAC, SB, OPT_TTYPE, QUAL_SEND, IAC, SE,

            IAC, DO, OPT_NAWS,

            IAC, WILL, OPT_ECHO,

            IAC, WILL, OPT_SGA,

            IAC, WILL, OPT_BINARY,
            IAC, DO,   OPT_BINARY,
        ];
        stream.write(init_msg).unwrap();

        // Return a new connection
        TelnetConnection {
            cb_resize_term: cb_resize_term,
            process_state: ProcessState::None,
            process_buffer: Vec::new()
        }
    }

    pub fn process(&mut self, byte: u8) {
        // println!("Get byte: {}", byte);

        // Process Commands
        match self.process_state {
            // Wait for a command
            ProcessState::WaitForCommand => {
                self.process_state = ProcessState::None;

                match byte {
                    IAC => {}, // TODO: Should throw an error
                    AYT => {}, // TODO: Response
                    DONT | DO | WONT | WILL => {}, // TODO: Wait for implementation
                    SB => {
                        self.process_state = ProcessState::SubNegotiation;
                    },
                    SE => {
                        self.process_subnegotiation();
                    },
                    _ => {} // TODO: Default action
                }
            },

            // Sub-negotiation
            ProcessState::SubNegotiation => match byte {
                IAC => {
                    // Change to "wait for command" state when receiving IAC
                    self.process_state = ProcessState::WaitForCommand;
                },
                _ => {
                    self.process_buffer.push(byte);
                }
            },

            // Not in any state
            ProcessState::None => match byte {
                IAC => {
                    // Change to "wait for command" state when receiving IAC
                    self.process_state = ProcessState::WaitForCommand;
                },
                _ => {} // TODO: Default action
            }
        }
    }

    pub fn process_subnegotiation(&mut self) {
        let buf = &mut self.process_buffer;
        let option = buf[0];

        // println!("Option: {}", option);
        match option {
            // Resize terminal
            OPT_NAWS => {
                // Get the width and the height
                let width = ((buf[1] as u32) << 8) + (buf[2] as u32);
                let height = ((buf[3] as u32) << 8) + (buf[4] as u32);

                // Call term_resize callback
                match self.cb_resize_term {
                    Some(cb) => cb(width, height),
                    None => {} // Do Nothing
                }

                // TODO: Call update client code callback
            },

            // Terminal Type
            OPT_TTYPE => {}, // TODO: Wait for implementation

            // Default
            _ => {} // TODO: Default action
        }

        // Clear the buffer
        buf.clear();
    }
}
