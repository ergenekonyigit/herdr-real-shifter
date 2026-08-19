# DIY Hardware Guide: Logitech Shifter USB Adapter

The **Logitech Driving Force Shifter** (compatible with G29, G920, and G923) terminates in a female **DB9 (DE-9)** connector designed to plug into a Logitech steering wheel base.

With an **Arduino Leonardo** (or **SparkFun Pro Micro**) and a solderless **DB9 male screw-terminal breakout board**, you can build a standalone plug-and-play USB HID adapter in **under 10 minutes for ~$10**.

```
┌──────────────────────────────┐          ┌───────────────────────────┐
│ Logitech Driving Force       │          │ Arduino Leonardo          │
│ Shifter (DB9 Male Breakout)  │          │ (ATmega32U4 Native USB)   │
│                              │          │                           │
│  [Pin 2] Reverse Switch ─────┼──────────┼──> Digital Pin 2 (D2)     │
│  [Pin 4] X-Axis Potentiometer┼──────────┼──> Analog Pin A0 (A0)     │
│  [Pin 6] Ground (GND) ───────┼──────────┼──> Ground (GND)           │
│  [Pin 8] Y-Axis Potentiometer┼──────────┼──> Analog Pin A2 (A2)     │
│  [Pin 9] Power (+5V VCC) ────┼──────────┼──> Power (5V)             │
└──────────────────────────────┘          └───────────────────────────┘
                                                       │
                                                 [Micro USB Cable]
                                                       │
                                                       ▼
                                            🖥️ PC / Mac / RealShifter
```

---

## 1. Bill of Materials (BOM)

| Item | Specification / Recommendation | Purpose |
| :--- | :--- | :--- |
| **Logitech Shifter** | Logitech Driving Force Shifter (G29 / G920 / G923) | Physical H-pattern gear stick (6 gears + reverse) |
| **Microcontroller** | **Arduino Leonardo** or **SparkFun Pro Micro 5V (ATmega32U4)** | Translates analog & digital signals into native USB HID Gamepad |
| **DB9 Breakout Board** | **DB9 (DE-9) Male to 9-Pin Screw Terminal Adapter** | Solderless connection to the Logitech shifter cable |
| **Jumper Wires** | 5x Dupont Wires (Male-to-Male or Male-to-Female) | Connects screw terminals to Arduino pin headers |
| **USB Cable** | Micro-USB to USB-A (or USB-C) | Connects Arduino to PC/Mac |
| **Tool** | Small Flathead Screwdriver | For tightening screw terminals |

> [!IMPORTANT]
> **Microcontroller Requirement:** You **MUST** use an Arduino board with native USB capability (such as **ATmega32U4** on Arduino Leonardo or Pro Micro). Standard Arduino boards (like Uno R3 or Nano with CH340 / FTDI USB-to-UART chips) **CANNOT** enumerate as a native USB HID Joystick without low-level DFU firmware modification.

> [!NOTE]
> **G25 / G27 vs Driving Force:** The Logitech G25 and G27 shifters have internal SPI shift registers and buttons, requiring different wiring. The pinout below is specifically tailored for the modern **Driving Force Shifter (G29 / G920 / G923)**.

---

## 2. Wiring & Pinout Specification

Strip or insert 5 Dupont jumper wires into the DB9 screw terminal board and tighten the screws firmly. Connect the other ends to the Arduino Leonardo headers:

| DB9 Pin | Internal Wire Color | Function | Arduino Leonardo Pin | Description |
| :---: | :---: | :---: | :---: | :--- |
| **Pin 1** | — | NC (Not Connected) | — | Unused |
| **Pin 2** | 🟢 Green | **Reverse Gear Switch** | **Digital Pin 2** (`D2`) | Active LOW microswitch (connects to GND when stick is pushed down) |
| **Pin 3** | — | NC (Not Connected) | — | Unused |
| **Pin 4** | 🟡 Yellow | **X-Axis Potentiometer** | **Analog Pin A0** (`A0`) | Left/Right position (0–5V analog voltage divider) |
| **Pin 5** | — | NC (Optional LED) | — | Unused |
| **Pin 6** | ⚫ Black | **Ground (GND)** | **GND** | Common reference ground |
| **Pin 7** | — | NC (Not Connected) | — | Unused |
| **Pin 8** | ⚪ White | **Y-Axis Potentiometer** | **Analog Pin A2** (`A2`) | Forward/Backward position (0–5V analog voltage divider) |
| **Pin 9** | 🔴 Red | **Power (+5V VCC)** | **5V** | 5V DC supply for potentiometers |

#### DB9 Connector Pin Layout (Front Face View - Male):
```
      1       2       3       4       5
    ┌───┬───┬───┬───┬───┐
    │ o   o   o   o   o │
     \  o   o   o   o  /
      └───┴───┴───┴───┘
        6   7   8   9
```

---

## 3. How It Works (Mechanical & Electrical Logic)

The Logitech Driving Force Shifter contains:
1. **X-Axis Potentiometer (DB9 Pin 4 -> A0)**: Reads horizontal stick movement (`Left <-> Center <-> Right`).
2. **Y-Axis Potentiometer (DB9 Pin 8 -> A2)**: Reads vertical stick movement (`Top <-> Neutral <-> Bottom`).
3. **Reverse Microswitch (DB9 Pin 2 -> D2)**: Activated only when you **push down** the shift knob into the housing and engage the bottom-right gate.

The Arduino reads analog values (0 to 1023 ADC) and maps the (X, Y) coordinate plane into the 6 physical gates:

