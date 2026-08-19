#!/usr/bin/env python3
"""
Render a pixel-perfect PNG of the RealShifter TUI dashboard.
Uses only stdlib + Pillow (pip install pillow). Falls back to generating
a self-contained SVG with fixed table cells if Pillow is unavailable.
"""

import sys
import os

# Column pixel positions (fixed, emoji-safe)
# Gear | Action Type | Label | Command/Flag | Shifts | Status
COL_GEAR    = 0
COL_TYPE    = 52
COL_LABEL   = 240
COL_CMD     = 480
COL_SHIFTS  = 810
COL_STATUS  = 880

ROWS = [
    ("N",   "",                    "",                           "",                               "0", "idle",       "normal"),
    ("1",   "🛸 Antigravity CLI",  "Gemini 3.7 Flash (Low)",    "/model gemini-3.7-flash-low",    "6", "idle",       "normal"),
    ("2",   "🛸 Antigravity CLI",  "Gemini 3.7 Flash (Medium)", "/model gemini-3.7-flash-medium",  "5", "idle",      "normal"),
    ("3",   "🛸 Antigravity CLI",  "Gemini 3.7 Flash (High)",   "/model gemini-3.7-flash-high",   "4", "idle",       "normal"),
    ("4",   "🛸 Antigravity CLI",  "Gemini 3.1 Pro (High)",     "/model gemini-3.1-pro-high",     "1", "idle",       "normal"),
    ("5",   "🛸 Antigravity CLI",  "Claude Sonnet 4.6 (Think)", "/model claude-sonnet-4-6",       "4", "idle",       "normal"),
    ("6",   "🛸 Antigravity CLI",  "Claude Opus 4.6 (Think)",   "/model claude-opus-4-6-thinking","1", "idle",       "normal"),
    ("R",   "📑 New Session (Tab)","New AGY Session (Tab)",     "agy",                            "9", "🟢 ENGAGED", "engaged"),
]

