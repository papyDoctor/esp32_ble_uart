#![no_std]
#![no_main]
mod task_bluetooth;
mod task_led;

use core::panic::PanicInfo;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use esp_alloc as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_println::println;
use task_bluetooth::task_bluetooth;
use task_led::task_led;
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    println!("Main function started");
    // Take peripherals
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timg0: esp_hal::peripherals::TIMG0<'static> = peripherals.TIMG0;
    let rng: esp_hal::peripherals::RNG<'static> = peripherals.RNG;
    let radio_clk: esp_hal::peripherals::RADIO_CLK<'static> = peripherals.RADIO_CLK;
    let gpio8: esp_hal::peripherals::GPIO8<'static> = peripherals.GPIO8;
    let gpio9: esp_hal::peripherals::GPIO9<'static> = peripherals.GPIO9;
    let bluetooth: esp_hal::peripherals::BT<'static> = peripherals.BT;
    // Init embassy timer
    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(systimer.alarm0);

    // Spawn tasks
    spawner.spawn(task_led(gpio8)).unwrap();
    spawner
        .spawn(task_bluetooth(timg0, rng, radio_clk, gpio9, bluetooth))
        .unwrap();

    loop {
        yield_now().await;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic occurred: {:?}", info);
    loop {}
}
