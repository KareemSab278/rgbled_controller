mod rgb_led;
/*
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the LED controller now...");
    rgb_led::set_color(rgb_led::Color::Red)?;
    wait(None);
    
    rgb_led::set_color(rgb_led::Color::Green)?;
    wait(None);
    
    rgb_led::set_color(rgb_led::Color::Blue)?;
    wait(None);
    
    rgb_led::set_color(rgb_led::Color::Yellow)?;
    wait(None);
    
    rgb_led::set_color(rgb_led::Color::White)?;
    wait(None);
    
    Ok(())
}

pub fn wait(secs: Option<u8>) {
    let timeout: u64 = secs.unwrap_or(6) as u64;
    std::thread::sleep(std::time::Duration::from_secs(timeout));
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