```
              Left Column                 Center Column                 Right Column
             (X < 400 ADC)             (400 <= X <= 650 ADC)            (X > 650 ADC)
        ┌───────────────────────────┬───────────────────────────┬───────────────────────────┐
  Top   │        1️⃣ GEAR 1          │        3️⃣ GEAR 3          │        5️⃣ GEAR 5          │
(Y > 650)│  (HID Button 1 / Idx 0)   │  (HID Button 3 / Idx 2)   │  (HID Button 5 / Idx 4)   │
        ├───────────────────────────┼───────────────────────────┼───────────────────────────┤
 Neutral│             ·             │        ⚪ NEUTRAL         │             ·             │
(400-650)│                           │    (All Buttons OFF)      │                           │
        ├───────────────────────────┼───────────────────────────┼───────────────────────────┤
 Bottom │        2️⃣ GEAR 2          │        4️⃣ GEAR 4          │        6️⃣ GEAR 6          │
(Y < 400)│  (HID Button 2 / Idx 1)   │  (HID Button 4 / Idx 3)   │  (HID Button 6 / Idx 5)   │
        │                           │                           │  ─────────────────────────│
        │                           │                           │    🔴 REVERSE (Button 7)  │
        │                           │                           │   (If Stick Pressed Down) │
        └───────────────────────────┴───────────────────────────┴───────────────────────────┘
```

---

## 4. Software Setup & Uploading Firmware

#### Step 1: Install Arduino IDE
Download and install the official Arduino IDE (v2.x or v1.8.x) from [arduino.cc/en/software](https://www.arduino.cc/en/software).

#### Step 2: Install the Arduino Joystick Library
1. Open Arduino IDE.
2. Go to **Tools** ➔ **Manage Libraries...** (or press `Ctrl+Shift+I` / `Cmd+Shift+I`).
3. In the search box, type: `ArduinoJoystickLibrary` or `Joystick by Matthew Heironimus`.
4. Click **Install** (or install from [GitHub Releases](https://github.com/MHeironimus/ArduinoJoystickLibrary/releases)).

#### Step 3: Open and Flash the Firmware
Open the project sketch located at [`firmware/logitech_shifter_usb/logitech_shifter_usb.ino`](../firmware/logitech_shifter_usb/logitech_shifter_usb.ino) in Arduino IDE.

#### Step 4: Configure Board & Port in Arduino IDE
- **Tools** ➔ **Board** ➔ **Arduino AVR Boards** ➔ **Arduino Leonardo** (or SparkFun Pro Micro 5V / 16MHz).
- **Tools** ➔ **Port** ➔ Select your connected device port (e.g. `/dev/cu.usbmodem...` on macOS or `COMx` on Windows).
- Click **Upload** (`➔`).

---

## 5. Diagnostic & Calibration Tool

If your potentiometer center values differ slightly from manufacturing tolerances, upload the diagnostic tool from [`firmware/diagnostic_serial_calibrator/diagnostic_serial_calibrator.ino`](../firmware/diagnostic_serial_calibrator/diagnostic_serial_calibrator.ino):

1. Upload `diagnostic_serial_calibrator.ino` to your board.
2. Open **Tools** ➔ **Serial Monitor** (Set baud rate to **115200**).
3. Move the shifter stick into all gear slots and observe live output:
   ```
   X (A0): 215  | Y (A2): 890  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 1️⃣ GEAR 1
   X (A0): 220  | Y (A2): 180  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 2️⃣ GEAR 2
   X (A0): 510  | Y (A2): 885  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 3️⃣ GEAR 3
   X (A0): 512  | Y (A2): 185  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 4️⃣ GEAR 4
   X (A0): 840  | Y (A2): 890  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 5️⃣ GEAR 5
   X (A0): 845  | Y (A2): 190  | Rev Switch (D2): [RELEASED (HIGH)] | Detected: 6️⃣ GEAR 6
   X (A0): 850  | Y (A2): 185  | Rev Switch (D2): [PRESSED (LOW)]  | Detected: 🔴 REVERSE (R)
   ```
4. If a gear fails to detect, note the observed X and Y values and adjust the threshold constants (`X_THRESHOLD_LOW`, `X_THRESHOLD_HIGH`, etc.) in `logitech_shifter_usb.ino`.

---

## 6. Testing & Verification

| OS | Test Tool / Method | What to Look For |
| :--- | :--- | :--- |
| **macOS** | Run `cargo run -p realshifter-tui` or `hidutil list` | Device appears under `VendorID: 0x2341` (Arduino) and shifts show in live TUI |
| **Windows** | Press `Win + R` ➔ type `joy.cpl` ➔ Properties | Standard 7-button gamepad; Buttons 1–7 light up red as you shift |
| **Linux** | Run `jstest /dev/input/js0` or `evtest` | Button indices 0–6 toggle `on`/`off` with zero jitter |

---

## 7. Troubleshooting & FAQ

- **Q: Can I use an Arduino Uno or Arduino Nano?**
  - **A:** No. Standard Uno and Nano boards use FTDI or CH340 USB-serial chips which cannot speak native USB HID. Use an **ATmega32U4** board (Arduino Leonardo or SparkFun Pro Micro 5V).
- **Q: Reverse gear triggers Gear 6 instead of Reverse.**
  - **A:** Ensure you push the shifter knob **straight down** before moving into the 6th gear slot. Check that DB9 Pin 2 is connected to Arduino Pin 2 and configured as `INPUT_PULLUP`.
- **Q: Shifting feels jittery or random buttons flicker in Neutral.**
  - **A:** Ensure the Ground (GND) wire is tightly screwed into DB9 Pin 6 and connected to the Arduino GND pin. A floating ground creates ADC noise.
- **Q: 1st gear is recognized as 3rd gear.**
  - **A:** The X-axis potentiometer reading is higher than `X_THRESHOLD_LOW`. Use the Diagnostic sketch to read your exact X value in 1st gear and increase `X_THRESHOLD_LOW` (e.g. from 400 to 450).
