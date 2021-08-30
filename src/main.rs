use streamdeck::{Colour, StreamDeck};

fn main() {
    let mut m = StreamDeck::connect(0x0fd9, 0x0060, None).unwrap();
    dbg!(&m.serial());
    m.set_brightness(20).unwrap();
    m.set_button_rgb(0, &Colour { r: 255, g: 0, b: 0 }).unwrap();
    println!("Hello, world!");
}
