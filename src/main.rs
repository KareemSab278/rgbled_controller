mod rgb_led;

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