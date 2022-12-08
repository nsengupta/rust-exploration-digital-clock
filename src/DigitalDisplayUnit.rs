#![allow(non_snake_case)]

use crate::LedRepresentation::Led;
pub struct Nibbles(pub u8, pub u8, pub u8, pub u8);

// https://www.electricaltechnology.org/2018/05/bcd-to-7-segment-display-decoder.html

#[derive(Debug)]

pub struct DigitDisplayUnit {
    // TODO: These leds should be in a map, identifiable  by the letter associated
    led_a: Led,
    led_b: Led,
    led_c: Led,
    led_d: Led,
    led_e: Led,
    led_f: Led,
    led_g: Led,
}

impl DigitDisplayUnit {
    pub fn new() -> DigitDisplayUnit {
        let leda = Led::new("a", "┏━┓", 0i8, 1i8);
        let ledb = Led::new("b", "┃", 1i8, 3i8);
        let ledc = Led::new("c", "┃", 3i8, 3i8);
        let ledd = Led::new("d", "┗━┛", 4i8, 1i8);
        let lede = Led::new("e", "┃", 3i8, 1i8);
        let ledf = Led::new("f", "┃", 1i8, 1i8);
        let ledg = Led::new("g", "┣━┫", 2i8, 1i8);

        DigitDisplayUnit {
            led_a: leda,
            led_b: ledb,
            led_c: ledc,
            led_d: ledd,
            led_e: lede,
            led_f: ledf,
            led_g: ledg,
        }
    }

