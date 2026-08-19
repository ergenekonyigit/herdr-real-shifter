import Cocoa

// ── Palette ──
let BG      = NSColor(calibratedRed: 0.075, green: 0.083, blue: 0.102, alpha: 1)
let TITLEBG = NSColor(calibratedRed: 0.11,  green: 0.122, blue: 0.149, alpha: 1)
let BORD    = NSColor(calibratedRed: 0.165, green: 0.177, blue: 0.204, alpha: 1)
let BLUE    = NSColor(calibratedRed: 0.384, green: 0.690, blue: 0.941, alpha: 1)
let GREEN   = NSColor(calibratedRed: 0.604, green: 0.784, blue: 0.471, alpha: 1)
let YELLOW  = NSColor(calibratedRed: 0.902, green: 0.753, blue: 0.482, alpha: 1)
let CYAN    = NSColor(calibratedRed: 0.337, green: 0.714, blue: 0.761, alpha: 1)
let NORM    = NSColor(calibratedRed: 0.722, green: 0.749, blue: 0.800, alpha: 1)
let PURPLE  = NSColor(calibratedRed: 0.780, green: 0.490, blue: 0.871, alpha: 1)
let RED     = NSColor(calibratedRed: 0.878, green: 0.424, blue: 0.459, alpha: 1)

let FS: CGFloat  = 13
let LH: CGFloat  = 21
let SHADOW: CGFloat = 26
let TITLE_H: CGFloat = 40

// ── Fonts ──
let font     = NSFont.monospacedSystemFont(ofSize: FS, weight: .regular)
let boldFont = NSFont.monospacedSystemFont(ofSize: FS, weight: .bold)

func a(_ s: String, _ c: NSColor, bold: Bool = false) -> NSAttributedString {
    NSAttributedString(string: s, attributes: [
        .font: bold ? boldFont : font,
        .foregroundColor: c
    ])
}

// Measure single char width from actual font metrics
let charW: CGFloat = {
    let s = NSAttributedString(string: "A", attributes: [.font: font])
    return s.size().width
}()

// ── Canvas ──
// 130 visible chars wide + side paddings + shadow
let COLS = 138
let PAD_X: CGFloat = 22
let PAD_BOT: CGFloat = 22
let contentW = CGFloat(COLS) * charW + PAD_X * 2
let W = contentW + SHADOW * 2

// Count content rows — generous so bottom border never clips outside window
let CONTENT_ROWS = 26  // header(3)+gap(1)+tabs(3)+gap(1)+grid(12)+gap(1)+status(4)+bottom-pad(1)
let contentH = TITLE_H + CGFloat(CONTENT_ROWS) * LH + PAD_BOT
let H = contentH + SHADOW * 2



let size = NSSize(width: W, height: H)
let image = NSImage(size: size)
image.lockFocus()
let ctx = NSGraphicsContext.current!.cgContext

// Clear transparent
ctx.clear(CGRect(origin: .zero, size: size))

// ── Window body ──
let winRect = CGRect(x: SHADOW, y: SHADOW, width: contentW, height: contentH)
let winPath = CGPath(roundedRect: winRect, cornerWidth: 10, cornerHeight: 10, transform: nil)

ctx.saveGState()
ctx.setShadow(offset: CGSize(width: 0, height: -12), blur: 22,
              color: NSColor.black.withAlphaComponent(0.65).cgColor)
ctx.setFillColor(BG.cgColor)
ctx.addPath(winPath)
ctx.fillPath()
ctx.restoreGState()

ctx.setStrokeColor(BORD.cgColor)
ctx.setLineWidth(1.5)
ctx.addPath(winPath)
ctx.strokePath()

// ── Title bar ──
let tbY = SHADOW + contentH - TITLE_H
let tbRect = CGRect(x: SHADOW, y: tbY, width: contentW, height: TITLE_H)
let tbClipPath = CGPath(roundedRect: CGRect(x: SHADOW, y: tbY, width: contentW, height: TITLE_H + 12),
                         cornerWidth: 10, cornerHeight: 10, transform: nil)
ctx.saveGState()
ctx.addPath(tbClipPath)
ctx.clip()
ctx.setFillColor(TITLEBG.cgColor)
ctx.fill(tbRect)
ctx.restoreGState()

ctx.setStrokeColor(BORD.cgColor)
ctx.setLineWidth(1)
ctx.strokeLineSegments(between: [CGPoint(x: SHADOW, y: tbY), CGPoint(x: SHADOW + contentW, y: tbY)])

// Traffic lights
let btnY = tbY + TITLE_H / 2
[(SHADOW + 18, NSColor(calibratedRed: 1, green: 0.373, blue: 0.341, alpha: 1)),
 (SHADOW + 36, NSColor(calibratedRed: 1, green: 0.741, blue: 0.184, alpha: 1)),
 (SHADOW + 54, NSColor(calibratedRed: 0.157, green: 0.788, blue: 0.251, alpha: 1))].forEach { (bx, bc) in
    ctx.setFillColor(bc.cgColor)
    ctx.fillEllipse(in: CGRect(x: CGFloat(bx) - 5.5, y: btnY - 5.5, width: 11, height: 11))
}

