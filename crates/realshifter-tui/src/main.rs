mod app;
mod ui;

use app::{App, EditField};
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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        app.refresh();
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Ok(Event::Key(key)) = event::read() {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    app.should_quit = true;
                } else if app.show_models_modal {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('m') | KeyCode::Char('M') => {
                            app.show_models_modal = false;
                        }
                        KeyCode::Char('q') => app.should_quit = true,
                        _ => {}
                    }
                } else if app.show_help_modal {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('?') => {
                            app.show_help_modal = false;
                        }
                        KeyCode::Char('q') => app.should_quit = true,
                        _ => {}
                    }
                } else if app.edit_state.is_some() {
                    match key.code {
                        KeyCode::Esc => app.cancel_editing(),
                        KeyCode::Enter => {
                            if let Some(ref es) = app.edit_state {
                                if es.focused_field == EditField::Cancel {
                                    app.cancel_editing();
                                } else {
                                    app.save_editing();
                                }
                            }
                        }
                        KeyCode::Tab | KeyCode::Down => {
                            if let Some(ref mut es) = app.edit_state {
                                es.focused_field = es.focused_field.next();
                            }
                        }
                        KeyCode::BackTab | KeyCode::Up => {
                            if let Some(ref mut es) = app.edit_state {
                                es.focused_field = es.focused_field.prev();
                            }
                        }
                        KeyCode::Left | KeyCode::Right => {
                            let current_field = app.edit_state.as_ref().map(|es| es.focused_field.clone());
                            match current_field {
                                Some(EditField::ActionType) => app.cycle_edit_action_type(),
                                Some(EditField::Model) => app.cycle_edit_model(),
                                Some(EditField::Effort) => app.cycle_edit_effort(),
                                Some(EditField::Save) => app.save_editing(),
                                Some(EditField::Cancel) => app.cancel_editing(),
                                _ => {}
                            }
                        }
                        KeyCode::Backspace => app.handle_edit_backspace(),
                        KeyCode::Char(c) => {
                            let current_field = app.edit_state.as_ref().map(|es| es.focused_field.clone());
                            match current_field {
                                Some(EditField::CustomCommand) | Some(EditField::Label) => {
                                    app.handle_edit_char(c);
                                }
                                Some(EditField::ActionType) => app.cycle_edit_action_type(),
                                Some(EditField::Model) => app.cycle_edit_model(),
                                Some(EditField::Effort) => app.cycle_edit_effort(),
                                Some(EditField::Save) if c == ' ' => app.save_editing(),
                                Some(EditField::Cancel) if c == ' ' => app.cancel_editing(),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                } else {
                    // Dashboard Mode (Normal)
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Down | KeyCode::Char('j') => app.select_next_gear(),
                        KeyCode::Up | KeyCode::Char('k') => app.select_prev_gear(),
                        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('p') => app.cycle_view_profile(),
                        KeyCode::Left | KeyCode::Char('h') => app.prev_view_profile(),
                        KeyCode::Char(' ') | KeyCode::Char('a') | KeyCode::Char('A') => app.set_view_as_active_profile(),
                        KeyCode::Enter | KeyCode::Char('e') | KeyCode::Char('E') => app.start_editing_selected_gear(),
                        KeyCode::Char('1') => app.shift_gear(GearPosition::Gear1),
                        KeyCode::Char('2') => app.shift_gear(GearPosition::Gear2),
                        KeyCode::Char('3') => app.shift_gear(GearPosition::Gear3),
                        KeyCode::Char('4') => app.shift_gear(GearPosition::Gear4),
                        KeyCode::Char('5') => app.shift_gear(GearPosition::Gear5),
                        KeyCode::Char('6') => app.shift_gear(GearPosition::Gear6),
                        KeyCode::Char('r') | KeyCode::Char('R') => app.shift_gear(GearPosition::Reverse),
                        KeyCode::Char('n') | KeyCode::Char('N') => app.shift_gear(GearPosition::Neutral),
                        KeyCode::Char('m') | KeyCode::Char('M') => app.toggle_models_modal(),
                        KeyCode::Char('?') => app.toggle_help_modal(),
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