# SVG generation — works without any Python deps
def generate_svg():
    BG = "#13151a"
    TITLE_BG = "#1c1f26"
    BORDER = "#2a2d34"
    BLUE = "#62b0f0"
    GREEN = "#9ac878"
    YELLOW = "#e6c07b"
    CYAN = "#56b6c2"
    NORMAL = "#b8bfc8"
    PURPLE = "#c77dde"
    RED = "#e06c75"

    W = 1100
    TITLE_H = 42
    CONTENT_PAD_X = 22
    CONTENT_PAD_TOP = 56
    LINE_H = 22
    FS = 13
    FONT = "'SF Mono', 'Menlo', 'Monaco', monospace"
    CX = CONTENT_PAD_X  # content left edge

    # We'll use foreignObject with an HTML table inside SVG for perfect layout
    html_table_rows = ""
    for row in ROWS:
        gear, atype, label, cmd, shifts, status, style = row
        if style == "engaged":
            tr_style = f"color:{RED};font-weight:bold;"
            status_style = f"color:{GREEN};font-weight:bold;"
        elif gear == "N":
            tr_style = f"color:{PURPLE};font-weight:bold;"
            status_style = f"color:{PURPLE};"
        else:
            tr_style = f"color:{NORMAL};"
            status_style = f"color:{NORMAL};"

        cursor = "▶" if gear == "N" else " "
        html_table_rows += f"""
        <tr style="{tr_style}">
          <td style="width:52px;padding:1px 4px;">{cursor} {gear}</td>
          <td style="width:192px;padding:1px 8px;">{atype if atype else '—'}</td>
          <td style="width:244px;padding:1px 8px;">{label if label else '—'}</td>
          <td style="width:330px;padding:1px 8px;">{cmd if cmd else '—'}</td>
          <td style="width:58px;padding:1px 8px;text-align:right;">{shifts}</td>
          <td style="padding:1px 8px;{status_style}">{status}</td>
        </tr>"""

    SVG_W = 1160
    SVG_H = 680
    SHADOW = 28

    # Build full SVG with embedded foreignObject HTML
    svg = f"""<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xhtml="http://www.w3.org/1999/xhtml"
     width="{SVG_W}" height="{SVG_H}" viewBox="0 0 {SVG_W} {SVG_H}">
  <defs>
    <filter id="shadow" x="-5%" y="-5%" width="120%" height="130%">
      <feDropShadow dx="0" dy="10" stdDeviation="14" flood-color="rgba(0,0,0,0.6)"/>
    </filter>
    <clipPath id="win-clip">
      <rect x="{SHADOW}" y="{SHADOW}" width="{SVG_W - SHADOW*2}" height="{SVG_H - SHADOW*2}" rx="10" ry="10"/>
    </clipPath>
  </defs>

  <!-- Window body -->
  <rect x="{SHADOW}" y="{SHADOW}" width="{SVG_W - SHADOW*2}" height="{SVG_H - SHADOW*2}"
        rx="10" ry="10" fill="{BG}" filter="url(#shadow)" stroke="{BORDER}" stroke-width="1.5"/>

  <!-- Title bar -->
  <rect x="{SHADOW}" y="{SHADOW}" width="{SVG_W - SHADOW*2}" height="{TITLE_H}"
        rx="10" ry="10" fill="{TITLE_BG}" clip-path="url(#win-clip)"/>
  <line x1="{SHADOW}" y1="{SHADOW + TITLE_H}" x2="{SVG_W - SHADOW}" y2="{SHADOW + TITLE_H}"
        stroke="{BORDER}" stroke-width="1"/>

  <!-- Traffic lights -->
  <circle cx="{SHADOW + 18}" cy="{SHADOW + TITLE_H//2}" r="6" fill="#ff5f57"/>
  <circle cx="{SHADOW + 36}" cy="{SHADOW + TITLE_H//2}" r="6" fill="#ffbd2e"/>
  <circle cx="{SHADOW + 54}" cy="{SHADOW + TITLE_H//2}" r="6" fill="#28c940"/>

  <!-- Window title -->
  <text x="{SVG_W // 2}" y="{SHADOW + TITLE_H//2 + 5}" text-anchor="middle"
        font-family="-apple-system, 'Helvetica Neue', sans-serif" font-size="12" font-weight="600"
        fill="#8b909e">RealShifter TUI Dashboard — Herdr Overlay</text>

  <!-- Content via foreignObject -->
  <foreignObject x="{SHADOW}" y="{SHADOW + TITLE_H}" width="{SVG_W - SHADOW*2}" height="{SVG_H - SHADOW - TITLE_H}">
    <xhtml:div xmlns="http://www.w3.org/1999/xhtml" style="
      background:{BG};
      color:{NORMAL};
      font-family:'SF Mono','Menlo','Monaco',monospace;
      font-size:13px;
      line-height:1.55;
      padding:14px 22px 18px;
      height:100%;
      overflow:hidden;
    ">
      <!-- Header -->
      <pre style="color:{BLUE};font-weight:bold;white-space:pre;">┌ RealShifter Dashboard ─────────────────────────────────────────────────────────────────────────────────────────────────────┐</pre>
      <pre style="color:{GREEN};font-weight:bold;white-space:pre;">│  RealShifter v0.1.0  [Theme: Dark]  |  🟢 ACTIVE: Antigravity (AGY)  | Current Gear: [R]                                  │</pre>
      <pre style="color:{BLUE};font-weight:bold;white-space:pre;">└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘</pre>

      <!-- Profile Tabs -->
      <pre style="color:{BLUE};font-weight:bold;margin-top:6px;white-space:pre;">┌ Profile View [Press 'h/l' or 'p' to switch | 'Space' to set Active] ──────────────────────────────────────────────────────┐</pre>
      <pre style="color:{NORMAL};white-space:pre;">│  🟢 🛸 Antigravity (AGY)  │  🧠 Claude Code  │  💻 Codex CLI  │  ⚡ OpenCode CLI  │  π Pi Agent  │  🎛️ Custom / Multi-Tool  │</pre>
      <pre style="color:{BLUE};font-weight:bold;white-space:pre;">└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘</pre>

      <!-- Gear Grid -->
      <pre style="color:{BLUE};font-weight:bold;margin-top:6px;white-space:pre;">┌ Gear Mapping Status Grid [Use 'j/k' to select, 'Enter/e' to edit] ────────────────────────────────────────────────────────┐</pre>
      <xhtml:table style="width:100%;border-collapse:collapse;table-layout:fixed;">
        <xhtml:colgroup>
          <xhtml:col style="width:52px;"/>
          <xhtml:col style="width:192px;"/>
          <xhtml:col style="width:244px;"/>
          <xhtml:col style="width:330px;"/>
          <xhtml:col style="width:62px;"/>
          <xhtml:col style="width:auto;"/>
        </xhtml:colgroup>
        <xhtml:tr style="color:{YELLOW};font-weight:bold;">
          <xhtml:td style="padding:1px 4px;">│ Gear</xhtml:td>
          <xhtml:td style="padding:1px 8px;">Action Type</xhtml:td>
          <xhtml:td style="padding:1px 8px;">Label</xhtml:td>
          <xhtml:td style="padding:1px 8px;">Command / Flag</xhtml:td>
          <xhtml:td style="padding:1px 8px;text-align:right;">Shifts</xhtml:td>
          <xhtml:td style="padding:1px 8px;">Status  │</xhtml:td>
        </xhtml:tr>
        <xhtml:tr style="color:{NORMAL};">
          <xhtml:td colspan="6" style="padding:2px 4px;">│</xhtml:td>
        </xhtml:tr>
        {html_table_rows}
      </xhtml:table>
      <pre style="color:{BLUE};font-weight:bold;white-space:pre;">└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘</pre>

      <!-- Status Bar -->
      <pre style="color:{BLUE};font-weight:bold;margin-top:6px;white-space:pre;">┌ Status &amp; Controls ─────────────────────────────────────────────────────────────────────────────────────────────────────────┐</pre>
      <pre style="color:{NORMAL};white-space:pre;">│  Last Action: New AGY Session (Tab)  |  Total Shifts: 30  |  Device: Connected (Arduino Leonardo)                         │</pre>
      <pre style="color:{CYAN};white-space:pre;">│  Controls: [j/k]: Nav  |  [h/l/p]: Tab  |  [Space]: Active  |  [e/Enter]: Edit  |  [t]: Theme  |  [1-6/r]: Shift  |  [q]: Exit  │</pre>
      <pre style="color:{BLUE};font-weight:bold;white-space:pre;">└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘</pre>
    </xhtml:div>
  </foreignObject>
</svg>"""

    return svg


