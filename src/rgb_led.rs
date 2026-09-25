/*
    module is meant to control WS2812B RGB LED Strips with the rasp pi 4b using embedded hal
    there is a good example of this here: https://github.com/rpi-ws281x/rpi-ws281x-rust/blob/master/examples/basic.rs
    using this module: https://crates.io/crates/rs_ws281x/0.5.1 (3 years old 🙏😭)
    max brightness is 255 min is 0. defaults to 255 is empty
*/
/*
use rs_ws281x::ChannelBuilder;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

const LED_COUNT: u16 = 148;
const GPIO_SPI0_MOSI_PIN: u8 = 10;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    // White,
    // Yellow,
}

impl Color {
    fn to_rgb(&self) -> [u8; 4] {
        match self {
            // Color::Red    => [0, 0, 255, 0],
            // Color::Green  => [0, 255, 0, 0],
            // Color::Blue   => [255, 0, 0, 0],
            // Color::White  => [255, 255, 255, 0],
            // Color::Yellow => [0, 255, 255, 0],
            Color::Red => [255, 0, 0, 0],
            Color::Green => [0, 255, 0, 0],
            Color::Blue => [0, 0, 255, 0],
        }
    }
}

pub fn set_color(color: Color) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting color to {:?}", color);
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
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
        *led = color.to_rgb();
    }
    println!("Finished setting all LEDs to {:?}", color);

    println!("Colors set to {:?}", color);
    controller.render()?;
    Ok(())
}

*/

use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};

use std::env;

const LED_COUNT: i32 = 148;

const GPIO_PIN: i32 = 10;

const BRIGHTNESS: u8 = 255;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage:");

        eprintln!("  sudo {} <R> <G> <B>", args[0]);

        eprintln!();

        eprintln!("Examples:");

        eprintln!("  sudo {} 255 0 0     # Red", args[0]);

        eprintln!("  sudo {} 0 255 0     # Green", args[0]);

        eprintln!("  sudo {} 0 0 255     # Blue", args[0]);

        eprintln!("  sudo {} 255 255 255 # White", args[0]);

        eprintln!("  sudo {} 0 0 0       # Off", args[0]);

        return Ok(());
    }

    let r: u8 = args[1].parse()?;

    let g: u8 = args[2].parse()?;

    let b: u8 = args[3].parse()?;

    println!("Setting LEDs:");

    println!("R = {}", r);

    println!("G = {}", g);

    println!("B = {}", b);

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(GPIO_PIN)
                .count(LED_COUNT)
                .strip_type(StripType::Ws2812)
                .brightness(BRIGHTNESS)
                .build(),
        )
        .build()?;

    let leds = controller.leds_mut(0);

    for led in leds.iter_mut() {
        // rs_ws281x uses a 4-byte internal pixel representation.

        // Fourth channel remains 0 for our RGB strip.

        *led = [r, g, b, 0];
    }

    controller.render()?;

    println!("Done - {} LEDs updated.", LED_COUNT);

    Ok(())
}
