/*
    module controls WS2812B RGB LED strips on Raspberry Pi 5 using SPI
    SPI10 MOSI is used because GPIO10 (SPI0 MOSI) is occupied.

    DIN is wired to GPIO21 (physical pin 40)
*/

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use smart_leds_trait::{RGB8, SmartLedsWrite};
use ws2812_spi::Ws2812;

const LED_COUNT: usize = 148;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Yellow,
}

impl Color {
    fn to_rgb8(&self) -> RGB8 {
        match self {
            Color::Red => RGB8 { r: 255, g: 0, b: 0 },
            Color::Green => RGB8 { r: 0, g: 255, b: 0 },
            Color::Blue => RGB8 { r: 0, g: 0, b: 255 },
            Color::White => RGB8 {
                r: 255,
                g: 255,
                b: 255,
            },
            Color::Yellow => RGB8 {
                r: 255,
                g: 255,
                b: 0,
            },
        }
    }
}

pub fn set_color(color: Color) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting color to {:?}", color);

    let spi = Spi::new(Bus::Spi10, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)?;

    let mut controller = Ws2812::new(spi);

    let leds = vec![color.to_rgb8(); LED_COUNT];

    controller.write(leds.into_iter())?;

    println!("Colors set to {:?}", color);

    Ok(())
}