def main():
    os.makedirs("assets", exist_ok=True)

    svg_path = "assets/realshifter-dashboard.svg"
    png_path = "assets/realshifter-dashboard.png"

    svg = generate_svg()
    with open(svg_path, "w", encoding="utf-8") as f:
        f.write(svg)
    print(f"SVG written to {svg_path}")

    # Try to convert with rsvg-convert, then cairosvg, then qlmanage
    converted = False

    # Option 1: rsvg-convert (homebrew librsvg)
    ret = os.system(f"which rsvg-convert > /dev/null 2>&1 && rsvg-convert -w 2320 -h 1360 -o {png_path} {svg_path}")
    if ret == 0 and os.path.exists(png_path) and os.path.getsize(png_path) > 1000:
        print(f"PNG written via rsvg-convert to {png_path}")
        converted = True

    # Option 2: cairosvg python package
    if not converted:
        try:
            import cairosvg
            cairosvg.svg2png(url=svg_path, write_to=png_path, scale=2)
            print(f"PNG written via cairosvg to {png_path}")
            converted = True
        except ImportError:
            pass

    # Option 3: qlmanage (Quick Look, macOS built-in)
    if not converted:
        ret = os.system(f"qlmanage -t -s 2320 -o assets/ {svg_path} 2>/dev/null && mv assets/realshifter-dashboard.svg.png {png_path} 2>/dev/null")
        if ret == 0 and os.path.exists(png_path) and os.path.getsize(png_path) > 1000:
            print(f"PNG written via qlmanage to {png_path}")
            converted = True

    # Option 4: Swift CoreGraphics fallback (precise, emoji-safe table cells)
    if not converted:
        swift_code = generate_swift_fallback()
        swift_path = "/tmp/render_tui_fallback.swift"
        with open(swift_path, "w") as sf:
            sf.write(swift_code)
        ret = os.system(f"swift {swift_path}")
        if ret == 0:
            print(f"PNG written via Swift fallback to {png_path}")
            converted = True

    if not converted:
        print("WARNING: Could not produce PNG. SVG is available at", svg_path)
        sys.exit(1)


