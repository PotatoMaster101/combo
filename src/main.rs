mod args;
mod config;

use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use inputbot::{handle_input_events, KeybdKey};
use crate::args::Args;
use crate::config::*;

fn main() {
    let args = Args::parse();
    let configs = if args.local {
        get_file_config(&args.url).unwrap()
    } else {
        get_network_config(&args.url).unwrap()
    };

    for conf in configs {
        let cl = conf.clone();
        if cl.repeat {
            KeybdKey::from(cl.trigger).bind(move || {
                while check_modifiers(cl.modifiers.as_slice()) && KeybdKey::from(cl.trigger).is_pressed() {
                    press(cl.combo.as_slice(), cl.delay);
                }
            });
        } else {
            KeybdKey::from(cl.trigger).bind(move || {
                if check_modifiers(cl.modifiers.as_slice()) {
                    press(cl.combo.as_slice(), cl.delay);
                }
            });
        }
        println!("Loaded {}...", cl.description);
    }
    handle_input_events();
}

/// Checks all modifier keys are pressed.
///
/// # Arguments
/// - `modifiers`: Specifies the modifier keys to check.
///
/// # Return
/// Whether all modifier keys are pressed.
fn check_modifiers(modifiers: &[u64]) -> bool {
    for m in modifiers {
        if !KeybdKey::from(*m).is_pressed() {
            return false;
        }
    }
    true
}

/// Presses the keys in sequence.
///
/// # Arguments
/// - `keys`: Specifies the keys to press.
/// - `delay`: Specifies the delay between each press.
fn press(keys: &[u64], delay: u64) {
    for k in keys {
        let key = KeybdKey::from(*k);
        key.press();
        sleep(Duration::from_millis(2));
        key.release();
        sleep(Duration::from_millis(delay));
    }
}
