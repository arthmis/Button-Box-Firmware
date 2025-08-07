# HID Button Box Documentation

## Overview

This firmware implements a USB HID (Human Interface Device) button box with 2 buttons for the Raspberry Pi Pico (RP2040). The device presents itself as a gamepad to the host computer and sends button press/release events via HID reports.

## Hardware Setup

### Pin Configuration
- **Button 1**: GPIO14 (Pin 19)
- **Button 2**: GPIO15 (Pin 20)
- **Ground**: Any GND pin

### Wiring
Connect each button between the GPIO pin and ground. The firmware uses internal pull-up resistors, so no external resistors are needed.

```
Button 1: GPIO14 ----[Button]---- GND
Button 2: GPIO15 ----[Button]---- GND
```

## HID Report Descriptor

The device uses a custom HID report descriptor that defines it as a gamepad with the following structure:

### Report Format
```
Byte 0: Button states (bits 0-1 used, bits 2-7 padding)
  - Bit 0: Button 1 state (1 = pressed, 0 = released)
  - Bit 1: Button 2 state (1 = pressed, 0 = released)
  - Bits 2-7: Padding (always 0)
```

### HID Descriptor Details
- **Usage Page**: Generic Desktop (0x01)
- **Usage**: Gamepad (0x05)
- **Collection**: Application
- **Button Usage Page**: Button (0x09)
- **Button Range**: 1-2 (0x01-0x02)
- **Report Size**: 1 byte
- **Report ID**: None (single report type)

## USB Device Information

- **Vendor ID (VID)**: 0x16C0 (Van Ooijen Technische Informatica)
- **Product ID (PID)**: 0x27DD (Generic HID device)
- **Manufacturer**: "Button Box Co"
- **Product**: "2-Button Box"
- **Serial Number**: "001"
- **Device Class**: Interface-specific (0x00)

## Firmware Behavior

### Initialization
1. Configure GPIO pins as inputs with pull-up resistors
2. Initialize USB HID device
3. Enter main loop

### Main Loop
1. Poll USB device for host communication
2. Read current button states
3. Compare with previous state to detect changes
4. Send HID report only when button states change
5. Small delay to prevent USB bus overflow

### Button State Detection
- Buttons are active-low (pressed = logic 0, released = logic 1)
- Internal pull-up resistors ensure clean logic levels
- Debouncing is not implemented in firmware (relies on hardware or OS-level handling)

## Usage in Applications

### Windows
The device will appear as a generic gamepad in Device Manager under "Human Interface Devices". Applications that support generic HID gamepads can read the button states.

### Linux
The device will create an input device (typically `/dev/input/eventX`) that can be read using standard input APIs or tools like `evtest`.

### macOS
The device will be accessible through the IOHIDManager framework and appear in system profiler under USB devices.

## Testing

### Using `evtest` on Linux
```bash
sudo evtest /dev/input/eventX
```
Replace X with the appropriate event device number.

### Using Windows Game Controller Test
1. Open "Set up USB game controllers" from Control Panel
2. Select the "2-Button Box" device
3. Click "Properties" to test button functionality

## Building and Flashing

### Prerequisites
- Rust toolchain with thumbv6m-none-eabi target
- probe-rs or other RP2040 flashing tool

### Build Commands
```bash
# Check for compilation errors
cargo check

# Build the firmware
cargo build --release

# Flash to device (requires probe-rs)
cargo run --release
```

### Alternative Flashing Methods
1. **UF2 Bootloader**: Hold BOOTSEL while connecting USB, copy generated UF2 file
2. **OpenOCD**: Use with SWD debugger probe
3. **picotool**: Raspberry Pi's official tool

## Customization

### Adding More Buttons
1. Modify the HID descriptor to include more buttons:
   ```rust
   (usage_min = 0x01, usage_max = 0x04) // For 4 buttons
   #[packed_bits 4] buttons=input; // 4 bits for 4 buttons
   #[packed_bits 4] padding=input; // Adjust padding
   ```

2. Add GPIO pin configurations for additional buttons

3. Update the button reading logic in `read_buttons()` method

### Changing Button Pins
Modify the GPIO pin numbers in the main function:
```rust
let button1 = pins.gpio12.into_pull_up_input(); // Change to GPIO12
let button2 = pins.gpio13.into_pull_up_input(); // Change to GPIO13
```

### Adding Debouncing
Implement software debouncing by adding a delay or state tracking in the button reading logic.

## Troubleshooting

### Device Not Recognized
- Check USB cable and connection
- Verify the device appears in system device manager
- Try different USB ports

### Buttons Not Responding
- Check wiring connections
- Verify GPIO pin assignments match hardware
- Test with multimeter for proper voltage levels

### Compilation Errors
- Ensure all dependencies are correctly specified in Cargo.toml
- Check Rust toolchain includes thumbv6m-none-eabi target
- Verify usbd-hid and usb-device crate versions are compatible

## References

- [USB HID Usage Tables](https://www.usb.org/sites/default/files/documents/hut1_12v2.pdf)
- [RP2040 Datasheet](https://datasheets.raspberrypi.org/rp2040/rp2040-datasheet.pdf)
- [usbd-hid Documentation](https://docs.rs/usbd-hid/)
- [Raspberry Pi Pico Getting Started Guide](https://datasheets.raspberrypi.org/pico/getting-started-with-pico.pdf)