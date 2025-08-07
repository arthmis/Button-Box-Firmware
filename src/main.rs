//! USB HID Button Box with 2 buttons
//!
//! This implements a USB HID device that reports button states for a 2-button box.
#![no_std]
#![no_main]

mod hid_descriptor;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::InputPin;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::{FunctionSio, Pin, PullUp, SioInput},
    pac,
    sio::Sio,
    usb::UsbBus,
    watchdog::Watchdog,
};

use usb_device::{class_prelude::*, prelude::*};
use usbd_hid::{descriptor::generator_prelude::*, hid_class::HIDClass};

// HID Report descriptor for a 2-button gamepad
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage_page = BUTTON, usage_min = 0x01, usage_max = 0x02) = {
                #[packed_bits 2] #[item_settings data,variable,absolute] buttons=input;
            };
            // Padding to align to byte boundary
            #[packed_bits 6] #[item_settings constant,variable,absolute] padding=input;
        };
    }
)]
pub struct ButtonBoxReport {
    pub buttons: u8,
    pub padding: u8,
}

// GPIO pin type aliases for button inputs
type Button1Pin = Pin<bsp::hal::gpio::bank0::Gpio23, FunctionSio<SioInput>, PullUp>;
type Button2Pin = Pin<bsp::hal::gpio::bank0::Gpio15, FunctionSio<SioInput>, PullUp>;

struct ButtonBox {
    button1: Button1Pin,
    button2: Button2Pin,
    last_report: ButtonBoxReport,
}

impl ButtonBox {
    fn new(button1: Button1Pin, button2: Button2Pin) -> Self {
        Self {
            button1,
            button2,
            last_report: ButtonBoxReport {
                buttons: 0,
                padding: 0,
            },
        }
    }

    fn read_buttons(&mut self) -> ButtonBoxReport {
        let mut buttons = 0u8;

        // Read button states (buttons are active low with pull-up resistors)
        if self.button1.is_low().unwrap_or(false) {
            buttons |= 0x01; // Button 1
        }
        if self.button2.is_low().unwrap_or(false) {
            buttons |= 0x02; // Button 2
        }

        ButtonBoxReport {
            buttons,
            padding: 0,
        }
    }

    fn has_changed(&mut self) -> bool {
        let current_report = self.read_buttons();
        let changed = current_report.buttons != self.last_report.buttons;
        self.last_report = current_report;
        changed
    }

    fn get_report(&self) -> ButtonBoxReport {
        self.last_report
    }
}

#[entry]
fn main() -> ! {
    info!("Button Box starting...");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure button pins with pull-up resistors
    // Button 1 on GPIO14, Button 2 on GPIO15
    let button1 = pins.gpio14.into_pull_up_input();
    let button2 = pins.gpio15.into_pull_up_input();

    // Create button box instance
    let mut button_box = ButtonBox::new(button1, button2);

    // Set up USB
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Create HID class
    let mut hid = HIDClass::new(&usb_bus, ButtonBoxReport::desc(), 1);

    // Create USB device
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Button Box Co")
            .product("2-Button Box")
            .serial_number("001")])
        .unwrap()
        .device_class(0x00) // Use interface-specific class
        .build();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    info!("Button Box ready!");

    loop {
        // Poll USB device
        if usb_dev.poll(&mut [&mut hid]) {
            // Check if buttons have changed
            if button_box.has_changed() {
                let report = button_box.get_report();
                info!("Button state changed: {}", report.buttons);

                // Send HID report
                match hid.push_input(&report) {
                    Ok(_) => {
                        debug!("HID report sent successfully");
                    }
                    Err(UsbError::WouldBlock) => {
                        // Host not ready, will try again next loop
                    }
                    Err(_e) => {
                        warn!("Failed to send HID report");
                    }
                }
            }
        }

        // Small delay to prevent overwhelming the USB bus
        delay.delay_us(100);
    }
}

// End of file
