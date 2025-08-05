use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{ Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap,
        block::Title,
    },
};


use crate::utils::fuzzy_matcher::{search_questions, search_topics};
use crate::utils::parse_html::parse_html_to_lines;
use crate::{
    db::zuko_cli::get_all_questions,
    types::{AppState, CurrentScreen, DifficultyFilter},
    utils::ui::centered_rect,
};

use crate::config::ui::{
    BACKGROUND_COLOR, BLOCK_PADDING, BORDER_COLOR, HIGHLIGHT_COLOR, HIGHLIGHT_SYMBOL,
    POPUP_BACKGROUND_COLOR, POPUP_BORDER_COLOR, TEXT_COLOR, TITLE_TEXT_COLOR,  DARK_TEXT_COLOR, LIGHT_ORANGE
};

pub async fn run_list_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    // question list state
    // question list state
    update_question_list(app);
    let mut question_list_state: ListState = ListState::default();
    question_list_state.select(Some(app.selected_index));

    // topic list state
    update_topic_list(app);
    let mut topic_list_state: ListState = ListState::default();
    topic_list_state.select(Some(app.selected_topic_index));

    // difficulty list state
    update_difficulty_list(app);
    let mut difficulty_list_state: ListState = ListState::default();
    difficulty_list_state.select(Some(app.selected_difficulty_index));

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
                .constraints([Constraint::Min(3), Constraint::Length(3)])
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
                        // title should be dynamic based on the topic and difficulty
                        // update title and difficulty
                        .title(format!(
                            " Questions ({} - {})",
                            app.selected_topic
                                .as_ref()
                                .map_or("All Topics", |t| &t.name),
                            app.selected_difficulty
                        ))
                        .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                        .borders(Borders::ALL)
                        .padding(Padding::new(0, 0, 1, 0))
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .border_type(BorderType::Rounded),
                )
                .highlight_symbol(HIGHLIGHT_SYMBOL)
                .highlight_style(
                    Style::default()
                        .fg(HIGHLIGHT_COLOR)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_stateful_widget(list, question_list_ui, &mut question_list_state);

            // Search input box
            let search_input = Paragraph::new(app.query.clone())
                .block(
                    Block::default()
                        .title(" Search ")
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

            //footer note shows all the keybindings and the current screen based on the app state and current screen

            let footer = render_footer(app);
            frame.render_widget(footer, footer_note_ui);

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

                frame.render_stateful_widget(topic_list, popup_chunks[0], &mut topic_list_state);

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

            if let CurrentScreen::DifficultyFilter = app.current_screen {
                // Render difficulty filter UI
                let difficulty_filter_block = Block::default().style(Style::default().bg(POPUP_BACKGROUND_COLOR));

                let difficulty_popup_area = centered_rect(20, 20, frame.area());

                frame.render_widget(Clear, difficulty_popup_area);
                frame.render_widget(difficulty_filter_block, difficulty_popup_area);

                let difficulty_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(3)])
                    .split(difficulty_popup_area);

                // Render difficulty options
                let difficulty_items: Vec<ListItem> = app
                    .difficulties
                    .iter()
                    .map(|d| {
                        ListItem::new(Line::from(Span::styled(
                            d.to_str().to_string(),
                            Style::default(),
                        )))
                    })
                    .collect();

                let difficulty_list = List::new(difficulty_items)
                    .block(
                        Block::default()
                            .title(" Select Difficulty ")
                            .title_style(Style::default().fg(TITLE_TEXT_COLOR))
                            .padding(Padding::new(1, 1, 1, 1))
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
                    difficulty_list,
                    difficulty_chunks[0],
                    &mut difficulty_list_state,
                );
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
                            app.selected_index = 0;
                            update_question_list(app);
                            question_list_state.select(Some(app.selected_index));
                            app.scroll = 0;
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            app.query.pop();
                            app.selected_index = 0;
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
                        // ctrl + d to toggle difficulty filter popup
                        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                            app.current_screen = CurrentScreen::DifficultyFilter;
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
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            
                            if let Some(selected_topic) = app
                                .filtered_topic_indices
                                .get(app.selected_topic_index)
                                .and_then(|&idx| app.all_topics.get(idx).cloned())
                            {
                                app.selected_topic = Some(selected_topic);
                                filter_questions_by_topic_and_difficulty(app).await;
                                app.topic_query.clear();
                                update_topic_list(app);
                                update_question_list(app);
                            }
                            app.selected_index = 0;
                            question_list_state.select(Some(app.selected_index));
                            app.current_screen = CurrentScreen::QuestionList;
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
                    match (key.code, key.modifiers) {
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if app.selected_difficulty_index > 0 {
                                app.selected_difficulty_index -= 1;

                                difficulty_list_state.select(Some(app.selected_difficulty_index));
                            }
                        }
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if app.selected_difficulty_index + 1 < app.difficulties.len() {
                                app.selected_difficulty_index += 1;
                                difficulty_list_state.select(Some(app.selected_difficulty_index));
                            }
                        }
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            app.selected_index = 0;
                            question_list_state.select(Some(app.selected_index));
                            update_difficulty_list(app);
                            filter_questions_by_topic_and_difficulty(app).await;
                            app.current_screen = CurrentScreen::QuestionList;
                        }
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            app.current_screen = CurrentScreen::QuestionList;
                        }
                        _ => {}
                    }
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

    // Fetch questions based on the selected topic and difficulty

    app.all_questions = match get_all_questions(
        Some(selected_topic_slug),
        Some(app.selected_difficulty.clone().to_str().to_uppercase()),
    )
    .await
    {
        Result::Ok(qs) => qs,
        Result::Err(e) => {
            eprintln!("Failed to get questions from the database: {}", e);
            return;
        }
    };
}

