# Button Box Firmware

A USB HID button box implementation for the Raspberry Pi Pico (RP2040) that provides 2-button input functionality.

## Overview

This firmware transforms a Raspberry Pi Pico into a USB HID gamepad device with 2 buttons. The device sends button press/release events to the host computer and is compatible with Windows, Linux, and macOS.

## Features

- **USB HID Gamepad Interface**: Presents as a standard gamepad device
- **2 Button Support**: GPIO14 and GPIO15 configured as button inputs
- **Hardware Pull-ups**: No external resistors required
- **Real-time Updates**: Only sends HID reports when button states change
- **Cross-platform Compatible**: Works with Windows, Linux, and macOS

## Hardware Setup

### Required Components
- Raspberry Pi Pico
- 2 momentary push buttons
- Jumper wires

### Wiring
Connect buttons between GPIO pins and ground:

```
Button 1: GPIO14 (Pin 19) ──[Button]── GND
Button 2: GPIO15 (Pin 20) ──[Button]── GND
```

No pull-up resistors needed - the firmware uses internal pull-ups.

## HID Report Format

The device sends 1-byte HID reports with the following structure:

| Bit | Function |
|-----|----------|
| 0   | Button 1 state (1 = pressed, 0 = released) |
| 1   | Button 2 state (1 = pressed, 0 = released) |
| 2-7 | Padding (always 0) |

### Example Reports
- No buttons: `0x00`
- Button 1 only: `0x01`
- Button 2 only: `0x02`
- Both buttons: `0x03`

## USB Device Information

- **Vendor ID**: 0x16C0 (Van Ooijen Technische Informatica)
- **Product ID**: 0x27DD (Generic HID device)
- **Product Name**: "2-Button Box"
- **Manufacturer**: "Button Box Co"
- **Serial Number**: "001"

## Building and Flashing

### Prerequisites

1. Install Rust and the embedded toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv6m-none-eabi
```

2. Install probe-rs for flashing:
```bash
cargo install probe-rs --features cli
```

### Build Commands

```bash
# Check for compilation errors
cargo check

# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Flash to Pico (requires probe-rs and debug probe)
cargo run --release
```

### Alternative Flashing (UF2 Bootloader)

1. Hold the BOOTSEL button while connecting USB
2. Convert ELF to UF2:
```bash
elf2uf2-rs target/thumbv6m-none-eabi/release/button-box-firmware button-box.uf2
```
3. Copy `button-box.uf2` to the RPI-RP2 drive

## Testing

### Linux
```bash
# List input devices
ls /dev/input/event*

# Test with evtest
sudo evtest /dev/input/eventX
```

### Windows
1. Open "Set up USB game controllers" from Control Panel
2. Select "2-Button Box" device
3. Click "Properties" to test buttons

### macOS
Use system profiler or third-party HID testing tools to verify functionality.

## Customization

### Adding More Buttons

1. Update the HID descriptor in `main.rs`:
```rust
(usage_min = 0x01, usage_max = 0x04) // For 4 buttons
#[packed_bits 4] buttons=input; // 4 bits for buttons
#[packed_bits 4] padding=input; // Adjust padding
```

2. Configure additional GPIO pins
3. Update button reading logic

### Changing Button Pins

Modify the GPIO assignments in `main()`:
```rust
let button1 = pins.gpio12.into_pull_up_input(); // Use GPIO12
let button2 = pins.gpio13.into_pull_up_input(); // Use GPIO13
```

### Adding Debouncing

Implement software debouncing by adding delays or state tracking in the button reading logic.

## File Structure

- `src/main.rs` - Main firmware implementation
- `src/hid_descriptor.rs` - HID descriptor analysis and helper functions
- `HID_BUTTON_BOX.md` - Detailed technical documentation
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- `cortex-m` - ARM Cortex-M runtime
- `cortex-m-rt` - Cortex-M runtime startup code
- `embedded-hal` - Hardware abstraction layer
- `rp-pico` - Raspberry Pi Pico board support package
- `usb-device` - USB device framework
- `usbd-hid` - USB HID class implementation
- `defmt` - Efficient logging framework
- `panic-probe` - Panic handler for debugging

## Troubleshooting

### Device Not Detected
- Verify USB cable and connection
- Check if device appears in system device manager
- Try different USB ports

### Buttons Not Working
- Check wiring connections to GPIO14/GPIO15
- Verify buttons are connected to ground
- Test button continuity with multimeter

### Build Errors
- Ensure thumbv6m-none-eabi target is installed
- Check that all dependencies are compatible
- Verify Rust toolchain is up to date

## License

This project is dual-licensed under MIT OR Apache-2.0.

## References

- [USB HID Usage Tables](https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf)
- [RP2040 Datasheet](https://datasheets.raspberrypi.org/rp2040/rp2040-datasheet.pdf)
- [Raspberry Pi Pico Getting Started](https://datasheets.raspberrypi.org/pico/getting-started-with-pico.pdf)
- [usbd-hid Documentation](https://docs.rs/usbd-hid/)