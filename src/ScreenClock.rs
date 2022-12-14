#![allow(non_snake_case)]

use crate::{DigitDisplayUnit, Nibbles};

//#[derive(Debug)]

// const CLOCK_ROW_0_FORMAT: &str = "   {}    {}    {}    {}";
// const CLOCK_ROW_1_FORMAT: &str = "  {} {} {} {} {} {} {} {}";
// const CLOCK_ROW_2_FORMAT: &str = "   {}    {}    {}    {}";
// const CLOCK_ROW_3_FORMAT: &str = "  {} {} {} {} {} {} {} {}";
// const CLOCK_ROW_4_FORMAT: &str = "   {}    {}    {}    {}";

pub struct ScreenClock {
    top_left_row: u8,
    top_left_col: u8,
    display_units: [DigitDisplayUnit;6],
}

impl ScreenClock {
    pub fn new( start_at_row: u8, start_at_col: u8 ) -> ScreenClock {

        // From left to right, on the display panel!
        let digital_display_unit0 = DigitDisplayUnit::new(); // h_ of hh
        let digital_display_unit1 = DigitDisplayUnit::new(); // _h of hh
        let digital_display_unit2 = DigitDisplayUnit::new(); // m_ of mm
        let digital_display_unit3 = DigitDisplayUnit::new(); // _m of mm
        let digital_display_unit4 = DigitDisplayUnit::new(); // s_ of ss
        let digital_display_unit5 = DigitDisplayUnit::new(); // _s of ss


        ScreenClock {
            top_left_row: start_at_row,
            top_left_col: start_at_col,
            display_units: [
                digital_display_unit0,
                digital_display_unit1,
                digital_display_unit2,
                digital_display_unit3,
                digital_display_unit4,
                digital_display_unit5,
            ]
        }
    }

    pub fn on_next_second(&mut self, hh_1: u8, hh_2: u8, min_1: u8, min_2: u8, sec_1: u8, sec_2: u8) -> &mut ScreenClock {

          // One DisplayUnit exists for each digit, and there are 6 digits!
          let mins_and_secs_arrayfied: [u8;6] = [hh_1,hh_2,min_1,min_2,sec_1,sec_2];
          for i in 0..6 {
              self.display_units[i].on_arrival_of_next_signal(&Nibbles(mins_and_secs_arrayfied[i]));
          }
          self

    }

    pub fn refresh(&self) -> &ScreenClock {

        let all_columns_of_row = self.prepare_clock_for_display();

        print!("\x1b[{};{}H\x1b[1;33m{}", &self.top_left_row ,   &self.top_left_col , &all_columns_of_row[0]);
        print!("\x1b[{};{}H\x1b[1;33m{}", self.top_left_row +1, self.top_left_col, all_columns_of_row[1]);
        print!("\x1b[{};{}H\x1b[1;33m{}", self.top_left_row +2, self.top_left_col, all_columns_of_row[2]);
        print!("\x1b[{};{}H\x1b[1;33m{}", self.top_left_row +3, self.top_left_col, all_columns_of_row[3]);
        print!("\x1b[{};{}H\x1b[1;33m{}", self.top_left_row +4, self.top_left_col, all_columns_of_row[4]);

        self
    }

    fn prepare_clock_for_display(&self) -> [String; 5] {

        // @TODO: Explore StrFmt library, to replace hard-coded formatters, below!
        let formatted_row_0 = format!(" {}  {}    {}  {}    {}  {}",
                                      self.display_units[0].get_led_a(),
                                      self.display_units[1].get_led_a(),
                                      self.display_units[2].get_led_a(),
                                      self.display_units[3].get_led_a(),
                                      self.display_units[4].get_led_a(),
                                      self.display_units[5].get_led_a()
        );

        let formatted_row_1 = format!(" {}{}  {}{} o  {}{}  {}{} o  {}{}  {}{}",
                                      self.display_units[0].get_led_f(),self.display_units[0].get_led_b(),
                                      self.display_units[1].get_led_f(),self.display_units[1].get_led_b(),
                                      self.display_units[2].get_led_f(),self.display_units[2].get_led_b(),
                                      self.display_units[3].get_led_f(),self.display_units[3].get_led_b(),
                                      self.display_units[4].get_led_f(),self.display_units[4].get_led_b(),
                                      self.display_units[5].get_led_f(),self.display_units[5].get_led_b()
        );

        let formatted_row_2 = format!(" {}  {}    {}  {}    {}  {}",
                                      self.display_units[0].get_led_g(),
                                      self.display_units[1].get_led_g(),
                                      self.display_units[2].get_led_g(),
                                      self.display_units[3].get_led_g(),
                                      self.display_units[4].get_led_g(),
                                      self.display_units[5].get_led_g()
        );

        let formatted_row_3 = format!(" {}{}  {}{} o  {}{}  {}{} o  {}{}  {}{}",
                                      self.display_units[0].get_led_e(),self.display_units[0].get_led_c(),
                                      self.display_units[1].get_led_e(),self.display_units[1].get_led_c(),
                                      self.display_units[2].get_led_e(),self.display_units[2].get_led_c(),
                                      self.display_units[3].get_led_e(),self.display_units[3].get_led_c(),
                                      self.display_units[4].get_led_e(),self.display_units[4].get_led_c(),
                                      self.display_units[5].get_led_e(),self.display_units[5].get_led_c()
        );

        let formatted_row_4 = format!(" {}  {}    {}  {}    {}  {}",
                                      self.display_units[0].get_led_d(),
                                      self.display_units[1].get_led_d(),
                                      self.display_units[2].get_led_d(),
                                      self.display_units[3].get_led_d(),
                                      self.display_units[4].get_led_d(),
                                      self.display_units[5].get_led_d()
        );

        let row_holder: [String; 5] = [formatted_row_0,formatted_row_1,formatted_row_2,formatted_row_3,formatted_row_4];

        row_holder
    }

}

