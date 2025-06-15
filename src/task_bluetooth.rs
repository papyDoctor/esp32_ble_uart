use bleps::{
    ad_structure::{
        AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE, create_advertising_data,
    },
    async_attribute_server::AttributeServer,
    asynch::Ble,
    attribute_server::NotificationData,
    gatt,
};
use core::cell::RefCell;
use esp_hal::{
    gpio::{Input, InputConfig, Pull},
    rng::Rng,
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{EspWifiController, ble::controller::BleConnector, init};

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[embassy_executor::task]
pub async fn task_bluetooth(
    timg0: esp_hal::peripherals::TIMG0<'static>,
    rng: esp_hal::peripherals::RNG<'static>,
    radio_clk: esp_hal::peripherals::RADIO_CLK<'static>,
    gpio9: esp_hal::peripherals::GPIO9<'static>,
    mut bluetooth: esp_hal::peripherals::BT<'static>,
) {
    println!("task_bluetooth started");
    // Init ESP wifi engine
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let timg0 = TimerGroup::new(timg0);
    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        init(timg0.timer0, Rng::new(rng), radio_clk).unwrap()
    );

    // Take the button pin
    let config = InputConfig::default().with_pull(Pull::Down);
    let button: Input<'_> = Input::new(gpio9, config);

    // Initialize BLE
    let connector = BleConnector::new(&esp_wifi_ctrl, bluetooth.reborrow());

    let now = || time::Instant::now().duration_since_epoch().as_millis();
    let mut ble = Ble::new(connector, now);
    println!("Connector created");

    let pin_ref = RefCell::new(button);
    let pin_ref = &pin_ref;
    loop {
        println!("{:?}", ble.init().await);
        println!("{:?}", ble.cmd_set_le_advertising_parameters().await);
        println!(
            "{:?}",
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids128(&[Uuid::Uuid128([
                        0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5,
                        0x01, 0x00, 0x40, 0x6E,
                    ])]),
                    AdStructure::CompleteLocalName("Fenyx"),
                ])
                .unwrap()
            )
            .await
        );
        println!("{:?}", ble.cmd_set_le_advertise_enable(true).await);
        println!("Started advertising");

        let mut wf = |offset: usize, data: &[u8]| {
            println!("RECEIVED: {} {:?}", offset, data);
        };
        let mut rf = |_offset: usize, data: &mut [u8]| {
            data[..5].copy_from_slice(&b"Hola!"[..]);
            5
        };

        gatt!([service {
            uuid: "6E400001-B5A3-F393-E0A9-E50E24DCCA9E", // Nordic UART Service UUID
            characteristics: [
                characteristic {
                    name: "RX", // phone → device
                    uuid: "6E400002-B5A3-F393-E0A9-E50E24DCCA9E",
                    write: wf,
                },
                characteristic {
                    name: "TX", // device → phone
                    uuid: "6E400003-B5A3-F393-E0A9-E50E24DCCA9E",
                    notify: true,
                    read: rf,
                },
            ],
        }]);

        let mut rng = bleps::no_rng::NoRng;
        let mut srv = AttributeServer::new(&mut ble, &mut gatt_attributes, &mut rng);

        let counter = RefCell::new(0u8);
        let counter = &counter;

        let mut notifier = || {
            // TODO how to check if notifications are enabled for the characteristic?
            // maybe pass something into the closure which just can query the characteristic
            // value probably passing in the attribute server won't work?

            async {
                pin_ref.borrow_mut().wait_for_rising_edge().await;
                let mut data = [0u8; 13];
                data.copy_from_slice(b"Notification0");
                {
                    let mut counter = counter.borrow_mut();
                    data[data.len() - 1] += *counter;
                    *counter = (*counter + 1) % 10;
                }
                NotificationData::new(TX_handle, &data)
            }
        };

        srv.run(&mut notifier).await.unwrap();
    }
}
