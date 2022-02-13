mod midi;
mod hour;
mod lpx;
use std::{thread, time};

fn main() {
    // Get connection to LPX
    let mut lpx_out = midi::get_output_connection("Launchpad X LPX MIDI In".to_string());
    // Go to Programmer mode
    lpx_out.send(&[0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C, 0x0E, 0x01, 0xF7]).unwrap();
    // Run indefinitely
    loop {
        // Wait for next update
        thread::sleep(time::Duration::from_millis(100));
        // Get time in binary format
        let time = hour::get_time();
        // Init message vector
        let mut msg = vec![];
        // Center our on 8x8 grid
        for x in 2..6 {
            for y in 1..=6 {
                // Assign color to 1 and 0
                let color = match time[y-1].chars().nth(x-2).unwrap() {
                    '1' => vec![127u8, 127u8, 0u8],
                    _ => vec![0u8, 0u8, 0u8]
                };
                // Add u8 vector message
                msg.append(&mut vec![
                    3u8, 
                    lpx::xy_to_index(x as u8, y as u8), 
                    color[0], color[1], color[2]]);
            }
        }
        // Send mat message to LPX
        lpx::send_mat_message(&mut lpx_out, msg);
    }
}