// Window title
let winTitle = NSAttributedString(string: "RealShifter TUI Dashboard — Herdr Overlay", attributes: [
    .font: NSFont.systemFont(ofSize: 12, weight: .semibold),
    .foregroundColor: NSColor(calibratedRed: 0.545, green: 0.565, blue: 0.620, alpha: 1)
])
let wts = winTitle.size()
winTitle.draw(at: CGPoint(x: SHADOW + (contentW - wts.width) / 2, y: btnY - wts.height / 2))

// ── Content drawing ──
// NSImage coordinate origin is bottom-left.
// We draw top-to-bottom by tracking `lineY` starting just below title bar.

var lineIdx = 0  // increments per line drawn

// Convert line index to actual bottom-left Y coordinate
func Y(_ idx: Int) -> CGFloat {
    // First line drawn is at top of content area (just below title bar).
    // Content top Y = SHADOW + contentH - TITLE_H - LH (first row baseline)
    let contentTop = SHADOW + contentH - TITLE_H - LH - 2
    return contentTop - CGFloat(idx) * LH
}

func drawStr(_ s: NSAttributedString, lineIndex: Int, xOffset: CGFloat = 0) {
    s.draw(at: CGPoint(x: SHADOW + PAD_X + xOffset, y: Y(lineIndex)))
}

func line(_ text: String, _ color: NSColor, bold: Bool = false) {
    drawStr(a(text, color, bold: bold), lineIndex: lineIdx)
    lineIdx += 1
}

// Box helpers — all 112-char wide inner content
let INNER = 134
func boxTop(_ title: String) -> String {
    let inner = " \(title) "
    let fill = max(0, INNER - inner.count)
    return "┌\(inner)\(String(repeating: "─", count: fill))┐"
}
func boxBot() -> String { "└\(String(repeating: "─", count: INNER))┘" }
func boxRow(_ s: String) -> String {
    var display = s
    // strip to INNER-2 visible width (rough; monospace ASCII/emoji mix)
    if display.count > INNER - 2 { display = String(display.prefix(INNER - 2)) }
    let pad = max(0, INNER - 2 - display.count)
    return "│ \(display)\(String(repeating: " ", count: pad)) │"
}

// ── Header ──
line(boxTop("RealShifter Dashboard"), BLUE, bold: true)
line(boxRow("RealShifter v0.1.0  [Theme: Dark]  |  🟢 ACTIVE: Antigravity (AGY)  | Current Gear: [R]"), GREEN, bold: true)
line(boxBot(), BLUE, bold: true)
lineIdx += 1  // gap

// ── Profile Tabs ──
line(boxTop("Profile View [Press 'h/l' or 'p' to switch | 'Space' to set Active]"), BLUE, bold: true)
line(boxRow("🟢 🛸 Antigravity (AGY)  │  🧠 Claude Code  │  💻 Codex CLI  │  ⚡ OpenCode CLI  │  π Pi  │  🎛️ Custom"), NORM)
line(boxBot(), BLUE, bold: true)
lineIdx += 1  // gap

// ── Gear Grid — draw border lines but table row content with fixed pixel offsets ──
line(boxTop("Gear Mapping Status Grid [Use 'j/k' to select, 'Enter/e' to edit]"), BLUE, bold: true)

// Column X offsets (pixels from left pad)
let cBar:  CGFloat = 0
let cGear: CGFloat = charW * 2
let cType: CGFloat = charW * 9    // wider gap: "Gear" is 4 chars + 3 spaces
let cLbl:  CGFloat = charW * 29
let cCmd:  CGFloat = charW * 54
let cShi:  CGFloat = charW * 88
let cStat: CGFloat = charW * 95

func drawRowAt(idx: Int, gear: String, atype: String, label: String,
               cmd: String, shifts: String, status: String,
               color: NSColor, statColor: NSColor? = nil,
               bold: Bool = false, cursor: Bool = false) {
    let sc = statColor ?? color
    let yi = Y(idx)
    let lx = SHADOW + PAD_X
    // Left border: aligns with the '│' at char position 0 of boxRow strings
    a("│", BLUE, bold: true).draw(at: CGPoint(x: lx, y: yi))
    let gStr = (cursor ? "▶ " : "  ") + gear
    a(gStr, color, bold: bold).draw(at: CGPoint(x: lx + cGear, y: yi))
    if !atype.isEmpty  { a(atype,  color, bold: false).draw(at: CGPoint(x: lx + cType, y: yi)) }
    if !label.isEmpty  { a(label,  color, bold: false).draw(at: CGPoint(x: lx + cLbl,  y: yi)) }
    if !cmd.isEmpty    { a(cmd,    color, bold: false).draw(at: CGPoint(x: lx + cCmd,  y: yi)) }
    a(shifts, color, bold: bold).draw(at: CGPoint(x: lx + cShi, y: yi))
    a(status, sc,    bold: bold).draw(at: CGPoint(x: lx + cStat, y: yi))
    // Right border: placed at same X as the '┐' in boxTop — (INNER+2) chars from lx,
    // minus 1 char width because '┐' itself is 1 char wide and we want to overlap it exactly.
    let rx = lx + CGFloat(INNER + 1) * charW
    a("│", BLUE, bold: true).draw(at: CGPoint(x: rx, y: yi))
}


