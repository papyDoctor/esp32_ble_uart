use core::marker::Sized;
use embassy_executor;
use esp_hal::gpio::{Level, Output};
use esp_println::println;

#[embassy_executor::task]
pub async fn task_led(gpio8: esp_hal::peripherals::GPIO8<'static>) {
    println!("task_led started");
    // Configure GPIO8 as push-pull output for LED
    let mut led = Output::new(gpio8, Level::Low, esp_hal::gpio::OutputConfig::default());
    loop {
        led.toggle();
        embassy_time::Timer::after_millis(500).await;
    }
}
