// ABOUTME: Entry point for UI template sandbox
// ABOUTME: Event loop with keyboard toggles for experimenting with UI

mod config;
mod mock;
mod theme;
mod tui;
mod ui;

use anyhow::Result;
use config::{UiConfig, UiElement};
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind};
use std::time::Duration;
use ui::SandboxState;

fn main() -> Result<()> {
    // Initialize config and state
    let mut config = UiConfig::default();
    let mock_data = mock::create_mock_data();
    let mut state = SandboxState::new(mock_data);

    // Initialize terminal
    let mut terminal = tui::init()?;

    // Main event loop
    let result = run(&mut terminal, &mut state, &mut config);

    // Restore terminal
    tui::restore()?;

    result
}

fn run(terminal: &mut tui::Tui, state: &mut SandboxState, config: &mut UiConfig) -> Result<()> {
    loop {
        // Render
        terminal.draw(|frame| ui::render(frame, state, config))?;

        // Handle all pending events (drain queue to prevent mouse event flooding)
        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key) => {
                    // Close help overlay on any key if it's open
                    if config.show_help && key.code != KeyCode::Char('?') {
                        config.show_help = false;
                        continue;
                    }

                    match key.code {
                        // Quit
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            return Ok(());
                        }

                        // UI element toggles
                        KeyCode::Char('1') => config.toggle(UiElement::SearchBar),
                        KeyCode::Char('2') => config.toggle(UiElement::StatusBar),
                        KeyCode::Char('3') => config.toggle(UiElement::ResultsPane),
                        KeyCode::Char('4') => config.toggle(UiElement::PreviewPane),
                        KeyCode::Char('5') => config.toggle(UiElement::ScopeWidget),

                        // Theme cycling
                        KeyCode::Char('t') => config.cycle_theme(),

                        // Pane split adjustment
                        KeyCode::Char('[') => config.adjust_split(-5),
                        KeyCode::Char(']') => config.adjust_split(5),

                        // Help
                        KeyCode::Char('?') => config.toggle_help(),

                        // Reset
                        KeyCode::Char('r') => config.reset(),

                        // Navigation
                        KeyCode::Up => state.on_up(),
                        KeyCode::Down => state.on_down(),
                        KeyCode::PageUp => state.scroll_preview_up(15),
                        KeyCode::PageDown => state.scroll_preview_down(15),

                        // Search input (for demo)
                        KeyCode::Backspace => state.on_backspace(),
                        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Only add to search if it's a regular character (not a control key)
                            if c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_' {
                                state.on_char(c);
                            }
                        }

                        _ => {}
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => state.scroll_preview_up(3),
                    MouseEventKind::ScrollDown => state.scroll_preview_down(3),
                    _ => {}
                },
                _ => {}
            }
        }

        // Small sleep to prevent busy loop
        std::thread::sleep(Duration::from_millis(16));
    }
}
