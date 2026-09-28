/*
    module is meant to control WS2812B RGB LED Strips with the rasp pi 4b using embedded hal
    there is a good example of this here: https://github.com/rpi-ws281x/rpi-ws281x-rust/blob/master/examples/basic.rs
    using this module: https://crates.io/crates/rs_ws281x/0.5.1 (3 years old 🙏😭)
    max brightness is 255 min is 0. defaults to 255 is empty

    run with cargo build && sudo ./target/debug/rgbled_controller when using the gpio pin 18
*/

use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};

const LED_COUNT: u16 = 148;
const GPIO_SPI0_MOSI_PIN: u8 = 18; // you can use gpio pin 10 which is better but it is currently occupied

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
}

impl Color {
    /*
        Returns the LED bytes for this color.
        The WS2812B strip here interprets the array as `[B, G, R, W]`
     */
    
    fn to_led_bytes(&self) -> [u8; 4] {
        match self {
            Color::Red   => [0, 0, 255, 0],         // R on 3rd byte
            Color::Green => [0, 255, 0, 0],         // G on 2nd byte
            Color::Blue  => [255, 0, 0, 0],         // B on 1st byte
            Color::White => [255, 255, 255, 0],     // all RGB channels on
        }
    }
}

pub fn set_color(color: Color) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing {:?}", color);

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(GPIO_SPI0_MOSI_PIN as i32)
                .count(LED_COUNT as i32)
                .strip_type(StripType::Ws2812)
                .brightness(32)
                .build(),
        )
        .build()
        .expect("Failed to build LED controller");

    let bytes = color.to_led_bytes();
    println!("Sending LED bytes {:?}", bytes);

    let leds = controller.leds_mut(0);
    for led in leds.iter_mut() {
        *led = bytes;
    }

    controller.render()?;
    Ok(())
}