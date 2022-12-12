#![allow(non_snake_case)]



use chrono::Local;
use crate::DigitalDisplayUnit::DigitDisplayUnit;
use crate::DigitalDisplayUnit::Nibbles;


mod ScreenClock;
mod DigitalDisplayUnit;
mod LedRepresentation;


// mod Scree::new()nLockExperimentation;


pub fn main() {

    // print!("\x1b[H\x1b[J\x1b[12;34H\x1b[1;31mHello,World!\x1b[22;39mNo More bright Red.");//println!("\x1b[31mHello"); println!("\x1b[m, world!");

    print!("\x1b[?25l");

    let mut screen_clock = ScreenClock::ScreenClock::new(12,35);
    let mut digital_display_unit = DigitDisplayUnit::new();

    loop {

        let t = Local::now();
        let time = t.format("%M:%S").to_string();

        let min_and_sec : Vec<u8> = time
            .chars()
            .filter( | c  | c != &':')
            .map(| c | c as u8 - '0' as u8)
            .collect();

        // println!("time {}", time);

        screen_clock.on_next_clock_tick(min_and_sec[0],min_and_sec[1],min_and_sec[2],min_and_sec[3]).refresh();
        std::thread::sleep(std::time::Duration::from_millis(99));
        print!("\x1b[7A");
    }


    // screen_clock.on_next_clock_tick(0,0,3,6).refresh();

    println!();println!(); println!(); println!();

}

