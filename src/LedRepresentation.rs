#![allow(non_snake_case)]
#[derive(Debug)]

// Datatype that captures a Led
pub struct Led {
    name: String,
    show_character: String,
    hide_character: String ,
    lightStatus: bool,
    rowFromOrigin: i8, // row position relative to a top-left corner position for display
    colFromOrigin: i8, // col position relative to a top-left corner position for display
}
impl Led {
    pub fn new(name: &str, displayChar: &str, rowFromOrigin: i8, colFromOrigin: i8) -> Led {
        Led {
            name: name.to_string(),
            show_character: displayChar.to_string(),
            hide_character: " ".to_string(),
            lightStatus: false,
            rowFromOrigin: rowFromOrigin,
            colFromOrigin: colFromOrigin,
        }
    }

    pub fn switch_on(&mut self) -> () {
        self.lightStatus = true;
    }

    pub fn isOn(&self) -> bool {
        self.lightStatus
    }

    pub fn display_location(&self) -> (u8, u8, &str) {
        let what_to_show = if !self.lightStatus { &self.hide_character } else { &self.show_character};
        (self.rowFromOrigin as u8, self.colFromOrigin as u8, what_to_show.as_str())
    }
}
