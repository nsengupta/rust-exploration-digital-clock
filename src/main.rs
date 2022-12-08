#![allow(non_snake_case)]
use crate::DigitalDisplayUnit::DigitDisplayUnit;
use crate::DigitalDisplayUnit::Nibbles;


mod DigitalDisplayUnit;
mod LedRepresentation;
mod ScreenClock;


pub fn main() {

    print!("\x1b[H\x1b[J\x1b[12;34H\x1b[1;31mHello,World!\x1b[22;39mNo More bright Red.");//println!("\x1b[31mHello"); println!("\x1b[m, world!");

    // print!("\x1b[?25l");



    let mut digital_display_unit = DigitDisplayUnit::new();

    let digits_from_clock = ['1' as u8,'0' as u8, '2' as u8, '3' as u8];

    let nibbles = Nibbles(0,1,1,0);

    // let &mut digital_display_unit = DigitDisplayUnit::new();

    digital_display_unit.on_arrival_of_next_signal(&nibbles);

    digital_display_unit.show(13,40);

    println!(); println!();

}

