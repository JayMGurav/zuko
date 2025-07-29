use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use crossterm::{
    event::{self, Event as CEvent, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{stdout},
    time::Duration,
};

use tui_markdown::{from_str};

use crate::ui::{AppState,update_filtered};




pub fn run_ui(mut app: AppState) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    update_filtered(&mut app);
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    let result = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(f.area());

            // Left: Search list
            let items: Vec<ListItem> = app
                .filtered_question_indices
                .iter()
                .filter_map(|&idx| app.all_questions.get(idx))
                .map(|q| {
                    ListItem::new(Line::from(Span::styled(
                        q.title.clone(),
                        Style::default(),
                    )))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Questions").borders(Borders::ALL))
                .highlight_symbol(">> ")
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            f.render_stateful_widget(list, chunks[0], &mut list_state);

            // Right: Markdown preview
            let preview = app
                .filtered_question_indices
                .get(app.selected_index)
                .and_then(|&idx| app.all_questions.get(idx))
                .map(|q| q.content.clone())
                .unwrap_or_default();

            let content_text = from_str(&preview);
            f.render_widget(content_text, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        if c == 'q'{
                            break Ok(());
                        }
                        app.query.push(c);
                        update_filtered(&mut app);
                        list_state.select(Some(app.selected_index));
                        app.scroll = 0;
                    }
                    KeyCode::Backspace => {
                        app.query.pop();{}
                        update_filtered(&mut app);
                        list_state.select(Some(app.selected_index));
                        app.scroll = 0;
                    }
                    KeyCode::Up => {
                        if app.selected_index > 0 {
                            app.selected_index -= 1;
                            list_state.select(Some(app.selected_index));
                            app.scroll = 0;
                        }
                    }
                    KeyCode::Down => {
                        if app.selected_index + 1 < app.filtered_question_indices.len() {
                            app.selected_index += 1;
                            list_state.select(Some(app.selected_index));
                            app.scroll = 0;
                        }
                    }
                    KeyCode::PageDown => {
                        app.scroll += 5;
                    }
                    KeyCode::PageUp => {
                        app.scroll = app.scroll.saturating_sub(5);
                    }
                    // KeyCode::Char('q') => break Ok(()),
                    _ => {}
                }
            }
        }
    };

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}