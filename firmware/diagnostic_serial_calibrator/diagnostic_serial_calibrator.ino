/*
 * ============================================================================
 * Logitech Shifter Hardware Diagnostic & Calibration Tool
 * ============================================================================
 * 
 * Upload this sketch to your Arduino Leonardo / Pro Micro to:
 * 1. Verify your wiring without needing any game or joystick software.
 * 2. View live real-time X-Axis (A0) and Y-Axis (A2) ADC values (0-1023).
 * 3. Inspect Reverse microswitch state on Pin 2.
 * 4. Determine precise threshold values for your specific shifter hardware.
 * 
 * Instructions:
 * 1. Upload sketch to Arduino Leonardo / Pro Micro.
 * 2. Open Arduino IDE Serial Monitor (Baud rate: 115200).
 * 3. Move shifter through all gears (1-6, Neutral, Reverse).
 * ============================================================================
 */

const int PIN_X_AXIS = A0;     // DB9 Pin 4
const int PIN_Y_AXIS = A2;     // DB9 Pin 8
const int PIN_REVERSE = 2;     // DB9 Pin 2 (Active LOW)

void setup() {
  Serial.begin(115200);
  pinMode(PIN_REVERSE, INPUT_PULLUP);
  pinMode(PIN_X_AXIS, INPUT);
  pinMode(PIN_Y_AXIS, INPUT);
  
  while (!Serial && millis() < 3000); // Wait for Serial connection on Leonardo
  
  Serial.println("==========================================================");
  Serial.println("🏎️  Logitech Shifter Diagnostic & Calibration Tool");
  Serial.println("==========================================================");
  Serial.println("Move the shifter stick to all positions (1-6, N, Reverse).");
  Serial.println("Check X/Y values and verify Reverse switch toggles to PRESSED.");
  Serial.println("----------------------------------------------------------");
}

void loop() {
  int xVal = analogRead(PIN_X_AXIS);
  int yVal = analogRead(PIN_Y_AXIS);
  bool revPressed = (digitalRead(PIN_REVERSE) == LOW);

  String detectedGear = "Neutral";

  // Reverse Gate Check
  if (revPressed && xVal > 650 && yVal < 400) {
    detectedGear = "🔴 REVERSE (R)";
  }
  // Left Column
  else if (xVal < 400) {
    if (yVal > 650)      detectedGear = "1️⃣  GEAR 1";
    else if (yVal < 400) detectedGear = "2️⃣  GEAR 2";
  }
  // Center Column
  else if (xVal <= 650) {
    if (yVal > 650)      detectedGear = "3️⃣  GEAR 3";
    else if (yVal < 400) detectedGear = "4️⃣  GEAR 4";
  }
  // Right Column
  else {
    if (yVal > 650)      detectedGear = "5️⃣  GEAR 5";
    else if (yVal < 400) detectedGear = "6️⃣  GEAR 6";
  }

  Serial.print("X (A0): ");
  if (xVal < 100) Serial.print(" ");
  if (xVal < 10)  Serial.print(" ");
  Serial.print(xVal);

  Serial.print("  | Y (A2): ");
  if (yVal < 100) Serial.print(" ");
  if (yVal < 10)  Serial.print(" ");
  Serial.print(yVal);

  Serial.print("  | Rev Switch (D2): ");
  if (revPressed) {
    Serial.print("[PRESSED (LOW)] ");
  } else {
    Serial.print("[RELEASED (HIGH)]");
  }

  Serial.print("  | Detected: ");
  Serial.println(detectedGear);

  delay(120);
}