// Grid header
drawRowAt(idx: lineIdx, gear: "Gear", atype: "Action Type", label: "Label",
          cmd: "Command / Flag", shifts: "Shifts", status: "Status",
          color: YELLOW, bold: true)
lineIdx += 1

// Separator row
let sepLx = SHADOW + PAD_X
a("│", BLUE, bold: true).draw(at: CGPoint(x: sepLx, y: Y(lineIdx)))
a("│", BLUE, bold: true).draw(at: CGPoint(x: sepLx + CGFloat(INNER + 1) * charW, y: Y(lineIdx)))
lineIdx += 1


struct GRow { let gear, atype, label, cmd, shifts, status, style: String }

let gridRows: [GRow] = [
    GRow(gear:"N", atype:"",                   label:"",                          cmd:"",                              shifts:"0", status:"idle",        style:"neutral"),
    GRow(gear:"1", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (Low)",    cmd:"/model gemini-3.7-flash-low",   shifts:"6", status:"idle",        style:"normal"),
    GRow(gear:"2", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (Medium)", cmd:"/model gemini-3.7-flash-medium",shifts:"5", status:"idle",        style:"normal"),
    GRow(gear:"3", atype:"🛸 Antigravity CLI", label:"Gemini 3.7 Flash (High)",   cmd:"/model gemini-3.7-flash-high",  shifts:"4", status:"idle",        style:"normal"),
    GRow(gear:"4", atype:"🛸 Antigravity CLI", label:"Gemini 3.1 Pro (High)",     cmd:"/model gemini-3.1-pro-high",    shifts:"1", status:"idle",        style:"normal"),
    GRow(gear:"5", atype:"🛸 Antigravity CLI", label:"Claude Sonnet 4.6 (Think)", cmd:"/model claude-sonnet-4-6",      shifts:"4", status:"idle",        style:"normal"),
    GRow(gear:"6", atype:"🛸 Antigravity CLI", label:"Claude Opus 4.6 (Think)",   cmd:"/model claude-opus-4-6-think",  shifts:"1", status:"idle",        style:"normal"),
    GRow(gear:"R", atype:"📑 New Session(Tab)", label:"New AGY Session (Tab)",   cmd:"agy",                           shifts:"9", status:"🟢 ENGAGED",  style:"engaged"),
]

for row in gridRows {
    let isN = row.gear == "N"
    let isR = row.style == "engaged"
    let color: NSColor = isN ? PURPLE : (isR ? RED : NORM)
    let sc: NSColor    = isR ? GREEN : color
    let dashes = row.atype.isEmpty
    drawRowAt(idx: lineIdx,
              gear: row.gear,
              atype:  dashes ? "—" : row.atype,
              label:  dashes ? "—" : row.label,
              cmd:    dashes ? "—" : row.cmd,
              shifts: row.shifts,
              status: row.status,
              color: color, statColor: sc,
              bold: isN || isR, cursor: isN)
    lineIdx += 1
}

line(boxBot(), BLUE, bold: true)
lineIdx += 1  // gap

// ── Status ──
line(boxTop("Status & Controls"), BLUE, bold: true)
line(boxRow("Last Action: New AGY Session (Tab)  |  Total Shifts: 30  |  Device: Connected (Arduino Leonardo)"), NORM)
line(boxRow("Controls: [j/k]: Nav  |  [h/l/p]: Tab  |  [Space]: Active  |  [e/Enter]: Edit  |  [t]: Theme  |  [1-6/r]: Shift  |  [q]: Exit"), CYAN)
line(boxBot(), BLUE, bold: true)

image.unlockFocus()

guard let tiff = image.tiffRepresentation,
      let rep = NSBitmapImageRep(data: tiff),
      let png = rep.representation(using: .png, properties: [:]) else {
    fputs("ERROR: Could not encode PNG\n", stderr)
    exit(1)
}

try! png.write(to: URL(fileURLWithPath: "assets/realshifter-dashboard.png"))
print("✓ PNG saved: assets/realshifter-dashboard.png  (\(Int(W))×\(Int(H)) pt)")
