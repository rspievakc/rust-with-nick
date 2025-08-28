#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_net::{Runner, StackResources};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, OutputConfig, Pull};
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::{
    gpio::{Io, Level, Output},
    timer::timg::TimerGroup,
};

use defmt::{error, info};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_wifi::EspWifiController;
use panic_rtt_target as _;

use esp_wifi::wifi::{
    ClientConfiguration, Configuration, ScanConfig, WifiController, WifiDevice, WifiEvent,
};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

// WIFI Credentials
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    // Need to make it static since the wifi_controller is going to live forever
    let wifi_init = mk_static!(
        EspWifiController<'_>,
        esp_wifi::init(timer1.timer0, rng).expect("Failed to initialize WIFI/BLE controller")
    );
    let (wifi_controller, interfaces) = esp_wifi::wifi::new(wifi_init, peripherals.WIFI)
        .expect("Failed to initialize WIFI controller");
    let wifi_interface = interfaces.sta;
    let wifi_config = embassy_net::Config::dhcpv4(Default::default());
    let wifi_seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        wifi_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        wifi_seed,
    );

    // Initializes GPIO for the LED Blink
    let io = Io::new(peripherals.IO_MUX);
    let mut LED_Pin_11 = Output::new(
        peripherals.GPIO11,
        Level::High,
        OutputConfig::default()
            .with_drive_mode(DriveMode::PushPull)
            .with_pull(Pull::Up),
    )
    .into_flex();

    let mut LED_Pin_4 = Output::new(
        peripherals.GPIO4,
        Level::High,
        OutputConfig::default()
            .with_drive_mode(DriveMode::PushPull)
            .with_pull(Pull::Up),
    )
    .into_flex();

    spawner.spawn(connection(wifi_controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
        LED_Pin_11.toggle();
        LED_Pin_4.toggle();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    info!("Starting WIFI connection task");
    info!("Device capabilities: {:?}", controller.capabilities());

    match controller.set_mode(esp_wifi::wifi::WifiMode::Sta) {
        Ok(_result) => {}
        Err(error) => {
            error!("Problem found setting the wifi mode to Station. {}", error);
        }
    };
    match controller.start_async().await {
        Ok(_result) => {}
        Err(error) => {
            error!("Wifi controller start failure. {}", error);
        }
    }

    info!("Wifi started = {}", controller.is_started());

    loop {
        // Wait for the controller to be disconnected

        // match controller.is_connected() {
        //     Ok(value) => {
        //         if value {
        //             controller.wait_for_event(WifiEvent::StaDisconnected).await;
        //         }
        //     }
        //     _ => {}
        // }

        info!("Scanning networks...");
        // Only make operations when the controller is started.
        if controller.is_started().is_ok_and(|b| b) {
            // let client_config = Configuration::Client(ClientConfiguration {
            //     ssid: SSID.into(),
            //     password: PASSWORD.into(),
            //     ..Default::default()
            // });
            let scan_config = ScanConfig::default();
            let result = controller.scan_with_config_async(scan_config).await;
            match result {
                Ok(scan_result) => {
                    info!("Scan finished with {} STAs found.", scan_result.len());
                    for network in scan_result {
                        info!("SSID: {}", network.ssid.as_str())
                    }
                }
                Err(err) => {
                    info!("Problem found scanning for WIFI networks. {}", err);
                }
            }
        }
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
