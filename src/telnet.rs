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
const DO: u8 = 253;     // please, you use option
const WILL: u8 = 251;   // I will use option
const SB: u8 = 250;     // interpret as subnegotiation
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
    WaitForCommand
}

pub struct TelnetConnection {
    // Processing States
    process_state: ProcessState
}

impl TelnetConnection {
    pub fn new(stream: &mut TcpStream) -> TelnetConnection {
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
            process_state: ProcessState::None
        }
    }

    pub fn process(&mut self, byte: u8) {
        // Change to "wait for command" state when receiving IAC
        if byte == IAC {
            self.process_state = ProcessState::WaitForCommand;
            return;
        }

        match self.process_state {
            ProcessState::WaitForCommand => {
                println!("{}", byte);
            }

            ProcessState::None => {
                // Do Nothing
            }
        }
    }

}
