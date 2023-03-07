use anyhow::{anyhow, Result};
use embedded_hal::can::nb;
use embedded_hal::serial::Read;
use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin};
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_hal::uart::{SerialError, UartDriver};
use esp_idf_hal::units::*;
use esp_idf_sys as _;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    let p = Peripherals::take().unwrap();

    // TX: 14
    // RX: 13

    // Init UART1
    let mut uart = uart::UartDriver::new(
        p.uart1,
        p.pins.gpio14,
        p.pins.gpio13,
        Option::<AnyInputPin>::None,
        Option::<AnyOutputPin>::None,
        &uart::UartConfig::default().baudrate(3_000_000.into()),
    )
    .expect("Failed to init UART1");

    // Write some text to the UART periodically
    loop {
        // uart.write(b"Hello, world!\n")
        //     .expect("Failed to write to UART1");

        let mut line = String::new();
        'line: loop {
            let mut buf = [0; 1];
            let amount_read = uart.read(&mut buf, 100).expect("Failed to read from UART1");
            if amount_read > 0 {
                let c = buf[0] as char;
                match c {
                    '\r' => continue,
                    '\n' => break 'line,
                    _ => line.push(c),
                }
            }
        }

        println!("Received: {}", line);
    }
}

// fn read_line<T: Read<u8>>(serial: &mut T) -> Result<String> {
//     let mut line = String::new();
//
//     loop {
//         // Read one byte from the serial port
//         let byte = serial.read().expect("bla");
//         let c = byte as char;
//
//         match c {
//             '\r' => continue,
//             '\n' => return Ok(line),
//             _ => line.push(c),
//         }
//     }
// }
