/*
    module is meant to control WS2812B RGB LED Strips with the rasp pi 4b using embedded hal
    there is a good example of this here: https://github.com/rpi-ws281x/rpi-ws281x-rust/blob/master/examples/basic.rs
    using this module: https://crates.io/crates/rs_ws281x/0.5.1 (3 years old 🙏😭)
    max brightness is 255 min is 0. defaults to 255 is empty
*/

use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;

const LED_COUNT: u16 = 160*5;
const GPIO_SPI0_MOSI_PIN: u8 = 10;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Yellow,
}

impl Color {
    fn to_rgbw(&self) -> [u8; 4] {
        match self {
            Color::Red => [255, 0, 0, 0],
            Color::Green => [0, 255, 0, 0],
            Color::Blue => [0, 0, 255, 0],
            Color::White => [0, 0, 0, 255],
            Color::Yellow => [255, 255, 0, 0],
        }
    }
}

pub fn set_color(color: Color) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting color to {:?}", color);
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(5)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(GPIO_SPI0_MOSI_PIN as i32)
                .count(LED_COUNT as i32)
                .strip_type(StripType::Ws2812)
                .brightness(255)
                .build(),
        )
        .build()
        .expect("Failed to build LED controller");

    let leds = controller.leds_mut(0);
    println!("Number of LEDs: {}", leds.len());

    for led in leds.iter_mut() {
        *led = color.to_rgbw();
    }
    println!("Finished setting all LEDs to {:?}", color);

    println!("Colors set to {:?}", color);
    controller.render()?;
    Ok(())
}