pub fn update_difficulty_list(app: &mut AppState) {
    // Update the difficulty list based on the current state
    app.difficulties = DifficultyFilter::all_difficulties();
    if app.selected_difficulty_index >= app.difficulties.len() {
        app.selected_difficulty_index = 0;
    } else {
        app.selected_difficulty = app.difficulties[app.selected_difficulty_index].clone();
    }
}

fn render_footer(app: &AppState) -> Paragraph<'static> {
    let current_navigation_text: Vec<Span> = vec![
        // First segment describing current screen
        match app.current_screen {
            CurrentScreen::QuestionList => {
                Span::styled(" Question Browser ", Style::default().bg(LIGHT_ORANGE).fg(DARK_TEXT_COLOR))
            }
            CurrentScreen::TopicList => {
                Span::styled(" Topic Filter ", Style::default().bg(LIGHT_ORANGE).fg(DARK_TEXT_COLOR))
            }
            CurrentScreen::DifficultyFilter => {
                Span::styled(" Difficulty Filter ", Style::default().bg(LIGHT_ORANGE).fg(DARK_TEXT_COLOR))
            }
        },
        // Divider
        Span::styled(" | ", Style::default().fg(TITLE_TEXT_COLOR)),
        // Additional context or instructions
        match app.current_screen {
            CurrentScreen::QuestionList => Span::styled(
                "Type to search | Backspace: Delete | ↑/↓: Navigate",
                Style::default().fg(HIGHLIGHT_COLOR),
            ),
            CurrentScreen::TopicList => Span::styled(
                "Type to search | Backspace: Delete | ↑/↓: Navigate ",
                Style::default().fg(HIGHLIGHT_COLOR),
            ),
            CurrentScreen::DifficultyFilter => Span::styled(
                "↑/↓: Navigate",
                Style::default().fg(HIGHLIGHT_COLOR),
            ),
        },

        Span::styled(" | ", Style::default().fg(TITLE_TEXT_COLOR)),
    ];

    // Key hints (right side of footer or below navigation)
    let current_keys_hint: Span = match app.current_screen {
        CurrentScreen::QuestionList => Span::styled(
            "Ctrl + T to filter by topic / Ctrl + D to filter by difficulty / Ctrl + C: Quit",
            Style::default().fg(LIGHT_ORANGE),
        ),
        CurrentScreen::TopicList => Span::styled(
            "Enter to select the topic / Esc to cancel",
            Style::default().fg(LIGHT_ORANGE),
        ),
        CurrentScreen::DifficultyFilter => Span::styled(
            "Enter to apply difficulty level / Esc to cancel",
            Style::default().fg(LIGHT_ORANGE),
        ),
    };

    // Final footer paragraph
    let footer_paragraph = Paragraph::new(Line::from(
        current_navigation_text
            .into_iter()
            .chain(vec![Span::raw(""), current_keys_hint])
            .collect::<Vec<Span>>(),
    ))
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(BACKGROUND_COLOR))
            .padding(Padding::new(1, 1, 1, 0))
    )
    .style(Style::default().bg(POPUP_BACKGROUND_COLOR));

    footer_paragraph
}
