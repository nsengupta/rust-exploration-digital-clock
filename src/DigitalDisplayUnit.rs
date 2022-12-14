#![allow(non_snake_case)]

use crate::LedRepresentation::Led;
pub struct Nibbles(pub u8);

//#[derive(Debug)]

const LED_A_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: A + C + B.D + ~B.~D

    (input & 0b00001000 == 0x08)              // *nibble_a == 1u8
        || (input & 0b00000010 == 0x02)       // *nibble_c == 1u8
        || (input & 0b00000101 == 0x05)       // (*nibble_b == 1u8 && *nibble_d == 1u8)
        || (input & 0b00000101 ==  0x00)      // (*nibble_b == 0u8 && *nibble_d == 0u8)
};

const LED_B_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: ~B + ~C~D + CD

    (input & 0b00000100 == 0x00)  // *nibble_b == 0u8
        || (input & 0b00000011 == 0x00) //(*nibble_c == 0u8 && *nibble_d == 0u8)
        || (input & 0b00000011 == 0x03) // (*nibble_c == 1u8 && *nibble_d == 1u8)

};

const LED_C_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: B + ~C + D

    (input & 0b00000100 == 0x04) // (*nibble_b == 1u8
      || (input & 0b00000010 == 0x00) // *nibble_c == 0u8
      || (input & 0b00000001 == 0x01) // *nibble_d == 1u8)

};

const LED_D_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: A + ~B~D + ~BC + C~D + B~CD

    (input & 0b00001000 == 0x08) // *nibble_a == 1u8
        || (input & 0b00000101 == 0x00) // (*nibble_b == 0u8 && *nibble_d == 0u8)
        || (input & 0b00000110 == 0x02) // (*nibble_b == 0u8 && *nibble_c == 1u8)
        || (input & 0b00000011 == 0x02) // (*nibble_c == 1u8 && *nibble_d == 0u8)
        || (input & 0b00000111 == 0x05) // (*nibble_b == 1u8 && *nibble_c == 0u8 && *nibble_d == 1u8)

};

const LED_E_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: ~B~D + C~D

    (input & 0b00000101 == 0x00) // (*nibble_b == 0u8 && *nibble_d == 0u8)
        || (input & 0b00000011 == 0x02) // (*nibble_c == 1u8 && *nibble_d == 0u8)

};

const LED_F_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: A + B~C + B~D + ~C~D

    (input & 0b00001000 == 0x08) // *nibble_a == 1u8
        || (input & 0b00000110 == 0x04) // (*nibble_b == 1u8 && *nibble_c == 0u8)
        || (input & 0b00000101 == 0x04) // (*nibble_b == 1u8 && *nibble_d == 0u8)
        || (input & 0b00000011 == 0x00) // (*nibble_c == 0u8 && *nibble_d == 0u8)

};

const LED_G_GATE_LOGIC: fn(&u8) -> bool = | input: &u8 | {
    // 8-bits and BCD (MSBs start from leftmost)
    // 0       0       0       0       A       B      C      D
    // Using karnaugh Map and don't care conditions: A + B~C + ~BC + C~D

    (input & 0b00001000 == 0x08) // *nibble_a == 1u8
        || (input & 0b00000110 == 0x04) // (*nibble_b == 1u8 && *nibble_c == 0u8)
        || (input & 0b00000110 == 0x02) // (*nibble_b == 0u8 && *nibble_c == 1u8)
        || (input & 0b00000011 == 0x02) // (*nibble_c == 1u8 && *nibble_d == 0u8)

};

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
        let leda = Led::new("a", "????????????", "    ", LED_A_GATE_LOGIC);
        let ledb = Led::new("b", " ???", "  ",     LED_B_GATE_LOGIC);
        let ledc = Led::new("c", " ???", "  ",     LED_C_GATE_LOGIC);
        let ledd = Led::new("d", "????????????", "    ", LED_D_GATE_LOGIC);
        let lede = Led::new("e", "??? ", "  ",     LED_E_GATE_LOGIC);
        let ledf = Led::new("f", "??? ", "  ",     LED_F_GATE_LOGIC);
        let ledg = Led::new("g", "????????????", "    ", LED_G_GATE_LOGIC);

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

    pub fn on_arrival_of_next_signal(&mut self, nibbles_of_BCD: &Nibbles) -> () {

        self.led_a.flip_led(&nibbles_of_BCD.0);
        self.led_b.flip_led(&nibbles_of_BCD.0);
        self.led_c.flip_led(&nibbles_of_BCD.0);
        self.led_d.flip_led(&nibbles_of_BCD.0);
        self.led_e.flip_led(&nibbles_of_BCD.0);
        self.led_f.flip_led(&nibbles_of_BCD.0);
        self.led_g.flip_led(&nibbles_of_BCD.0);

        ()
    }

    pub fn get_led_a(&self) -> &str {
        self.led_a.how_to_display()
    }

    pub fn get_led_b(&self) -> &str {
        self.led_b.how_to_display()
    }

    pub fn get_led_c(&self) -> &str {
        self.led_c.how_to_display()
    }

    pub fn get_led_d(&self) -> &str {
        self.led_d.how_to_display()
    }

    pub fn get_led_e(&self) -> &str {
        self.led_e.how_to_display()
    }

    pub fn get_led_f(&self) -> &str { self.led_f.how_to_display() }

    pub fn get_led_g(&self) -> &str {
        self.led_g.how_to_display()
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
        let nibbles = Nibbles(0u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
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
        let nibbles = Nibbles(0u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_0_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_1() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(1u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_1_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_2() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(2u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_2_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_3() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(3u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_3_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_4() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(4u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_4_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_5() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(5u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_5_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_6() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(6u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_6_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_7() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(7u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_7_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_8() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(8u8);
        digital_display_unit.on_arrival_of_next_signal(&nibbles);
        assert_eq!(is_digit_8_displayable(&digital_display_unit),true);
    }

    #[test]
    fn on_receiving_signal_for_digit_9() -> () {
        let mut digital_display_unit = DigitDisplayUnit::new();
        let nibbles = Nibbles(9u8);
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
        /*println!("BCD {:b}",7u8);

        println!("a: {}", digital_display_unit.led_a.isOn());
            println!("b: {}",  digital_display_unit.led_b.isOn());
            println!("c: {}",  digital_display_unit.led_c.isOn());
            println!("d: {}",  digital_display_unit.led_d.isOn());
            println!("e: {}",  digital_display_unit.led_e.isOn());
            println!("f: {}",  digital_display_unit.led_f.isOn());
            println!("g: {}",  digital_display_unit.led_g.isOn());*/

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
}
