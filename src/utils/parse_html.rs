use html_parser::{Dom, Node};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn parse_html_to_lines(html: &str) -> Vec<Line<'static>> {
    let dom = Dom::parse(html)
        .unwrap_or_else(|e| Dom::parse(&format!("Error parsing HTML: {}", e)).unwrap());
    let mut lines: Vec<Line<'static>> = Vec::new();

    for node in dom.children {
        // We only care about element nodes at the top level for block spacing
        if let Node::Element(element) = node {
            match element.name.as_str() {
                "p" => {
                    if is_blank_paragraph(&element) {
                        // This is a spacer paragraph like <p>&nbsp;</p>, treat as one blank line.
                        lines.push(Line::from(vec![]));
                    } else {
                        let mut spans = Vec::new();
                        walk_nodes_to_spans(&element.children, Style::default(), &mut spans);
                        lines.push(Line::from(spans));
                        // Add a blank line for spacing after the paragraph
                        lines.push(Line::from(vec![]));
                    }
                }
                "pre" => {
                    let mut pre_lines = vec![Line::default()];
                    walk_and_append_to_lines(&element.children, Style::default(), &mut pre_lines);

                    // The html-parser might introduce a leading newline from formatting.
                    if let Some(first_line) = pre_lines.get(0) {
                        if first_line.spans.iter().all(|s| s.content.trim().is_empty()) {
                            pre_lines.remove(0);
                        }
                    }
                    // Also remove trailing empty line if it only contains whitespace
                    if let Some(last_line) = pre_lines.last() {
                        if last_line.spans.iter().all(|s| s.content.trim().is_empty()) {
                            pre_lines.pop();
                        }
                    }
                    lines.extend(pre_lines);
                    // Add a blank line for spacing after the pre block
                    lines.push(Line::from(vec![]));
                }
                "ul" => {
                    for child in &element.children {
                        if let Node::Element(li_element) = child {
                            if li_element.name == "li" {
                                let mut item_spans =
                                    vec![Span::styled("• ", Style::default().fg(Color::Cyan))];
                                walk_nodes_to_spans(
                                    &li_element.children,
                                    Style::default(),
                                    &mut item_spans,
                                );
                                lines.push(Line::from(item_spans));
                            }
                        }
                    }
                    // Add a blank line for spacing after the list
                    lines.push(Line::from(vec![]));
                }
                _ => {} // Ignore other top-level tags
            }
        } else if let Node::Text(text) = node {
            // Handle top-level text nodes that are not just whitespace
            if !text.trim().is_empty() {
                lines.push(Line::from(text));
                lines.push(Line::from(vec![])); // Add spacing after
            }
        }
    }

    // Remove the very last blank line if it exists, to avoid trailing space
    if let Some(last_line) = lines.last() {
        if last_line.spans.is_empty() {
            lines.pop();
        }
    }

    lines
}

/// Recursively walks through HTML nodes and builds a vector of styled `Span`s.
fn walk_nodes_to_spans(nodes: &[Node], current_style: Style, spans: &mut Vec<Span<'static>>) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                // Decode HTML entities like `&#39;` or `&amp;`
                let decoded_text = html_escape::decode_html_entities(text);
                // Convert the resulting Cow<str> to an owned String for a 'static lifetime.
                spans.push(Span::styled(decoded_text.to_string(), current_style));
            }
            Node::Element(element) => {
                let new_style = match element.name.as_str() {
                    "strong" | "b" => current_style.add_modifier(Modifier::BOLD),
                    "em" | "i" => current_style.add_modifier(Modifier::ITALIC),
                    "code" => current_style.fg(Color::LightYellow).bg(Color::DarkGray),
                    "sup" => {
                        if let Some(Node::Text(text)) = element.children.get(0) {
                            let sup_text = text
                                .chars()
                                .map(|c| match c {
                                    '0' => '⁰',
                                    '1' => '¹',
                                    '2' => '²',
                                    '3' => '³',
                                    '4' => '⁴',
                                    '5' => '⁵',
                                    '6' => '⁶',
                                    '7' => '⁷',
                                    '8' => '⁸',
                                    '9' => '⁹',
                                    '+' => '⁺',
                                    '-' => '⁻',
                                    _ => c,
                                })
                                .collect::<String>();
                            spans.push(Span::styled(sup_text, current_style));
                        }
                        continue;
                    }
                    // This handles the `span` tag in the user's example, just passing the style through.
                    "span" => current_style,
                    _ => current_style,
                };
                walk_nodes_to_spans(&element.children, new_style, spans);
            }
            _ => {}
        }
    }
}

/// Recursively walks nodes for `<pre>` blocks, creating new lines on `\n`.
/// This preserves styling across line breaks.
fn walk_and_append_to_lines(nodes: &[Node], style: Style, lines: &mut Vec<Line<'static>>) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                // Decode entities before splitting into lines.
                let decoded_text = html_escape::decode_html_entities(text);
                let mut content_lines = decoded_text.split('\n');

                // Handle the first part of the text, which belongs to the current line.
                if let Some(first_line_part) = content_lines.next() {
                    if !first_line_part.is_empty() {
                        lines
                            .last_mut()
                            .unwrap()
                            .spans
                            .push(Span::styled(first_line_part.to_string(), style));
                    }
                }
                // Handle subsequent parts, each starting a new line.
                for remaining_line_part in content_lines {
                    lines.push(Line::from(Span::styled(
                        remaining_line_part.to_string(),
                        style,
                    )));
                }
            }
            Node::Element(element) => {
                let new_style = match element.name.as_str() {
                    "strong" | "b" => style.add_modifier(Modifier::BOLD),
                    "em" | "i" => style.add_modifier(Modifier::ITALIC),
                    "code" => style.fg(Color::LightYellow).bg(Color::DarkGray),
                    _ => style,
                };
                walk_and_append_to_lines(&element.children, new_style, lines);
            }
            _ => {}
        }
    }
}

/// Checks if a <p> element only contains &nbsp; or is empty.
fn is_blank_paragraph(p_element: &html_parser::Element) -> bool {
    if p_element.children.is_empty() {
        return true;
    }
    if p_element.children.len() == 1 {
        if let Some(Node::Text(text)) = p_element.children.get(0) {
            // The parser converts &nbsp; to a non-breaking space character \u{a0}
            return text.trim() == "\u{a0}" || text.trim().is_empty();
        }
    }
    false
}
