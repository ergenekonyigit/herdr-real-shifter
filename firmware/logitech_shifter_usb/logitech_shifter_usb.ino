/*
 * ============================================================================
 * Logitech Driving Force Shifter USB HID Adapter (No Soldering)
 * Compatible with G29 / G920 / G923 Shifter on Arduino Leonardo / Pro Micro
 * ============================================================================
 * 
 * Hardware Wiring (DB9 Male to Arduino Leonardo / Pro Micro):
 * -----------------------------------------------------------
 * DB9 Pin 2  -> Arduino Digital Pin 2  (Reverse Microswitch - Active Low)
 * DB9 Pin 4  -> Arduino Analog Pin A0  (X-Axis Potentiometer)
 * DB9 Pin 6  -> Arduino GND            (Ground)
 * DB9 Pin 8  -> Arduino Analog Pin A2  (Y-Axis Potentiometer)
 * DB9 Pin 9  -> Arduino 5V             (Power VCC)
 * 
 * Software Dependency:
 * Install "Joystick" by Matthew Heironimus via Arduino Library Manager
 * (https://github.com/MHeironimus/ArduinoJoystickLibrary)
 * ============================================================================
 */

#include <Joystick.h>

// Pin Definitions
const int PIN_X_AXIS = A0;     // DB9 Pin 4 (X-Axis: Left <-> Right)
const int PIN_Y_AXIS = A2;     // DB9 Pin 8 (Y-Axis: Top <-> Bottom)
const int PIN_REVERSE = 2;     // DB9 Pin 2 (Reverse Push Switch: Active LOW)

// Calibration Thresholds (ADC 0 - 1023)
// Center resting position is approx X=500..550, Y=500..550
const int X_THRESHOLD_LOW  = 400; // Left column (Gears 1 & 2)
const int X_THRESHOLD_HIGH = 650; // Right column (Gears 5, 6 & Reverse)
const int Y_THRESHOLD_LOW  = 400; // Bottom row (Gears 2, 4, 6, Reverse)
const int Y_THRESHOLD_HIGH = 650; // Top row (Gears 1, 3, 5)

// Joystick Configuration: 7 Buttons (1-6 Gears + Reverse), No Axes
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

  Joystick.begin(false); // false = manual sendState updates for optimal latency
}

void loop() {
  int xVal = analogRead(PIN_X_AXIS);
  int yVal = analogRead(PIN_Y_AXIS);
  bool isReversePressed = (digitalRead(PIN_REVERSE) == LOW);

  int currentGear = 0; // Default: Neutral (0)

  // Reverse Gear: Stick pressed down AND in bottom-right gate
  if (isReversePressed && xVal > X_THRESHOLD_HIGH && yVal < Y_THRESHOLD_LOW) {
    currentGear = 7; // Reverse (Button 7 / Index 6)
  }
  // Left Column (Gears 1 & 2)
  else if (xVal < X_THRESHOLD_LOW) {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 1; // Gear 1 (Top-Left)
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 2; // Gear 2 (Bottom-Left)
  }
  // Center Column (Gears 3 & 4)
  else if (xVal <= X_THRESHOLD_HIGH) {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 3; // Gear 3 (Top-Center)
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 4; // Gear 4 (Bottom-Center)
  }
  // Right Column (Gears 5 & 6)
  else {
    if (yVal > Y_THRESHOLD_HIGH)      currentGear = 5; // Gear 5 (Top-Right)
    else if (yVal < Y_THRESHOLD_LOW) currentGear = 6; // Gear 6 (Bottom-Right)
  }

  // Update HID Joystick buttons when position changes
  if (currentGear != lastGear) {
    // Release all buttons first
    for (int i = 0; i < 7; i++) {
      Joystick.setButton(i, 0);
    }

    // Press active button (0-indexed: 0=Gear1 ... 5=Gear6, 6=Reverse)
    if (currentGear >= 1 && currentGear <= 7) {
      Joystick.setButton(currentGear - 1, 1);
    }

    Joystick.sendState();
    lastGear = currentGear;
  }

  delay(10); // 10ms polling rate (100Hz)
}
