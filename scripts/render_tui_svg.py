#!/usr/bin/env python3
import sys
import html
import subprocess

def main():
    dashboard_lines = [
        " ┌ RealShifter Dashboard ───────────────────────────────────────────────────────────────────────────────────────────────────┐",
        " │ RealShifter v0.1.0  [Theme: Dark]  |  🟢 ACTIVE: Antigravity (AGY)  | Current Gear:  [R]                                  │",
        " └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        " ┌ Profile View [Press 'h/l' or 'p' to switch | 'Space' to set Active] ─────────────────────────────────────────────────────┐",
        " │ 🟢 🛸 Antigravity (AGY) │ 🧠 Claude Code │ 💻 Codex CLI │ ⚡ OpenCode CLI │ π Pi Agent │ 🎛️ Custom / Multi-Tool              │",
        " └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        " ┌ Gear Mapping Status Grid [Use 'j/k' to select, 'Enter/e' to edit] ───────────────────────────────────────────────────────┐",
        " │   Gear     Action Type        Label                    Command / Flag                        Shifts   Status             │",
        " │                                                                                                                          │",
        " │▶   N       —                  —                        —                                       0      idle               │",
        " │    1       🛸 Antigravity CLI Gemini 3.7 Flash (Low)   /model gemini-3.7-flash-low             6      idle               │",
        " │    2       🛸 Antigravity CLI Gemini 3.7 Flash (Medium)/model gemini-3.7-flash-medium          5      idle               │",
        " │    3       🛸 Antigravity CLI Gemini 3.7 Flash (High)  /model gemini-3.7-flash-high            4      idle               │",
        " │    4       🛸 Antigravity CLI Gemini 3.1 Pro (High)    /model gemini-3.1-pro-high              1      idle               │",
        " │    5       🛸 Antigravity CLI Claude Sonnet 4.6 (Think)/model claude-sonnet-4-6                4      idle               │",
        " │    6       🛸 Antigravity CLI Claude Opus 4.6 (Thinkin)/model claude-opus-4-6-thinking         1      idle               │",
        " │    R       📑 New Session(Tab)New AGY Session (Tab)    agy                                     9      🟢 ENGAGED         │",
        " └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘",
        " ┌ Status & Controls ───────────────────────────────────────────────────────────────────────────────────────────────────────┐",
        " │ Last Action: New AGY Session (Tab) | Total Shifts: 30 | Device: Connected (Arduino Leonardo)                             │",
        " │ Controls: [j/k]: Nav | [h/l/p]: Tab | [Space]: Active | [e/Enter]: Edit | [t]: Theme | [1-6/r]: Shift | [q]: Exit        │",
        " └──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘"
    ]

    char_width = 8.5
    line_height = 20
    padding_x = 24
    padding_top = 54
    padding_bottom = 24

    max_cols = max(len(l) for l in dashboard_lines)
    width = int(max_cols * char_width + padding_x * 2)
    height = int(len(dashboard_lines) * line_height + padding_top + padding_bottom)

    svg = []
    svg.append(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="100%" height="100%">')
    svg.append('<defs>')
    svg.append('  <filter id="shadow" x="-5%" y="-5%" width="110%" height="110%">')
    svg.append('    <feDropShadow dx="0" dy="12" stdDeviation="16" flood-color="#000000" flood-opacity="0.5"/>')
    svg.append('  </filter>')
    svg.append('  <linearGradient id="headerGrad" x1="0%" y1="0%" x2="0%" y2="100%">')
    svg.append('    <stop offset="0%" stop-color="#21252b"/>')
    svg.append('    <stop offset="100%" stop-color="#181a1f"/>')
    svg.append('  </linearGradient>')
    svg.append('</defs>')
    
    # Background Window
    svg.append(f'  <rect x="8" y="8" width="{width-16}" height="{height-16}" rx="12" ry="12" fill="#14161b" stroke="#282c34" stroke-width="1.5" filter="url(#shadow)"/>')
    
    # Title Bar
    svg.append(f'  <path d="M 8 20 Q 8 8 20 8 L {width-20} 8 Q {width-8} 8 {width-8} 20 L {width-8} 42 L 8 42 Z" fill="url(#headerGrad)"/>')
    svg.append(f'  <line x1="8" y1="42" x2="{width-8}" y2="42" stroke="#282c34" stroke-width="1"/>')
    
    # Window Buttons
    svg.append('  <circle cx="28" cy="25" r="6" fill="#ff5f56" stroke="#e0443e" stroke-width="0.5"/>')
    svg.append('  <circle cx="48" cy="25" r="6" fill="#ffbd2e" stroke="#dea123" stroke-width="0.5"/>')
    svg.append('  <circle cx="68" cy="25" r="6" fill="#27c93f" stroke="#1aab29" stroke-width="0.5"/>')
    
    # Title
    svg.append(f'  <text x="{width/2}" y="29" font-family="-apple-system, BlinkMacSystemFont, \'Segoe UI\', Roboto, sans-serif" font-size="12" font-weight="600" fill="#9da5b4" text-anchor="middle">RealShifter TUI Dashboard (Herdr Overlay)</text>')
    
    # Body
    svg.append('  <g font-family="JetBrains Mono, Menlo, Monaco, Consolas, monospace" font-size="13" font-weight="500" xml:space="preserve">')
    
    y = padding_top + 14
    for line in dashboard_lines:
        escaped = html.escape(line)
        
        color = "#abb2bf"
        if "ENGAGED" in line:
            color = "#98c379"
        elif "ACTIVE:" in line:
            color = "#98c379"
        elif "RealShifter Dashboard" in line or "Profile View" in line or "Gear Mapping Status Grid" in line or "Status & Controls" in line:
            color = "#61afef"
        elif "Gear" in line and "Action Type" in line:
            color = "#e5c07b"
        elif "Controls:" in line:
            color = "#56b6c2"
        elif line.strip().startswith("│▶"):
            color = "#c678dd"
        elif line.strip().startswith("│    R"):
            color = "#e06c75"
            
        svg.append(f'    <text x="{padding_x}" y="{y}" fill="{color}">{escaped}</text>')
        y += line_height

    svg.append('  </g>')
    svg.append('</svg>')

    out_file = "assets/realshifter-dashboard.svg"
    with open(out_file, "w", encoding="utf-8") as f:
        f.write("\n".join(svg))
    print(f"Clean TUI Dashboard SVG saved to {out_file}")

if __name__ == "__main__":
    main()
