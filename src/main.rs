mod rgb_led;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the LED controller now...");
    rgb_led::set_color(rgb_led::Color::Red)?;
    wait_sec(2);

    rgb_led::set_color(rgb_led::Color::Green)?;
    wait_sec(2);

    rgb_led::set_color(rgb_led::Color::Blue)?;
    wait_sec(2);

    rgb_led::set_color(rgb_led::Color::Yellow)?;
    wait_sec(2);

    rgb_led::set_color(rgb_led::Color::White)?;
    wait_sec(2);

    Ok(())
}

fn wait_sec(secs: u8) {
    std::thread::sleep(std::time::Duration::from_secs(secs as u64));
}