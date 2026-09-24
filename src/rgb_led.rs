/*
    module is meant to control WS2812B RGB LED Strips with the rasp pi 5 using embedded hal
    there is a good example of this here: https://github.com/rpi-ws281x/rpi-ws281x-rust/blob/master/examples/basic.rs
    using this module: https://crates.io/crates/rs_ws281x/0.5.1 (3 years old 🙏😭)
    max brightness is 255 min is 0. defaults to 255 is empty
*/

use rs_ws281x::ControllerBuilder;
use rs_ws281x::ChannelBuilder;
use rs_ws281x::StripType;

const LED_COUNT: u8 = 64;
const GPIO_SPI0_MOSI_PIN: u8 = 16;

struct Color {
    Red: [u8; 4],
    Green: [u8; 4],
    Blue: [u8; 4],
    White: [u8; 4],
    Yellow: [u8; 4],
}

impl Color {
    fn color(&self) -> [u8; 4] {
        match self {
            Color::Red => [255, 0, 0, 0],
            Color::Green => [0, 255, 0, 0],
            Color::Blue => [0, 0, 255, 0],
            Color::White => [0, 0, 0, 255],
            Color::Yellow => [255, 255, 0, 0],
        }
    }
}


fn set_color(&self, color: Color) {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(GPIO_SPI0_MOSI_PIN)
                .count(LED_COUNT)
                .strip_type(StripType::Ws2812)
                .brightness(255)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = color.color();
    }

    controller.render().unwrap();
}