    pub fn show(&self, row_origin: u8, col_origin: u8) -> () {

       // print!("\x1b")

        let led_a_color_and_position_and_character = if self.led_a.isOn() { (31 /* ANSI Red */,self.led_a.display_location()) } else { (39 /* ANSI Default */,self.led_a.display_location()) };
        let led_b_color_and_position_and_character = if self.led_b.isOn() { (31 /* ANSI Red */,self.led_b.display_location()) } else { (39 /* ANSI Default */,self.led_b.display_location()) };
        let led_c_color_and_position_and_character = if self.led_c.isOn() { (31 /* ANSI Red */,self.led_c.display_location()) } else { (39 /* ANSI Default */,self.led_c.display_location()) };
        let led_d_color_and_position_and_character = if self.led_d.isOn() { (31 /* ANSI Red */,self.led_d.display_location()) } else { (39 /* ANSI Default */,self.led_d.display_location()) };
        let led_e_color_and_position_and_character = if self.led_e.isOn() { (31 /* ANSI Red */,self.led_e.display_location()) } else { (39 /* ANSI Default */,self.led_e.display_location()) };
        let led_f_color_and_position_and_character = if self.led_f.isOn() { (31 /* ANSI Red */,self.led_f.display_location()) } else { (39 /* ANSI Default */,self.led_f.display_location()) };
        let led_g_color_and_position_and_character = if self.led_g.isOn() { (31 /* ANSI Red */,self.led_g.display_location()) } else { (39 /* ANSI Default */,self.led_g.display_location()) };

        //  \x1b[1;31mHello,World!\x1b[22;39mNo More bright Red.
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_a_color_and_position_and_character.1.0) ,(col_origin + led_a_color_and_position_and_character.1.1), led_a_color_and_position_and_character.0, led_a_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_b_color_and_position_and_character.1.0) ,(col_origin + led_b_color_and_position_and_character.1.1), led_b_color_and_position_and_character.0, led_b_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_c_color_and_position_and_character.1.0) ,(col_origin + led_c_color_and_position_and_character.1.1), led_c_color_and_position_and_character.0, led_c_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_d_color_and_position_and_character.1.0) ,(col_origin + led_d_color_and_position_and_character.1.1), led_d_color_and_position_and_character.0, led_d_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_e_color_and_position_and_character.1.0) ,(col_origin + led_e_color_and_position_and_character.1.1), led_e_color_and_position_and_character.0, led_e_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_f_color_and_position_and_character.1.0) ,(col_origin + led_f_color_and_position_and_character.1.1), led_f_color_and_position_and_character.0, led_f_color_and_position_and_character.1.2);
        print!("\x1b[{};{}H\x1b[{}m{}", (row_origin + led_g_color_and_position_and_character.1.0) ,(col_origin + led_g_color_and_position_and_character.1.1), led_g_color_and_position_and_character.0, led_g_color_and_position_and_character.1.2);

    }

    pub fn on_arrival_of_next_signal(&mut self, nibbles: &Nibbles) -> () {
        let Nibbles(nibble_a, nibble_b, nibble_c, nibble_d) = nibbles;

        if DigitDisplayUnit::should_be_lit_led_a(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_a.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_b(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_b.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_c(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_c.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_d(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_d.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_e(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_e.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_f(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_f.switch_on()
        };
        if DigitDisplayUnit::should_be_lit_led_g(&nibble_a, &nibble_b, &nibble_c, &nibble_d) {
            self.led_g.switch_on()
        };

        ()
    }

    fn should_be_lit_led_a(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: A + C + BD + ~B~D

        *nibble_a == 1u8
            || *nibble_c == 1u8
            || (*nibble_b == 1u8 && *nibble_d == 1u8)
            || (*nibble_b == 0u8 && *nibble_d == 0u8)
    }

    fn should_be_lit_led_b(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: ~B + ~C~D + CD

        *nibble_b == 0u8
            || (*nibble_c == 0u8 && *nibble_d == 0u8)
            || (*nibble_c == 1u8 && *nibble_d == 1u8)

    }

    fn should_be_lit_led_c(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: B + ~C + D

        (*nibble_b == 1u8 || *nibble_c == 0u8 || *nibble_d == 1u8)
    }

    fn should_be_lit_led_d(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: A + ~B~D + ~BC + C~D + B~CD

        *nibble_a == 1u8
            || (*nibble_b == 0u8 && *nibble_d == 0u8)
            || (*nibble_b == 0u8 && *nibble_c == 1u8)
            || (*nibble_c == 1u8 && *nibble_d == 0u8)
            || (*nibble_b == 1u8 && *nibble_c == 0u8 && *nibble_d == 1u8)
    }

    fn should_be_lit_led_e(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: ~B~D + C~D

        (*nibble_b == 0u8 && *nibble_d == 0u8) || (*nibble_c == 1u8 && *nibble_d == 0u8)
    }

    fn should_be_lit_led_f(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: A + B~C + B~D + ~C~D

        *nibble_a == 1u8
            || (*nibble_b == 1u8 && *nibble_c == 0u8)
            || (*nibble_b == 1u8 && *nibble_d == 0u8)
            || (*nibble_c == 0u8 && *nibble_d == 0u8)
    }

    fn should_be_lit_led_g(nibble_a: &u8, nibble_b: &u8, nibble_c: &u8, nibble_d: &u8) -> bool {
        // Using karnaugh Map and don't care conditions: A + B~C + ~BC + C~D

        *nibble_a == 1u8
            || (*nibble_b == 1u8 && *nibble_c == 0u8)
            || (*nibble_b == 0u8 && *nibble_c == 1u8)
            || (*nibble_c == 1u8 && *nibble_d == 0u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn led_switching_works() {
        let mut digital_display_unit = DigitDisplayUnit::new();

        // No signal till now, all leds should be off!
        assert_eq!(
            !digital_display_unit.led_a.isOn()
                && !digital_display_unit.led_b.isOn()
                && !digital_display_unit.led_c.isOn()
                && !digital_display_unit.led_d.isOn()
                && !digital_display_unit.led_e.isOn()
                && !digital_display_unit.led_f.isOn()
                && !digital_display_unit.led_g.isOn(),
            true
        );
        let nibbles = Nibbles(0, 0, 0, 0);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);

        // assert_eq!(DigitDisplayUnit::should_be_lit_led_g(&nibbles.0,&nibbles.1,&nibbles.2,&nibbles.3),false);
        assert_eq!(
            digital_display_unit.led_a.isOn()
                && digital_display_unit.led_b.isOn()
                && digital_display_unit.led_c.isOn()
                && digital_display_unit.led_d.isOn()
                && digital_display_unit.led_e.isOn()
                && digital_display_unit.led_f.isOn()
                && !digital_display_unit.led_g.isOn(),
            true
        );
    }

    #[test]
    fn on_receiving_signal_for_digit_0() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(0);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_0_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_1() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(1);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_1_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_2() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(2);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_2_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_3() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(3);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_3_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_4() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(4);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_4_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_5() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(5);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_5_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_6() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(6);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_6_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_7() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(7);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_7_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_8() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_8_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_9() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = generate_signal_for_given_digit(9);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_9_displayable(&digital_display_unit),true);
    }

    fn is_digit_0_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && !digital_display_unit.led_g.isOn()
    }

    fn is_digit_1_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        !digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && !digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && !digital_display_unit.led_f.isOn()
            && !digital_display_unit.led_g.isOn()
    }

    fn is_digit_2_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1101101
        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && !digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && digital_display_unit.led_e.isOn()
            && !digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_3_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1111001
        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && !digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_4_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 0110011

        !digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && !digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_5_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1011011

        digital_display_unit.led_a.isOn()
            && !digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_6_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1011111

        digital_display_unit.led_a.isOn()
            && !digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_7_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1110000

        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && !digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && !digital_display_unit.led_f.isOn()
            && !digital_display_unit.led_g.isOn()
    }

    fn is_digit_8_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1111111

        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn is_digit_9_displayable(digital_display_unit: &DigitDisplayUnit) -> bool {
        // 1111011

        digital_display_unit.led_a.isOn()
            && digital_display_unit.led_b.isOn()
            && digital_display_unit.led_c.isOn()
            && digital_display_unit.led_d.isOn()
            && !digital_display_unit.led_e.isOn()
            && digital_display_unit.led_f.isOn()
            && digital_display_unit.led_g.isOn()
    }

    fn generate_signal_for_given_digit(d: i8) -> Nibbles {

        match d {
            0 => Nibbles(0,0,0,0),
            1 => Nibbles(0,0,0,1),
            2 => Nibbles(0,0,1,0),
            3 => Nibbles(0,0,1,1),
            4 => Nibbles(0,1,0,0),
            5 => Nibbles(0,1,0,1),
            6 => Nibbles(0,1,1,0),
            7 => Nibbles(0,1,1,1),
            8 => Nibbles(1,0,0,0),
            9 => Nibbles(1,0,0,1),
            _ => Nibbles(1,1,1,1),
        }
    }
}
