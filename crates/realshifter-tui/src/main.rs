mod app;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use realshifter_core::GearPosition;
use std::io;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("RealShifter TUI error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        app.refresh();
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            #[allow(clippy::single_match)]
            if let Ok(Event::Key(key)) = event::read() {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    app.should_quit = true;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Char('1') => app.shift_gear(GearPosition::Gear1),
                    KeyCode::Char('2') => app.shift_gear(GearPosition::Gear2),
                    KeyCode::Char('3') => app.shift_gear(GearPosition::Gear3),
                    KeyCode::Char('4') => app.shift_gear(GearPosition::Gear4),
                    KeyCode::Char('5') => app.shift_gear(GearPosition::Gear5),
                    KeyCode::Char('6') => app.shift_gear(GearPosition::Gear6),
                    KeyCode::Char('r') | KeyCode::Char('R') => app.shift_gear(GearPosition::Reverse),
                    KeyCode::Char('n') | KeyCode::Char('N') => app.shift_gear(GearPosition::Neutral),
                    KeyCode::Char('p') | KeyCode::Char('P') => app.cycle_profile(),
                    _ => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
