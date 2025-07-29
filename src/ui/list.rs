use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap, Padding},
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

use crate::ui::{update_filtered, AppState};

use crate::utils::parse_html::parse_html_to_lines;


pub fn run_ui(mut app: AppState) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    update_filtered(&mut app);
    let mut list_state: ListState = ListState::default();
    list_state.select(Some(app.selected_index));

    let result = loop {
        terminal.draw(|f| {

            let zuko_screen = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(f.area());


            let questions_pannel = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),       // Question list
                    Constraint::Length(3),    // Search input box
                ])
                .split(zuko_screen[0]);

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

            // f.render_stateful_widget(list, chunks[0], &mut list_state);
            f.render_stateful_widget(list, questions_pannel[0], &mut list_state);


            // -- Search input (bottom left)
            let input = Paragraph::new(Text::from(app.query.clone()))
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::ALL).title("Search"));

            f.render_widget(input, questions_pannel[1]);


            // Right: Markdown preview
            let question_content = app
                .filtered_question_indices
                .get(app.selected_index)
                .and_then(|&idx| app.all_questions.get(idx))
                .map(|q| q.content.clone())
                .unwrap_or_default();


            let lines = parse_html_to_lines(&question_content);
            let paragraph = Paragraph::new(lines.to_vec())
                .block(
                    Block::default()
                        .title("Question Preview")
                        .borders(Borders::ALL)
                        // Add padding inside the block
                        .padding(Padding::uniform(1)),
                )
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, zuko_screen[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
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
                    KeyCode::Esc => {
                        break Ok(());
                    }
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