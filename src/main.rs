#![allow(non_snake_case)]

use std::time::Duration;
use chrono::Local;
use crossbeam::channel::{bounded};
use crossbeam::{Receiver, select, tick};

use crate::DigitalDisplayUnit::DigitDisplayUnit;
use crate::DigitalDisplayUnit::Nibbles;


mod ScreenClock;
mod DigitalDisplayUnit;
mod LedRepresentation;

pub fn main() {

    print!("\x1b[?25l"); // Hide the cursor!

    let ctrl_c_pressed = notify_me_when_user_exits().unwrap();
    let notification_on_next_second = tick(Duration::from_secs(1));

    let mut screen_clock = ScreenClock::ScreenClock::new(12,35);

    loop {

        select! {
            recv(notification_on_next_second) -> _ => {
                let time = read_clock_now()      // Current HH:MM:SS as a string
                .chars()                         // An iterator of characters it contains
                .filter( | c  | c != &':')       // Scrape the ':' character from the middle
                .map(| c | c as u8 - '0' as u8)  // Get the numeric digit from ascii digit
                .collect();

                screen_clock
                .on_next_second(
                    hr_and_min_and_sec[0],
                    hr_and_min_and_sec[1],
                    hr_and_min_and_sec[2],
                    hr_and_min_and_sec[3],
                    hr_and_min_and_sec[4],
                    hr_and_min_and_sec[5]
                )
                .refresh();

                print!("\x1b[7A");
            }

            recv(ctrl_c_pressed) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }
}

fn notify_me_when_user_exits() -> Result<Receiver<u8>, ctrlc::Error> {
    let (sender, receiver) =  bounded(8); // 8 is arbitrarily chosen, could be 1
    ctrlc::set_handler(move || {
        print!("\x1b[?25h");  // Restore the hidden cursor!
        let _ = sender.send(0xFF);
    })?;

    Ok(receiver)
}

fn read_clock_now() -> String {
    let t = Local::now();
    let time_now = t.format("%H:%M:%S").to_string();
    time_now
}

