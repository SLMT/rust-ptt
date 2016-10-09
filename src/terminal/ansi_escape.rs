
pub const ESC_START_CH: u8 = '\x1b' as u8; // ESC character
pub const CSI_START_CH: u8 = '\x5b' as u8; // '['

const CSI_PARAM_START_CH: u8 = '\x30' as u8; // '0'
const CSI_PARAM_END_CH: u8 = '\x3f' as u8; // '?'

pub fn is_csi_parameter(ch: u8) -> bool {
    ch >= CSI_PARAM_START_CH && ch <= CSI_PARAM_END_CH
}