def generate_swift_fallback():
    """Pure CoreGraphics renderer with fixed pixel column positions — emoji-safe."""
    return r"""
import Cocoa

let BG = NSColor(calibratedRed: 0.075, green: 0.083, blue: 0.102, alpha: 1)
let TITLE_BG = NSColor(calibratedRed: 0.11, green: 0.122, blue: 0.149, alpha: 1)
let BORDER = NSColor(calibratedRed: 0.165, green: 0.177, blue: 0.204, alpha: 1)
let C_BLUE   = NSColor(calibratedRed: 0.384, green: 0.690, blue: 0.941, alpha: 1)
let C_GREEN  = NSColor(calibratedRed: 0.604, green: 0.784, blue: 0.471, alpha: 1)
let C_YELLOW = NSColor(calibratedRed: 0.902, green: 0.753, blue: 0.482, alpha: 1)
let C_CYAN   = NSColor(calibratedRed: 0.337, green: 0.714, blue: 0.761, alpha: 1)
let C_NORMAL = NSColor(calibratedRed: 0.722, green: 0.749, blue: 0.800, alpha: 1)
let C_PURPLE = NSColor(calibratedRed: 0.780, green: 0.490, blue: 0.871, alpha: 1)
let C_RED    = NSColor(calibratedRed: 0.878, green: 0.424, blue: 0.459, alpha: 1)

let W: CGFloat = 1160
let H: CGFloat = 680
let SHADOW: CGFloat = 28
let TITLE_H: CGFloat = 42
let FS: CGFloat = 13
let LH: CGFloat = 22
let PX: CGFloat = 26    // content left padding
let PY: CGFloat = SHADOW + TITLE_H + 14  // content top

let font = NSFont.monospacedSystemFont(ofSize: FS, weight: .regular)
let boldFont = NSFont.monospacedSystemFont(ofSize: FS, weight: .bold)

func attr(_ s: String, _ c: NSColor, bold: Bool = false) -> NSAttributedString {
    NSAttributedString(string: s, attributes: [
        .font: bold ? boldFont : font,
        .foregroundColor: c
    ])
}

func drawText(_ s: NSAttributedString, x: CGFloat, y: CGFloat) {
    s.draw(at: CGPoint(x: x, y: y))
}

func drawLine(_ text: String, _ color: NSColor, y: CGFloat, bold: Bool = false) {
    drawText(attr(text, color, bold: bold), x: PX, y: y)
}

let size = NSSize(width: W, height: H)
let image = NSImage(size: size)
image.lockFocus()
let ctx = NSGraphicsContext.current!.cgContext

// Clear
ctx.clear(CGRect(origin: .zero, size: size))

// Window shadow + body
let winRect = CGRect(x: SHADOW, y: SHADOW, width: W - SHADOW*2, height: H - SHADOW*2)
let winPath = CGPath(roundedRect: winRect, cornerWidth: 10, cornerHeight: 10, transform: nil)
ctx.saveGState()
ctx.setShadow(offset: CGSize(width: 0, height: -12), blur: 22, color: NSColor.black.withAlphaComponent(0.65).cgColor)
ctx.setFillColor(BG.cgColor)
ctx.addPath(winPath)
ctx.fillPath()
ctx.restoreGState()

ctx.setStrokeColor(BORDER.cgColor)
ctx.setLineWidth(1.5)
ctx.addPath(winPath)
ctx.strokePath()

// Title bar
let tbRect = CGRect(x: SHADOW, y: SHADOW + H - SHADOW*2 - TITLE_H, width: W - SHADOW*2, height: TITLE_H)
ctx.saveGState()
let tbPath = CGPath(roundedRect: CGRect(x: SHADOW, y: SHADOW + (H-SHADOW*2-TITLE_H), width: W-SHADOW*2, height: TITLE_H+12), cornerWidth: 10, cornerHeight: 10, transform: nil)
ctx.addPath(tbPath)
ctx.clip()
ctx.setFillColor(TITLE_BG.cgColor)
ctx.fill(tbRect)
ctx.restoreGState()

// Title bar separator
ctx.setStrokeColor(BORDER.cgColor)
ctx.setLineWidth(1)
let sepY = SHADOW + (H - SHADOW*2) - TITLE_H
ctx.strokeLineSegments(between: [CGPoint(x: SHADOW, y: sepY), CGPoint(x: W-SHADOW, y: sepY)])

// Traffic lights
let btnY = SHADOW + (H - SHADOW*2) - TITLE_H/2
let btnColors: [NSColor] = [
    NSColor(calibratedRed: 1, green: 0.373, blue: 0.341, alpha: 1),
    NSColor(calibratedRed: 1, green: 0.741, blue: 0.184, alpha: 1),
    NSColor(calibratedRed: 0.157, green: 0.788, blue: 0.251, alpha: 1)
]
for (i, c) in btnColors.enumerated() {
    ctx.setFillColor(c.cgColor)
    ctx.fillEllipse(in: CGRect(x: SHADOW + 18 + CGFloat(i*18) - 6, y: btnY - 6, width: 12, height: 12))
}

// Window title text
let winTitleAttr = NSAttributedString(string: "RealShifter TUI Dashboard — Herdr Overlay", attributes: [
    .font: NSFont.systemFont(ofSize: 12, weight: .semibold),
    .foregroundColor: NSColor(calibratedRed: 0.545, green: 0.565, blue: 0.620, alpha: 1)
])
let ts = winTitleAttr.size()
winTitleAttr.draw(at: CGPoint(x: (W - ts.width)/2, y: btnY - ts.height/2))

// ── Content ──
// Fixed col X positions (relative to PX)
let cGear: CGFloat  = 0
let cType: CGFloat  = 52
let cLabel: CGFloat = 244
let cCmd: CGFloat   = 488
let cShif: CGFloat  = 818
let cStat: CGFloat  = 888

struct Row {
    let gear: String; let atype: String; let label: String
    let cmd: String; let shifts: String; let status: String; let style: String
}

let rows: [Row] = [
    Row(gear:"N", atype:"", label:"", cmd:"", shifts:"0", status:"idle", style:"neutral"),
    Row(gear:"1", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (Low)",    cmd:"/model gemini-3.7-flash-low",     shifts:"6", status:"idle", style:"normal"),
    Row(gear:"2", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (Medium)", cmd:"/model gemini-3.7-flash-medium",   shifts:"5", status:"idle", style:"normal"),
    Row(gear:"3", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (High)",   cmd:"/model gemini-3.7-flash-high",    shifts:"4", status:"idle", style:"normal"),
    Row(gear:"4", atype:"🛸 Antigravity CLI", label:"Gemini 3.1 Pro (High)",     cmd:"/model gemini-3.1-pro-high",      shifts:"1", status:"idle", style:"normal"),
    Row(gear:"5", atype:"🛸 Antigravity CLI", label:"Claude Sonnet 4.6 (Think)", cmd:"/model claude-sonnet-4-6",        shifts:"4", status:"idle", style:"normal"),
    Row(gear:"6", atype:"🛸 Antigravity CLI", label:"Claude Opus 4.6 (Think)",   cmd:"/model claude-opus-4-6-thinking", shifts:"1", status:"idle", style:"normal"),
    Row(gear:"R", atype:"📑 New Session (Tab)", label:"New AGY Session (Tab)",   cmd:"agy",                             shifts:"9", status:"🟢 ENGAGED", style:"engaged"),
]

var y = PY

func hr(_ color: NSColor = C_BLUE, bold: Bool = true) -> String { return "" }

// Section helper: draws a ──── bordered line at `y`
func sectionLine(_ text: String, y: CGFloat, color: NSColor = C_BLUE, bold: Bool = false) {
    drawText(attr(text, color, bold: bold), x: PX, y: y)
}

let BOX_W: Int = 112  // visual char width of box

func box_top(_ title: String) -> String {
    let inner = " \(title) "
    let fill = max(0, BOX_W - 2 - inner.count)
    return "┌" + inner + String(repeating: "─", count: fill) + "┐"
}
func box_bot() -> String { "└" + String(repeating: "─", count: BOX_W - 0) + "┘" }
func box_line(_ s: String) -> String {
    let padded = s + String(repeating: " ", count: max(0, BOX_W - s.count - 2))
    return "│ " + padded + "│"
}

// ── Header ──
sectionLine(box_top("RealShifter Dashboard"), y: y, color: C_BLUE, bold: true); y += LH
sectionLine(box_line("RealShifter v0.1.0  [Theme: Dark]  |  🟢 ACTIVE: Antigravity (AGY)  | Current Gear: [R]"), y: y, color: C_GREEN, bold: true); y += LH
sectionLine(box_bot(), y: y, color: C_BLUE, bold: true); y += LH + 4

// ── Profile Tabs ──
sectionLine(box_top("Profile View [Press 'h/l' or 'p' to switch | 'Space' to set Active]"), y: y, color: C_BLUE, bold: true); y += LH
sectionLine(box_line("🟢 🛸 Antigravity (AGY)  │  🧠 Claude Code  │  💻 Codex CLI  │  ⚡ OpenCode CLI  │  π Pi  │  🎛️ Custom"), y: y, color: C_NORMAL); y += LH
sectionLine(box_bot(), y: y, color: C_BLUE, bold: true); y += LH + 4

// ── Gear Grid ──
sectionLine(box_top("Gear Mapping Status Grid [Use 'j/k' to select, 'Enter/e' to edit]"), y: y, color: C_BLUE, bold: true); y += LH

// Header row with fixed pixel columns
func drawCols(gear: String, atype: String, label: String, cmd: String, shifts: String, status: String,
              color: NSColor, statusColor: NSColor? = nil, bold: Bool = false, cursor: Bool = false) {
    let barX = PX + 0
    drawText(attr("│", C_BLUE, bold: true), x: barX, y: y)
    let gearStr = (cursor ? "▶  " : "   ") + gear
    drawText(attr(gearStr, color, bold: bold), x: PX + cGear + 6, y: y)
    if !atype.isEmpty { drawText(attr(atype, color, bold: false), x: PX + cType, y: y) }
    if !label.isEmpty { drawText(attr(label, color, bold: false), x: PX + cLabel, y: y) }
    if !cmd.isEmpty   { drawText(attr(cmd, color, bold: false), x: PX + cCmd, y: y) }
    drawText(attr(shifts, color, bold: bold), x: PX + cShif, y: y)
    drawText(attr(status, statusColor ?? color, bold: bold), x: PX + cStat, y: y)
    // right border
    let rightX = W - SHADOW - PX + 2
    drawText(attr("│", C_BLUE, bold: true), x: rightX, y: y)
}

drawCols(gear: "Gear", atype: "Action Type", label: "Label", cmd: "Command / Flag", shifts: "Shifts", status: "Status  │",
         color: C_YELLOW, bold: true)
y += LH
drawText(attr("│", C_BLUE, bold: true), x: PX, y: y); y += LH

for row in rows {
    let isCurrent = row.gear == "N"
    let isEngaged = row.style == "engaged"
    let color: NSColor = isCurrent ? C_PURPLE : (isEngaged ? C_RED : C_NORMAL)
    let statColor: NSColor = isEngaged ? C_GREEN : color
    let displayAtype = row.atype.isEmpty ? "—" : row.atype
    let displayLabel = row.label.isEmpty ? "—" : row.label
    let displayCmd   = row.cmd.isEmpty   ? "—" : row.cmd
    drawCols(gear: row.gear, atype: displayAtype, label: displayLabel, cmd: displayCmd,
             shifts: row.shifts, status: row.status,
             color: color, statusColor: statColor, bold: isCurrent || isEngaged, cursor: isCurrent)
    y += LH
}
sectionLine(box_bot(), y: y, color: C_BLUE, bold: true); y += LH + 4

// ── Status ──
sectionLine(box_top("Status & Controls"), y: y, color: C_BLUE, bold: true); y += LH
sectionLine(box_line("Last Action: New AGY Session (Tab)  |  Total Shifts: 30  |  Device: Connected (Arduino Leonardo)"), y: y, color: C_NORMAL); y += LH
sectionLine(box_line("Controls: [j/k]: Nav  |  [h/l/p]: Tab  |  [Space]: Active  |  [e/Enter]: Edit  |  [t]: Theme  |  [1-6/r]: Shift  |  [q]: Exit"), y: y, color: C_CYAN); y += LH
sectionLine(box_bot(), y: y, color: C_BLUE, bold: true)

image.unlockFocus()
if let tiff = image.tiffRepresentation,
   let rep = NSBitmapImageRep(data: tiff),
   let png = rep.representation(using: .png, properties: [:]) {
    try? png.write(to: URL(fileURLWithPath: "assets/realshifter-dashboard.png"))
    print("PNG saved.")
} else {
    print("PNG failed.")
}
"""


if __name__ == "__main__":
    main()
