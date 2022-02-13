use midir::MidiOutputConnection;

/// Sends a matrix message to LPX
pub fn send_mat_message(lpx_out: &mut MidiOutputConnection, mut messages: Vec<u8>) {
    let mut msg = vec![240u8, 0u8, 32u8, 41u8, 2u8, 12u8, 3u8];
    msg.append(&mut messages);
    msg.append(&mut vec![247u8]);
    lpx_out.send(&msg).unwrap();
}

/// Convert from x, y (right, up) to LED index
pub fn xy_to_index(x: u8, y: u8) -> u8 {
    11 + x + y * 10
}