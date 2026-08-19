# 🏎️ RealShifter

**RealShifter** is a hardware-driven context & model switching plugin designed for [Herdr](https://github.com/ergenekonyigit/herdr) and AI CLI power-users. It maps **USB H-pattern manual gear shifters** (such as Logitech Driving Force Shifter) and configurable hotkeys directly to model switches, effort tiers, and prompt actions across modern AI coding CLIs (AGY / Antigravity, Claude Code, Codex CLI, OpenCode CLI, Pi Coding Agent).

---

## 🌟 Key Features

- **🏎️ Hardware H-Pattern Shifting**: Map physical gears (Gears 1–6 & Reverse) on Logitech USB shifters directly to LLM model profiles and reasoning effort tiers.
- **📑 Reverse Gear Session Spawning**: Engaging Reverse (R) automatically opens a new focused tab in Herdr and launches a fresh coding session for your active CLI profile (`agy`, `claude`, `codex`, `opencode`, `pi`).
- **🔌 Solder-Free DIY USB Adapter**: Complete hardware guide & Arduino firmware included to convert a standalone Logitech Shifter to USB HID in 10 minutes.
- **⚡ Hotkey & Action Fallbacks**: Full keyboard shortcut support (`Ctrl+Shift+1..6`, `Ctrl+Shift+R`, `Ctrl+Shift+P`) via `realshifter-action`.
- **🖥️ Terminal UI (TUI) Dashboard**: Interactive Ratatui-based overlay dashboard for live gear visualization, device status monitoring, and profile mapping edits.
- **🔄 Multi-CLI Profile Management**: Seamlessly sync and shift profiles across:
  - **Antigravity (AGY)**
  - **Claude Code**
  - **Codex CLI**
  - **OpenCode CLI**
  - **Pi Coding Agent**
  - **Custom Shell Actions**
- **🔌 Native Herdr Plugin Architecture**: Deeply integrated into Herdr with lifecycle management, custom pane placement, keybindings, and automated background daemon supervision.
- **🤖 AGY & Agent Skill Integration**: Built-in skill (`realshifter`) for automated model discovery and profile synchronization.

---

## 📊 CLI Feature Compatibility Matrix

RealShifter provides deep multi-CLI integration across supported AI coding assistants. Below is the current feature matrix status:

| Feature / Capability | Description | AGY (Antigravity) | Codex CLI | Claude Code | OpenCode CLI | Pi Agent |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **USB HID Hardware Shifting** | Hardware H-pattern gear shift detection (Logitech DFS, Arduino Leonardo) | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Active Pane Auto-Detection** | Auto-detect active CLI from terminal title / agent process | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Pinned Pane Targeting** | Target pinned pane (`realshifter-action profile pin-pane <ID>`) | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Profile JSON Persistence** | Config storage in `~/.config/realshifter/profiles/*.json` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Dynamic Gear Mapping (1–6, R)** | Mapping physical gears 1–6 and Reverse to custom actions | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Interactive Model Picker (`/model` / `/models`)** | Automated menu navigation & search filtering | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Reasoning Effort Tiers** | Granular effort level / variant selection | ✅ | ✅ | ✅ | ✅ | ✅ |
| **New Session / Tab Spawning** | Reverse gear spawns a fresh Herdr tab session for active profile | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Agent / CLI Skill Sync** | Automated model discovery skill & profile synchronization | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Custom Shell Command Execution** | Direct fallback execution of custom CLI flags & bash scripts | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Direct Model Switching** | Immediate single-turn model switching | ✅ | ✅ | ✅ | ✅ | ✅ |

*Legend: ✅ Supported & Verified*

---

## 🏎️ Profile Configurations & Supported CLI Models

### 🚀 Antigravity (AGY) Profile

| Gear | Model / Action | Effort Level | Role / Description |
| :---: | :--- | :---: | :--- |
| **1️⃣ Gear 1** | `gemini-3.7-flash-low` | Low | Ultra-fast lightweight assistance & quick edits |
| **2️⃣ Gear 2** | `gemini-3.7-flash-medium` | Medium | Balanced everyday coding & refactoring |
| **3️⃣ Gear 3** | `gemini-3.7-flash-high` | High | Deep reasoning & multi-step debugging |
| **4️⃣ Gear 4** | `gemini-3.1-pro-high` | High | Complex architecture & large codebase reasoning |
| **5️⃣ Gear 5** | `claude-sonnet-4-6` | Thinking | Claude Sonnet reasoning |
| **6️⃣ Gear 6** | `claude-opus-4-6-thinking` | Thinking | Claude Opus flagship reasoning |
| **🔴 Reverse (R)** | **New Tab Session** | `agy` | Spawns a new focused AGY tab session in Herdr |

### ⚡ OpenAI Codex CLI Profile

| Gear | Model / Action | Effort Level | Role / Description |
| :---: | :--- | :---: | :--- |
| **1️⃣ Gear 1** | `gpt-5.4-mini` | Medium | Small, fast, and cost-efficient model |
| **2️⃣ Gear 2** | `gpt-5.4` | Medium | Strong model for everyday coding |
| **3️⃣ Gear 3** | `gpt-5.6-luna` | Medium | Fast & affordable agentic coding model |
| **4️⃣ Gear 4** | `gpt-5.6-terra` | Medium | Balanced agentic coding model for everyday work |
| **5️⃣ Gear 5** | `gpt-5.5` | Medium | **Frontier:** Complex coding, research & deep reasoning |
| **6️⃣ Gear 6** | `gpt-5.5-high` | High | Complex high-effort reasoning |
| **🔴 Reverse (R)** | **New Tab Session** | `codex` | Spawns a new focused Codex tab session in Herdr |

### 🧠 Claude Code Profile

| Gear | Model / Action | Command | Role / Description |
| :---: | :--- | :--- | :--- |
| **1️⃣ Gear 1** | **Haiku 4.5** | `/model haiku` | Fastest model for quick answers ($1/$5 per Mtok) |
| **2️⃣ Gear 2** | **Sonnet 5** | `/model sonnet` | Efficient daily coding & routine tasks ($2/$10 promo) |
| **3️⃣ Gear 3** | **Opus 5 (1M)** | `/model opus` | 1M context flagship for complex tasks ($5/$25 per Mtok) |
| **4️⃣ Gear 4** | **Sonnet 5 (Thinking)** | `/model sonnet --thinking` | Sonnet with extended reasoning budget |
| **5️⃣ Gear 5** | **Opus 5 (Thinking)** | `/model opus --thinking` | Opus 1M flagship with extended reasoning |
| **6️⃣ Gear 6** | **Fable 5** | `/model fable` | Most capable for hardest & longest tasks ($10/$50 per Mtok) |
| **🔴 Reverse (R)** | **New Tab Session** | `claude` | Spawns a new focused Claude Code tab session in Herdr |

### ⚡ OpenCode CLI Profile (Free Models)

| Gear | Model / Action | Variant / Tier | Role / Description |
| :---: | :--- | :---: | :--- |
| **1️⃣ Gear 1** | `nemotron-3.5-lightning-free` | Default | Ultra-fast lightweight free assistance |
| **2️⃣ Gear 2** | `deepseek-v4-flash-free` | Default / Varied | Fast & intelligent free coding model |
| **3️⃣ Gear 3** | `laguna-s-2.1-free` | Default | Balanced everyday free coding & refactoring |
| **4️⃣ Gear 4** | `hy3-free` | Default | Alternative general-purpose free coding model |
| **5️⃣ Gear 5** | `nemotron-3-ultra-free` | Default | High-capacity reasoning & complex problem solving |
| **6️⃣ Gear 6** | `mimo-v2.5-free` | Default | Extended free reasoning & coding model |
| **🔴 Reverse (R)** | **New Tab Session** | `opencode` | Spawns a new focused OpenCode tab session in Herdr |

### 🥧 Pi Coding Agent Profile

| Gear | Model / Action | Effort / Variant | Role / Description |
| :---: | :--- | :---: | :--- |
| **1️⃣ Gear 1** | `gpt-5.4-mini` | Fast | Lightweight, fast & cost-efficient model |
| **2️⃣ Gear 2** | `gpt-5.4` | Medium | Strong everyday coding assistant |
| **3️⃣ Gear 3** | `gpt-5.6-luna` | Fast | Fast & capable agentic coding model |
| **4️⃣ Gear 4** | `gpt-5.6-terra` | Medium | Balanced multi-turn coding model |
| **5️⃣ Gear 5** | `gpt-5.5` | Frontier | Complex coding, research & deep reasoning |
| **6️⃣ Gear 6** | `claude-sonnet-4-6` | Thinking | Claude Sonnet reasoning & synthesis |
| **🔴 Reverse (R)** | **New Tab Session** | `pi` | Spawns a new focused Pi tab session in Herdr |

---

## 🔌 DIY Hardware Guide: Logitech Shifter USB Adapter (No Soldering!)

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

### 1. 🧰 Bill of Materials (BOM)

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

### 2. ⚡ Wiring & Pinout Specification (No Soldering)

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

### 3. 🧠 How It Works (Mechanical & Electrical Logic)

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

### 4. 💻 Software Setup & Uploading Firmware

#### Step 1: Install Arduino IDE
Download and install the official Arduino IDE (v2.x or v1.8.x) from [arduino.cc/en/software](https://www.arduino.cc/en/software).

#### Step 2: Install the Arduino Joystick Library
1. Open Arduino IDE.
2. Go to **Tools** ➔ **Manage Libraries...** (or press `Ctrl+Shift+I` / `Cmd+Shift+I`).
3. In the search box, type: `ArduinoJoystickLibrary` or `Joystick by Matthew Heironimus`.
4. Click **Install** (or install from [GitHub Releases](https://github.com/MHeironimus/ArduinoJoystickLibrary/releases)).

#### Step 3: Open and Flash the Firmware
Open the project sketch located in this repository at [`firmware/logitech_shifter_usb/logitech_shifter_usb.ino`](firmware/logitech_shifter_usb/logitech_shifter_usb.ino).

```cpp
#include <Joystick.h>

const int PIN_X_AXIS = A0;     // DB9 Pin 4
const int PIN_Y_AXIS = A2;     // DB9 Pin 8
const int PIN_REVERSE = 2;     // DB9 Pin 2 (Active LOW)

const int X_THRESHOLD_LOW  = 400; // Left column (Gears 1 & 2)
const int X_THRESHOLD_HIGH = 650; // Right column (Gears 5, 6 & Reverse)
const int Y_THRESHOLD_LOW  = 400; // Bottom row (Gears 2, 4, 6, Reverse)
const int Y_THRESHOLD_HIGH = 650; // Top row (Gears 1, 3, 5)

Joystick_ Joystick(
  JOYSTICK_DEFAULT_REPORT_ID,
  JOYSTICK_TYPE_GAMEPAD,
  7, 0,                  // 7 Buttons, 0 Hat Switches
  false, false, false,   // X, Y, Z Axis
  false, false, false,   // Rx, Ry, Rz Axis
  false, false,          // Rudder, Throttle
  false, false, false    // Accelerator, Brake, Steering
);

int lastGear = 0; // 0 = Neutral, 1-6 = Gears, 7 = Reverse

void setup() {
  pinMode(PIN_REVERSE, INPUT_PULLUP);
  pinMode(PIN_X_AXIS, INPUT);
  pinMode(PIN_Y_AXIS, INPUT);

  Joystick.begin(false); // Manual sendState updates for zero-lag latency
}

void loop() {
  int xVal = analogRead(PIN_X_AXIS);
  int yVal = analogRead(PIN_Y_AXIS);
  bool isReversePressed = (digitalRead(PIN_REVERSE) == LOW);

  int currentGear = 0; // Default: Neutral

  // Reverse: Stick pressed down AND placed into bottom-right gate
  if (isReversePressed && xVal > X_THRESHOLD_HIGH && yVal < Y_THRESHOLD_LOW) {
    currentGear = 7; // Reverse (Button 7)
  } else if (xVal < X_THRESHOLD_LOW) {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 1; // Gear 1
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 2; // Gear 2
  } else if (xVal <= X_THRESHOLD_HIGH) {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 3; // Gear 3
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 4; // Gear 4
  } else {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 5; // Gear 5
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 6; // Gear 6
  }

  // Broadcast button press when gear changes
  if (currentGear != lastGear) {
    for (int i = 0; i < 7; i++) {
      Joystick.setButton(i, 0);
    }
    if (currentGear >= 1 && currentGear <= 7) {
      Joystick.setButton(currentGear - 1, 1);
    }
    Joystick.sendState();
    lastGear = currentGear;
  }

  delay(10); // 100Hz scan loop
}
```

#### Step 4: Configure Board & Port in Arduino IDE
- **Tools** ➔ **Board** ➔ **Arduino AVR Boards** ➔ **Arduino Leonardo** (or SparkFun Pro Micro 5V / 16MHz).
- **Tools** ➔ **Port** ➔ Select your connected device port (e.g. `/dev/cu.usbmodem...` on macOS or `COMx` on Windows).
- Click **Upload** (`➔`).

---

### 5. 🔍 Diagnostic & Calibration Tool

If your potentiometer center values differ slightly from manufacturing tolerances, upload the diagnostic tool from [`firmware/diagnostic_serial_calibrator/diagnostic_serial_calibrator.ino`](firmware/diagnostic_serial_calibrator/diagnostic_serial_calibrator.ino):

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

### 6. 🧪 Testing & Verification Across Operating Systems

| OS | Test Tool / Method | What to Look For |
| :--- | :--- | :--- |
| **macOS** | Run `cargo run -p realshifter-tui` or `hidutil list` | Device appears under `VendorID: 0x2341` (Arduino) and shifts show in live TUI |
| **Windows** | Press `Win + R` ➔ type `joy.cpl` ➔ Properties | Standard 7-button gamepad; Buttons 1–7 light up red as you shift |
| **Linux** | Run `jstest /dev/input/js0` or `evtest` | Button indices 0–6 toggle `on`/`off` with zero jitter |

---

### 7. 🛠️ Troubleshooting & FAQ

- **Q: Can I use an Arduino Uno or Arduino Nano?**
  - **A:** No. Standard Uno and Nano boards use FTDI or CH340 USB-serial chips which cannot speak native USB HID. Use an **ATmega32U4** board (Arduino Leonardo or SparkFun Pro Micro 5V).
- **Q: Reverse gear triggers Gear 6 instead of Reverse.**
  - **A:** Ensure you push the shifter knob **straight down** before moving into the 6th gear slot. Check that DB9 Pin 2 is connected to Arduino Pin 2 and configured as `INPUT_PULLUP`.
- **Q: Shifting feels jittery or random buttons flicker in Neutral.**
  - **A:** Ensure the Ground (GND) wire is tightly screwed into DB9 Pin 6 and connected to the Arduino GND pin. A floating ground creates ADC noise.
- **Q: 1st gear is recognized as 3rd gear.**
  - **A:** The X-axis potentiometer reading is higher than `X_THRESHOLD_LOW`. Use the Diagnostic sketch to read your exact X value in 1st gear and increase `X_THRESHOLD_LOW` (e.g. from 400 to 450).

---

## 🏗️ Architecture & Workspace Structure

RealShifter is built as a high-performance, safe Rust workspace containing modular crates:

```
herdr-real-shifter/
├── crates/
│   ├── realshifter-core/     # Core domain logic, gear mapping, state management, profiles
│   ├── realshifter-daemon/   # HID USB listener & background event loop daemon
│   ├── realshifter-tui/      # Ratatui TUI dashboard interface
│   └── realshifter-action/   # CLI action runner & IPC trigger binary
├── firmware/
│   ├── logitech_shifter_usb/            # Ready-to-flash Arduino Leonardo / Pro Micro USB firmware
│   └── diagnostic_serial_calibrator/    # Live 115200 baud serial calibration & diagnostic sketch
├── skills/
│   └── realshifter/          # AGY Skill definition for model discovery & auto-sync
└── herdr-plugin.toml         # Herdr plugin manifest declaration
```

### Modular Crates Breakdown

1. **`realshifter-core`**: Defines gear positions (`Gear1`..`Gear6`, `Reverse`, `Neutral`), profile data models, serialization handlers (`config.json`, modular profile JSONs), and theme definitions.
2. **`realshifter-daemon`**: Connects to USB HID devices (e.g. Logitech DFS via `hidapi` or Arduino Leonardo `0x2341`), monitors gear shifts in the background, and triggers corresponding profile switches or IPC events.
3. **`realshifter-tui`**: Interactive TUI interface rendered in Herdr overlay panes using `ratatui` and `crossterm`.
4. **`realshifter-action`**: Lightweight CLI tool invoked by Herdr keybindings or external scripts to execute gear actions (`shift 1..6`, `profile next`, `on-agent-complete`).

---

## ⚙️ Configuration & Profiles

Profiles live in `~/.config/realshifter/profiles/` (or `$HERDR_PLUGIN_CONFIG_DIR/profiles/`):

- `config.json` — Global settings (`test_mode`, `preferred_terminal`, active profile)
- `profiles/agy.json` — Antigravity (AGY) model mappings and effort tiers (`low`, `medium`, `high`)
- `profiles/claude.json` — Claude Code profile mappings
- `profiles/codex.json` — Codex CLI profile mappings
- `profiles/opencode.json` — OpenCode CLI profile mappings
- `profiles/pi.json` — Pi Coding Agent profile mappings
- `profiles/custom.json` — User-defined custom command profiles

### Sample Gear Mapping Structure (`agy.json`)
```json
{
  "profile": "AgyCli",
  "_metadata": {
    "description": "RealShifter configuration snapshot of supported CLI models and effort levels.",
    "available_models": [
      {
        "id": "gemini-3.7-flash",
        "name": "Gemini 3.7 Flash",
        "effort_levels": ["low", "medium", "high"]
      }
    ]
  },
  "mappings": [
    {
      "gear": "Gear1",
      "action_type": "AgyCli",
      "command": "",
      "model": "gemini-3.7-flash-low",
      "label": "Gemini 3.7 Flash (Low)",
      "is_enabled": true
    }
  ]
}
```

---

## 🚀 Getting Started

### Prerequisites

- **OS**: macOS (native HID support), Linux, or Windows
- **Rust Toolchain**: 1.80+ (`cargo`, `rustc`)
- **Hardware**: Logitech Driving Force Shifter + Arduino USB Adapter *[Optional: Hotkeys available without hardware]*
- **Herdr**: `v0.7.0` or higher

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ergenekonyigit/herdr-real-shifter.git
cd herdr-real-shifter

# Build release binaries for all workspace crates
cargo build --release
```

The compiled binaries will be placed in `./target/release/`:
- `realshifter-daemon`
- `realshifter-tui`
- `realshifter-action`

---

## 🎮 Usage
 
### 1. Registering with Herdr (Recommended)
Link RealShifter directly to your local Herdr installation with a single command:

```bash
herdr plugin link .
```

Herdr will automatically:
- Compile release binaries (`cargo build --release`).
- Launch `realshifter-daemon --detach` in the background to listen to your USB shifter.
- Register the TUI dashboard overlay pane.
- Map keybindings: `Ctrl+Shift+1..6`, `Ctrl+Shift+R` (Reverse / New Session), `Ctrl+Shift+P` (Cycle Profile).

### 2. Manual Action Execution
You can also run actions directly from any terminal:

```bash
# Shift to Gear 1 (e.g. Gemini Flash / Haiku)
./target/release/realshifter-action shift 1

# Shift to Reverse (Opens new Herdr tab and starts active CLI session)
./target/release/realshifter-action shift reverse

# Cycle active CLI profile (AGY -> Claude -> Codex -> OpenCode -> Pi)
./target/release/realshifter-action profile next
```

### 3. Interactive Dashboard (TUI)
Launch the standalone dashboard or press your Herdr dashboard pane hotkey:

```bash
./target/release/realshifter-tui
```

---

## 🛠️ AGY Skill Integration

RealShifter comes bundled with an **Antigravity (AGY)** skill located under `skills/realshifter/`.

AGY agents can invoke the `realshifter` skill to:
1. Discover active models and effort tiers via `agy models`.
2. Sync the snapshot directly into `~/.config/realshifter/profiles/agy.json`.
3. Trigger instant gear shifts to test model configurations.

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for details.
