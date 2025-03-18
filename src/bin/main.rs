#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;

// macro_rules! mk_static {
//     ($t:ty,$val:expr) => {{
//         static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
//         #[deny(unused_attributes)]
//         let x = STATIC_CELL.uninit().write(($val));
//         x
//     }};
// }

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // https://www.electronicshub.org/esp32-pinout/
    // ADC input on pin 2 (GPIO 36)
    //let analog_pin = peripherals.GPIO36;
    let analog_pin = peripherals.GPIO13;
    let mut adc2_config = AdcConfig::new();
    let mut adc_pin = adc2_config.enable_pin(analog_pin, Attenuation::_6dB);
    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);

    // This is the blue LED
    let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        // Read 100 samples and average them
        // esp_println::println!("Starting sample read");
        let mut input: u32 = 0;
        for _ in 0..1000 {
            input += nb::block!(adc2.read_oneshot(&mut adc_pin)).unwrap() as u32;
        }
        input /= 1000;
        // esp_println::println!("Completed sample read");

        if input > 1000 {
            led.set_high();
            esp_println::println!("Input = {}", input);
        } else {
            led.set_low();
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
