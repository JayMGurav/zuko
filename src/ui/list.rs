use ratatui::{
    Terminal,
    backend::{Backend},
    crossterm::{
        event::{
            self, Event, KeyCode,
            KeyModifiers,
        },  
    },
    layout::{Constraint, Direction, Layout, Alignment},
    style::{ Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders,BorderType, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap,block::Title
    },
};

use crate::utils::fuzzy_matcher::{search_questions, search_topics};
use crate::utils::parse_html::parse_html_to_lines;
use crate::{
    db::zuko_cli::get_all_questions,
    types::{AppState, CurrentScreen},
    utils::ui::centered_rect,
};

use crate::config::ui::{
    BACKGROUND_COLOR, POPUP_BACKGROUND_COLOR, BORDER_COLOR, HIGHLIGHT_COLOR, TEXT_COLOR,POPUP_BORDER_COLOR, TITLE_TEXT_COLOR, BLOCK_PADDING, HIGHLIGHT_SYMBOL
};

pub fn run_list_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {

    // question list state
    update_question_list(app);
    let mut question_list_state: ListState = ListState::default();
    question_list_state.select(Some(app.selected_index));

    // topic list state
    update_topic_list(app);
    let mut topic_list_state: ListState = ListState::default();
    topic_list_state.select(Some(app.selected_topic_index));

    loop {
        // --------------------------- draw ui ---------------------------
        terminal.draw(|frame| {
            let zuko_area = Block::default()
                .title(Title::from(" Zuko List ").alignment(Alignment::Center))
                .title_style(Style::default().fg(HIGHLIGHT_COLOR))
                .padding(BLOCK_PADDING)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER_COLOR))
                .style(Style::default().bg(BACKGROUND_COLOR));
            
            frame.render_widget(zuko_area, frame.area());
            

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Min(3), Constraint::Length(2)])
                .split(frame.area());


            // Split the top chunk (chunks[0]) into two horizontally
            let question_list_ui_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(chunks[0]);

            // Split the first horizontal chunk vertically
            let question_list_chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(question_list_ui_chunk[0]);

            let footer_note_ui = chunks[1];
            let search_input_ui = question_list_chunk[1];
            let question_list_ui = question_list_chunk[0];
            let question_preview_ui = question_list_ui_chunk[1];

            // Question list
            let items: Vec<ListItem> = app
                .filtered_question_indices
                .iter()
                .filter_map(|&idx| app.all_questions.get(idx))
                .map(|q| ListItem::new(Line::from(Span::styled(q.title.clone(), Style::default()))))
                .collect();

            let list = List::new(items)
                .block(
                        Block::default()
                            .title(" Questions ")
                            .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(BORDER_COLOR))
                            .border_type(BorderType::Rounded)
                    )
                .highlight_symbol(HIGHLIGHT_SYMBOL)
                .highlight_style(
                    Style::default()
                        .fg(HIGHLIGHT_COLOR)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_stateful_widget(
                list,
                question_list_ui,
                &mut question_list_state,
            );

            // Search input box
            let search_input = Paragraph::new(app.query.clone())
                .block(
                    Block::default().title(" Search ")
                        .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(TEXT_COLOR));

            frame.render_widget(search_input, search_input_ui);

            // question preview
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
                        .title(" Question Preview ")
                        .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .border_type(BorderType::Rounded)
                        .padding(Padding::uniform(1)), // Add vertical padding
                )
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, question_preview_ui);

            // --------------------------- footer note ---------------------------

            //TODO

            // --------------------------- Filter topics popup -----------------
            if let CurrentScreen::TopicList = app.current_screen {
                let popup_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(POPUP_BORDER_COLOR))
                    .style(Style::default().bg(POPUP_BACKGROUND_COLOR));

                let popup_area = centered_rect(25, 40, frame.area());

                frame.render_widget(Clear, popup_area);
                frame.render_widget(popup_block, popup_area);

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(3), Constraint::Length(3)])
                    .split(popup_area);

                // Topic list
                let topic_items: Vec<ListItem> = app
                    .filtered_topic_indices
                    .iter()
                    .filter_map(|&idx| app.all_topics.get(idx))
                    .map(|t| {
                        ListItem::new(Line::from(Span::styled(t.name.clone(), Style::default())))
                    })
                    .collect();

                let topic_list = List::new(topic_items)
                    .block(
                        Block::default()
                            .title(" Select Topic ")
                            .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(BORDER_COLOR))
                            .border_type(BorderType::Rounded),
                    )
                    .highlight_symbol(HIGHLIGHT_SYMBOL)
                    .highlight_style(
                        Style::default()
                            .fg(HIGHLIGHT_COLOR)
                            .add_modifier(Modifier::BOLD),
                    );

                frame.render_stateful_widget(
                    topic_list,
                    popup_chunks[0],
                    &mut topic_list_state,
                );

                // Search input for topic filter
                let topic_search_input = Paragraph::new(app.topic_query.clone())
                    .block(
                        Block::default()
                            .title(" Search Topic ")
                            .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(BORDER_COLOR))
                            .border_type(BorderType::Rounded),
                    )
                    .wrap(Wrap { trim: true })
                    .style(Style::default().fg(TEXT_COLOR));

                frame.render_widget(topic_search_input, popup_chunks[1]);
            }
        })?;

        // --------------------------- event management ---------------------

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::QuestionList => {
                    // Handle question list events
                    match (key.code, key.modifiers) {
                        (KeyCode::Char(c), KeyModifiers::NONE) => {
                            app.query.push(c);
                            update_question_list(app);
                            question_list_state.select(Some(app.selected_index));
                            app.scroll = 0;
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            app.query.pop();
                            update_question_list(app);
                            question_list_state.select(Some(app.selected_index));
                            app.scroll = 0;
                        }
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                                question_list_state.select(Some(app.selected_index));
                                app.scroll = 0;
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if app.selected_index + 1 < app.filtered_question_indices.len() {
                                app.selected_index += 1;
                                question_list_state.select(Some(app.selected_index));
                                app.scroll = 0;
                            }
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            break ();
                        }
                        // ctrl + t to toggle topic filter popup
                        (KeyCode::Char('t'), KeyModifiers::CONTROL) => {
                            app.current_screen = CurrentScreen::TopicList;
                        }
                        // implement scroll functionality
                        _ => {}
                    }
                }
                CurrentScreen::TopicList => {
                    // Handle topic list events
                    match (key.code, key.modifiers) {
                        (KeyCode::Char(c), KeyModifiers::NONE) => {
                            app.topic_query.push(c);
                            update_topic_list(app);
                            topic_list_state.select(Some(app.selected_topic_index));
                            app.scroll = 0;
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            app.topic_query.pop();
                            update_topic_list(app);
                            topic_list_state.select(Some(app.selected_topic_index));
                            app.scroll = 0;
                        }
                         (KeyCode::Up, KeyModifiers::NONE) => {
                            if app.selected_topic_index > 0 {
                                app.selected_topic_index -= 1;
                                topic_list_state.select(Some(app.selected_topic_index));
                                app.scroll = 0;
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if app.selected_topic_index + 1 < app.filtered_topic_indices.len() {
                                app.selected_topic_index += 1;
                                topic_list_state.select(Some(app.selected_topic_index));
                                app.scroll = 0;
                            }
                        }
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            app.topic_query.clear();
                            update_topic_list(app);
                            app.current_screen = CurrentScreen::QuestionList;
                        }
                        _ => {}
                    }
                }
                CurrentScreen::DifficultyFilter => {
                    // Handle difficulty filter events
                }
            }
        }
    }

    Ok(())
}

pub fn update_question_list(app: &mut AppState) {
    app.filtered_question_indices = search_questions(&app.all_questions, &app.query);
    if app.selected_index >= app.filtered_question_indices.len() {
        app.selected_index = 0;
    }
}

pub fn update_topic_list(app: &mut AppState) {
    // get all topics for the selected topic
    // update the app context with the filtered topics
    app.filtered_topic_indices = search_topics(&app.all_topics, &app.topic_query);
    if app.selected_topic_index >= app.filtered_topic_indices.len() {
        app.selected_topic_index = 0;
    }
}

pub async fn filter_questions_by_topic_and_difficulty(app: &mut AppState) {
    let selected_topic_slug = app
        .selected_topic
        .as_ref()
        .map(|t| t.slug.clone())
        .unwrap_or_default();

    // let selected_difficulty = app.selected_difficulty.clone();

    app.all_questions = match get_all_questions(Some(selected_topic_slug), None).await {
        Result::Ok(qs) => qs,
        Result::Err(e) => {
            eprintln!("Failed to get questions from the database: {}", e);
            return;
        }
    };
}
