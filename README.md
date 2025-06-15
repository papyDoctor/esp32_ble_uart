# BLUETOOTH BLE UART

This is a clean **no_std** example of using GATT UART bluetooth BLE with the ESP32-H2 SOC using Rust embassy.
Don't change the uuids used, they are the nrf defacto standard for uart capabilities.
Used with ESP32-H2-DevKitM-1-N4 and with espflash (no probe-rs) but can be used with other Espressif SoC with bluetooth features.

Use the beautifull BluefruitConnect https://learn.adafruit.com/bluefruit-le-connect/ios-setup app to test this example

The task task_led(...) is just a task example, not working with the embedded WS2812B LED


## [Changelog](CHANGELOG.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
