use core::marker::Sized;
use embassy_executor;

#[embassy_executor::task]
pub async fn task_led(gpio8: esp_hal::peripherals::GPIO8<'static>) {
    let mut led = esp_hal::gpio::Output::new(
        gpio8,
        esp_hal::gpio::Level::Low,
        esp_hal::gpio::OutputConfig::default(),
    );
    loop {
        led.toggle();
        embassy_time::Timer::after_millis(50000).await;
    }
}
