use ybot::memory::{get_process_pid, GDMemory};
use ybot::input::{Action, Input};
use inputbot::{KeybdKey::*};
use tinyfiledialogs::{open_file_dialog, save_file_dialog_with_filter};
use std::fs;

#[derive(PartialEq)]
enum State {
    Recording,
    Playback,
    Pause,
}

fn main() {
    let pid = get_process_pid("GeometryDash.exe").expect("could not get process PID, is GD running?");
    let mut mem = GDMemory::from_pid(pid).expect("could not open process memory, must be running as root");

    let mut inputs = Vec::new();

    let mut space_was_pressed = false;
    let mut f2_was_pressed = false;
    let mut f3_was_pressed = false;
    let mut f4_was_pressed = false;
    let mut f8_was_pressed = false;
    let mut f9_was_pressed = false;
    let mut last_x_pos = 0.;
    let mut state = State::Recording;
    let mut current_input = 0;
    let mut smart_recording = false;

    loop {
        if F2Key.is_pressed() {
            if !f2_was_pressed {
                if let Some(path) = open_file_dialog("Open replay", "", Some((&["*.ybr"], "YBot Replay"))) {
                    inputs =
                        (&fs::read(&path).unwrap())
                            .chunks(9)
                            .map(Input::deserialize)
                            .collect::<Vec<_>>();
                    println!("Loaded replay from file: {}", path);
                }
                f2_was_pressed = true;
            }
        } else if f2_was_pressed {
            f2_was_pressed = false;
        }

        if F3Key.is_pressed() {
            if !f3_was_pressed {
                if let Some(path) = save_file_dialog_with_filter("Save replay", ".ybr", &["*.ybr"], "YBot Replay") {
                    fs::write(
                        &path,
                        inputs
                            .iter()
                            .map(|input| input.serialize())
                            .fold(Vec::with_capacity(inputs.len() * 9), |mut vec, input| { vec.extend_from_slice(&input); vec })
                    ).unwrap();
                    println!("Saved replay to file: {}", path);
                }
                f3_was_pressed = true;
            }
        } else if f3_was_pressed {
            f3_was_pressed = false;
        }

        if F4Key.is_pressed() {
            if !f4_was_pressed {
                match state {
                    State::Recording => {
                        println!("Paused recording");
                        state = State::Pause;
                    },
                    State::Playback => {
                        println!("Stopped playback, recording");
                        state = State::Recording;
                    },
                    State::Pause => {
                        println!("Unpaused recording");
                        state = State::Recording;
                    }
                }
                f4_was_pressed = true;
            }
        } else if f4_was_pressed {
            f4_was_pressed = false;
        }

        if F5Key.is_pressed() && !inputs.is_empty() && state != State::Playback {
            println!("Starting playback");
            state = State::Playback;
            current_input = 0;
        }

        match state {
            State::Recording => {
                if F6Key.is_pressed() && !inputs.is_empty() {
                    inputs.clear();
                    println!("Inputs cleared");
                }

                if F8Key.is_pressed() {
                    if !f8_was_pressed {
                        mem.update_addresses().unwrap();
                        f8_was_pressed = true;
                        println!("Updated level addresses")
                    }
                } else if f8_was_pressed {
                    f8_was_pressed = false;
                }

                if F9Key.is_pressed() {
                    if !f9_was_pressed {
                        smart_recording = !smart_recording;
                        println!("Toggled smart recording: {}", smart_recording);
                        f9_was_pressed = true;
                    }
                } else if f9_was_pressed {
                    f9_was_pressed = false;
                }

                if SpaceKey.is_pressed() {
                    if !space_was_pressed {
                        inputs.push(Input::new(mem.get_x_pos().unwrap(), mem.get_y_pos().unwrap(), Action::Press));
                        space_was_pressed = true;
                        println!("{:?}", inputs.last().unwrap());
                    }
                } else if space_was_pressed {
                    inputs.push(Input::new(mem.get_x_pos().unwrap(), mem.get_y_pos().unwrap(), Action::Release));
                    space_was_pressed = false;
                    println!("{:?}", inputs.last().unwrap());
                }

                if smart_recording {
                    let x_pos = mem.get_x_pos().unwrap();
                    if x_pos < last_x_pos {
                        let mut j = 0;
                        for i in (0..inputs.len()).rev() {
                            if inputs[i].x_pos >= x_pos {
                                inputs.pop();
                                j += 1;
                            } else {
                                break
                            }
                        }
                        println!("died - removed {} inputs", j);
                    }
                    last_x_pos = x_pos;
                }
            },

            State::Playback => {
                if mem.is_dead().unwrap() {
                    println!("Cancelling playback - died");
                    state = State::Recording;
                }

                let input = &inputs[current_input];
                if mem.get_x_pos().unwrap() >= input.x_pos {
                    match input.action {
                        Action::Press => UpKey.press(),
                        Action::Release => UpKey.release(),
                    }
                    mem.set_y_pos(input.y_pos).unwrap();

                    current_input += 1;
                    if current_input == inputs.len() {
                        println!("Done playing");
                        state = State::Recording;
                    }
                }
            },

            State::Pause => {},
        }
    }
}