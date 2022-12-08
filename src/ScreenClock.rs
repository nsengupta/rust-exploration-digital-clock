#![allow(non_snake_case)]

use crate::{DigitDisplayUnit, Nibbles};

#[derive(Debug)]

pub struct ScreenClock {
    top_left_row: u8,
    top_left_col: u8,
    display_units: [DigitDisplayUnit;4],
}

impl ScreenClock {
    pub fn new(top_left_row: u8, top_left_col: u8) -> ScreenClock {

        let digital_display_unit0 = DigitDisplayUnit::new();
        let digital_display_unit1 = DigitDisplayUnit::new();
        let digital_display_unit2 = DigitDisplayUnit::new();
        let digital_display_unit3 = DigitDisplayUnit::new();

        ScreenClock {
            top_left_row,
            top_left_col,
            display_units: [digital_display_unit0, digital_display_unit1, digital_display_unit2, digital_display_unit3]
        }
    }

    pub fn on_next_clock_tick(&mut self, min_1: &u8, min_2: &u8, sec_1: &u8, sec_2: &u8) {

          let mins_and_secs_arrayfied: [&u8;4] = [min_1,min_2,sec_1,sec_2];


    }

    fn convert_to_nibbles(digits_in_clock_tick:&[u8;4]) -> Nibbles {
        let mm_and_ss = digits_in_clock_tick.map(|d| d - '0' as u8);
        Nibbles (mm_and_ss[0],mm_and_ss[1],mm_and_ss[2],mm_and_ss[3])
    }

    fn digit_to_nibbles(digit: &u8) -> Nibbles {
        panic!()
    }
}