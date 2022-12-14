#![allow(non_snake_case)]
//#[derive(Debug)]

// Datatype that captures a Led
pub struct Led {
    name: String,
    show_character: String,
    hide_character: String ,
    light_status: bool,
    // Closure that evaluates a BCD-signal, before deciding of the LED should be (un)lit
    on_receiving_next_signal: fn(&u8) -> bool
}
impl Led {
    pub fn new(name: &str, displayChar: &str, hide_character: &str, evaluator: fn(&u8) -> bool) -> Led {
        Led {
            name: name.to_string(),
            show_character: displayChar.to_string(),
            hide_character: hide_character.to_string(),
            light_status: false,
            on_receiving_next_signal: evaluator
        }
    }

    pub fn flip_led(&mut self, signal_as_BCD: &u8) -> () {
        if (self.on_receiving_next_signal)(signal_as_BCD)
            { self.light_status = true;  }
        else
            { self.light_status = false; }
    }

    pub fn isOn(&self) -> bool {
        self.light_status == true
    }

    pub fn how_to_display(&self) -> &str {
        if self.light_status == true { &self.show_character } else { &self.hide_character }
    }

}
