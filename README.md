This application emulates a digital clock, showing current hour, minute and 
second continually. Each hour, minute and second is represented in a regular 
2-digit form. Every digit is represented as a 7-segment display (also called 
LED). With every second elapsed on wall-clock, a BCD input is sent to each of these 7-segment 
display units. A boolean logic derived from Karnaugh-map drives the logic to 
light or unlight each of these 7 segments.


#### Nibble

This is a struct which encapsulates an _unsigned 8 (u8)_ . LSB 4 of these 8 
bits capture the Binary Coded Decimal inputs required to light or unlight a 
LED. Of course,
an 
_u8_ can be used instead, but having a type that represents pins of a 
digital decoder, helps.

#### LedRepresentation

Each LED has a name (industry standard for segment displays) and an 
associated boolean logic that determines if the LED should remain lit or 
unlit. In the actual hardware, this is done by the decoder. Because we are 
emulating a standard screen, each LED here has a pair of strings: one for 
showing that the LED is lit, and the other for showing that it remains unlit.
This code uses ASCII charcters (refer to the constructor calls).

#### DigitalDisplayUnit

A DisplayUnit is an aggregation of 7 LEDs. It receives the BCD input as a 
`Nibble` and then propagates the input to all the LEDs, letting them decide 
by themselves whether to remain lit or unlit:

```rust
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
```

#### ScreenClock

This is the master, root type. A ScreenClock is composed of 6 
`DigitalDisplayUnit`s. The ScreenClock is responsible for displaying the 
digits, at correct position on the screen. For ease of display, it inquires 
with every `DigitalDiplayUnit`'s LEDs by name:

```rust
self.display_units[0].get_led_a()
```
This is not very clean, of course. Ideally, the `ScreenClock` should simply 
tell a `DigitalDisplayUnit`, to show itself at a certain row/col. That way, 
the step to check the lit/unlit status of every LED could be left to the 
`DigitalDisplayUnit` and its inner working, thereby obviating intrusive 
calls like `get_led_a()`. The current design is somewhat lazy and unclean, 
from this PoV.

#### Handling asynchronous inputs

In `main` , Rust's _channel_ facility is used to allow asynchronous 
communication between threads to take place. Every _channel_ has a _send_ 
and a _receive_ end. `main` thread intends to be notified about either of 
the following two:

* 1 second has elapsed by the wall-clock time (OS's own clock)
* The user wants to exit from this _ScreenClock_ application

It is unknown which of these two happens and in what order. Therefore, 
`main` sets itself up for listening to both of these but reacting to only 
one of them, at a time.

Again, this portion of the code can be strengthened, to take care of edge 
conditions. In a real-life application that interacts with the H/W, such a 
need will certainly arise. This application is an emulation and is not so 
exhaustive in its handling of different conditions.

##### Truth Table for BCD and Don't Care condition

The truth table for BCD to Segment display is here:

| Input bit 3 (A) | Input bit 2 (B) | Input bit 1 (C) | Input bit 0 (D) | Digit displayed |
|-----------------|-----------------|-----------------|-----------------|-----------------|
| 0               | 0               | 0               | 0               | 0               |
| 0               | 0               | 0               | 1               | 1               |
| 0               | 0               | 1               | 0               | 2               |
| 0               | 0               | 1               | 1               | 3               |
| 0               | 1               | 0               | 0               | 4               |
| 0               | 1               | 0               | 1               | 5               |
| 0               | 1               | 1               | 0               | 6               |
| 0               | 1               | 1               | 1               | 7               |
| 1               | 0               | 0               | 0               | 8               |
| 1               | 0               | 0               | 1               | 9               |
| 1               | 0               | 1               | 0               | X               |


The last row shows, that for such an input, LEDs have to do nothing. These 
are **Don't Care** conditions.

For a quick refresher on BCD to 7 segment Decoding Boolean logic, pleaser 
refer to the following:

* https://www.101computing.net/bcd-to-7-segment-display/

* https://www.electrical4u.com/bcd-to-seven-segment-decoder/


