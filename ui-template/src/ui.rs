// ABOUTME: Config-driven UI rendering for the sandbox
// ABOUTME: Adapted from main recall app with toggleable UI elements

use crate::config::UiConfig;
use crate::mock::{Role, SearchResult, SessionSource};
use crate::theme::theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

/// Application state for the sandbox
pub struct SandboxState {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub selected: usize,
    pub list_scroll: usize,
    pub preview_scroll: usize,
    pub preview_scrollable: bool,
}

impl SandboxState {
    pub fn new(results: Vec<SearchResult>) -> Self {
        Self {
            query: String::new(),
            results,
            selected: 0,
            list_scroll: 0,
            preview_scroll: 0,
            preview_scrollable: false,
        }
    }

    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.results.get(self.selected)
    }

    pub fn on_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            self.preview_scroll = 0;
        }
    }

    pub fn on_down(&mut self) {
        if self.selected + 1 < self.results.len() {
            self.selected += 1;
            self.preview_scroll = 0;
        }
    }

    pub fn scroll_preview_up(&mut self, amount: usize) {
        self.preview_scroll = self.preview_scroll.saturating_sub(amount);
    }

    pub fn scroll_preview_down(&mut self, amount: usize) {
        self.preview_scroll = self.preview_scroll.saturating_add(amount);
    }

    pub fn on_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn on_backspace(&mut self) {
        self.query.pop();
    }
}

/// Main UI rendering
pub fn render(frame: &mut Frame, state: &mut SandboxState, config: &UiConfig) {
    let area = frame.area();

    // Build dynamic layout based on config
    let mut constraints = Vec::new();

    if config.show_search_bar {
        constraints.push(Constraint::Length(3)); // Search bar
        constraints.push(Constraint::Length(1)); // Spacing
    }

    constraints.push(Constraint::Min(0)); // Content area

    if config.show_status_bar {
        constraints.push(Constraint::Length(1)); // Spacing before status bar
        constraints.push(Constraint::Length(1)); // Status bar
    }

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut layout_idx = 0;

    // Search bar
    if config.show_search_bar {
        let search_with_margin = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(main_layout[layout_idx]);
        render_search_bar(frame, state, config, search_with_margin[1]);
        layout_idx += 2; // search bar + spacing
    }

    // Content area
    let content_area = main_layout[layout_idx];
    let content_with_padding = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(content_area);

    render_content(frame, state, config, content_with_padding[1]);
    layout_idx += 1;

    // Status bar
    if config.show_status_bar {
        layout_idx += 1; // Skip spacing
        let status_with_padding = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(main_layout[layout_idx]);
        render_status_bar(frame, state, config, status_with_padding[1]);
    }

    // Help overlay (rendered last to be on top)
    if config.show_help {
        render_help_overlay(frame, area);
    }
}

fn render_content(frame: &mut Frame, state: &mut SandboxState, config: &UiConfig, area: Rect) {
    let show_results = config.show_results_pane;
    let show_preview = config.show_preview_pane;

    match (show_results, show_preview) {
        (true, true) => {
            // Two-pane layout
            let content_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(config.results_width_percent),
                    Constraint::Length(2),
                    Constraint::Percentage(config.preview_width_percent()),
                ])
                .split(area);
            render_results_list(frame, state, content_layout[0]);
            render_preview(frame, state, content_layout[2]);
        }
        (true, false) => {
            render_results_list(frame, state, area);
        }
        (false, true) => {
            render_preview(frame, state, area);
        }
        (false, false) => {
            // Nothing to render in content area
            let t = theme();
            let msg = Paragraph::new("All content panes hidden. Press 3 or 4 to show.")
                .style(Style::default().fg(t.dim_fg));
            frame.render_widget(msg, area);
        }
    }
}

fn render_search_bar(frame: &mut Frame, state: &SandboxState, config: &UiConfig, area: Rect) {
    let t = theme();

    // Scope widget content
    let scope_label = "everywhere";

    // Build scope widget (only if enabled)
    let scope_widget: Vec<Span> = if config.show_scope_widget {
        vec![
            Span::styled(" │ ", Style::default().fg(t.separator_fg)),
            Span::styled(" / ", Style::default().bg(t.keycap_bg)),
            Span::styled(format!(" {} ", scope_label), Style::default().fg(t.scope_label_fg)),
        ]
    } else {
        vec![]
    };

    let scope_width: usize = if config.show_scope_widget {
        3 + 3 + 1 + scope_label.len() + 1
    } else {
        0
    };

    let search_width = (area.width as usize).saturating_sub(scope_width + 1);

    // Build middle line with search on left, scope widget on right
    let middle_line = if state.query.is_empty() {
        let placeholder = " Search...";
        let padding = search_width.saturating_sub(placeholder.len());
        let mut spans = vec![
            Span::styled(placeholder, Style::default().fg(t.placeholder_fg)),
            Span::styled(" ".repeat(padding), Style::default()),
            Span::styled(" ", Style::default()),
        ];
        spans.extend(scope_widget.clone());
        Line::from(spans)
    } else {
        let query_len = 1 + state.query.chars().count() + 1;
        let padding = search_width.saturating_sub(query_len);
        let mut spans = vec![
            Span::raw(" "),
            Span::raw(&state.query),
            Span::styled("█", Style::default().fg(t.accent)),
            Span::raw(" ".repeat(padding)),
            Span::styled(" ", Style::default()),
        ];
        spans.extend(scope_widget.clone());
        Line::from(spans)
    };

    // Top and bottom lines with separator
    let separator_pos = search_width + 1;
    let (top_line, bottom_line) = if config.show_scope_widget {
        (
            Line::from(vec![
                Span::raw(" ".repeat(separator_pos)),
                Span::styled(" │ ", Style::default().fg(t.separator_fg)),
            ]),
            Line::from(vec![
                Span::raw(" ".repeat(separator_pos)),
                Span::styled(" │ ", Style::default().fg(t.separator_fg)),
            ]),
        )
    } else {
        (Line::from(""), Line::from(""))
    };

    let lines = vec![top_line, middle_line, bottom_line];
    let paragraph = Paragraph::new(lines).style(Style::default().bg(t.search_bg));
    frame.render_widget(paragraph, area);
}

fn render_results_list(frame: &mut Frame, state: &mut SandboxState, area: Rect) {
    let t = theme();
    let available_width = area.width.saturating_sub(2) as usize;

    if state.results.is_empty() {
        let paragraph = Paragraph::new(Span::styled(
            "No results",
            Style::default().fg(t.dim_fg),
        ));
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = state
        .results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let is_selected = i == state.selected;

            let time_ago = format_time_ago(result.session.timestamp);

            let source_color = match result.session.source {
                SessionSource::ClaudeCode => t.claude_source,
                SessionSource::CodexCli => t.codex_source,
            };

            let header_style = if is_selected {
                Style::default().fg(t.selection_header_fg)
            } else {
                Style::default()
            };

            let header_spans = vec![
                Span::styled("📁 ", header_style),
                Span::styled(&result.session.project_name, header_style),
                Span::styled("  ", header_style),
                Span::styled(
                    format!("{} {}", result.session.source.icon(), result.session.source.display_name()),
                    Style::default().fg(source_color),
                ),
                Span::styled(format!("  {}", time_ago), header_style),
            ];

            let snippet = truncate_snippet_around_match(&result.snippet, &state.query, available_width);
            let snippet_spans = highlight_matches_owned(&snippet, &state.query);

            let lines = vec![
                Line::from(header_spans),
                Line::from(
                    snippet_spans
                        .into_iter()
                        .map(|s| {
                            if s.style.add_modifier.contains(Modifier::BOLD) {
                                Span::styled(s.content, Style::default().fg(t.match_fg).add_modifier(Modifier::BOLD))
                            } else {
                                let fg = if is_selected { t.selection_snippet_fg } else { t.snippet_fg };
                                Span::styled(s.content, Style::default().fg(fg))
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Line::from(""),
            ];

            if is_selected {
                ListItem::new(lines).style(Style::default().bg(t.selection_bg))
            } else {
                ListItem::new(lines)
            }
        })
        .collect();

    let list = List::new(items);

    let lines_per_item = 3;
    let visible_items = (area.height as usize) / lines_per_item;

    if state.selected < state.list_scroll {
        state.list_scroll = state.selected;
    } else if state.selected >= state.list_scroll + visible_items && visible_items > 0 {
        state.list_scroll = state.selected - visible_items + 1;
    }

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));
    *list_state.offset_mut() = state.list_scroll;

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_preview(frame: &mut Frame, state: &mut SandboxState, area: Rect) {
    let t = theme();
    let Some(result) = state.selected_result() else {
        return;
    };

    let session = &result.session;
    let mut lines: Vec<Line> = Vec::new();
    let bubble_width = area.width.saturating_sub(4) as usize;

    let mut prev_role: Option<Role> = None;

    for (i, message) in session.messages.iter().enumerate() {
        let (accent_color, msg_bg) = match message.role {
            Role::User => (t.user_label, t.user_bubble_bg),
            Role::Assistant => match session.source {
                SessionSource::ClaudeCode => (t.claude_source, t.claude_bubble_bg),
                SessionSource::CodexCli => (t.codex_source, t.codex_bubble_bg),
            },
        };

        let speaker_changed = prev_role != Some(message.role);

        if i > 0 && speaker_changed {
            lines.push(Line::from(""));
        }

        if speaker_changed {
            let role_label = match message.role {
                Role::User => " You",
                Role::Assistant => match session.source {
                    SessionSource::ClaudeCode => " Claude",
                    SessionSource::CodexCli => " Codex",
                },
            };

            let time_str = format_time_ago(message.timestamp);

            lines.push(Line::from(vec![
                Span::styled(
                    role_label,
                    Style::default().fg(accent_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {}", time_str),
                    Style::default().fg(t.dim_fg),
                ),
            ]));
        }

        prev_role = Some(message.role);

        let wrapped_lines = wrap_text(&message.content, bubble_width);

        for (line_idx, display_line) in wrapped_lines.iter().take(12).enumerate() {
            let content_len = display_line.chars().count();
            let right_pad = bubble_width.saturating_sub(content_len);

            let mut spans = vec![Span::styled(" ", Style::default().bg(msg_bg))];

            if !display_line.is_empty() {
                let highlighted = highlight_matches_owned(display_line, &state.query);
                for span in highlighted {
                    spans.push(Span::styled(span.content, span.style.bg(msg_bg)));
                }
            }

            spans.push(Span::styled(" ".repeat(right_pad + 1), Style::default().bg(msg_bg)));
            lines.push(Line::from(spans));

            if line_idx == 11 && wrapped_lines.len() > 12 {
                let trunc_msg = format!("... ({} more lines)", wrapped_lines.len() - 12);
                lines.push(Line::from(Span::styled(
                    format!(" {:<width$}", trunc_msg, width = bubble_width + 1),
                    Style::default().fg(t.dim_fg).bg(msg_bg),
                )));
            }
        }
    }

    let visible_height = area.height as usize;
    let max_scroll = lines.len().saturating_sub(visible_height.min(lines.len()));
    state.preview_scrollable = max_scroll > 0;
    state.preview_scroll = state.preview_scroll.min(max_scroll);

    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(state.preview_scroll)
        .collect();

    let paragraph = Paragraph::new(visible_lines);
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, state: &SandboxState, config: &UiConfig, area: Rect) {
    let t = theme();
    let keycap = Style::default().bg(t.keycap_bg);
    let label = Style::default();
    let dim = Style::default().fg(t.dim_fg);

    let spans = vec![
        Span::styled(" 1-5 ", keycap),
        Span::styled(" toggles ", label),
        Span::styled(" │ ", dim),
        Span::styled(" t ", keycap),
        Span::styled(format!(" {} ", config.theme_mode.display()), label),
        Span::styled(" │ ", dim),
        Span::styled(" [] ", keycap),
        Span::styled(format!(" {}% ", config.results_width_percent), label),
        Span::styled(" │ ", dim),
        Span::styled(" ? ", keycap),
        Span::styled(" help ", label),
        Span::styled(" │ ", dim),
        Span::styled(" q ", keycap),
        Span::styled(" quit", label),
    ];

    let hints = Line::from(spans);

    let count = Span::styled(
        format!(" {} sessions", state.results.len()),
        dim,
    );

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(count.width() as u16)])
        .split(area);

    frame.render_widget(Paragraph::new(hints), layout[0]);
    frame.render_widget(Paragraph::new(count), layout[1]);
}

fn render_help_overlay(frame: &mut Frame, area: Rect) {
    let t = theme();

    // Center the help box
    let width = 50.min(area.width.saturating_sub(4));
    let height = 16.min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    let help_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, help_area);

    let help_text = vec![
        Line::from(Span::styled(" UI Template Sandbox ", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(" Keyboard Controls:"),
        Line::from(""),
        Line::from("  1       Toggle search bar"),
        Line::from("  2       Toggle status bar"),
        Line::from("  3       Toggle results pane"),
        Line::from("  4       Toggle preview pane"),
        Line::from("  5       Toggle scope widget"),
        Line::from("  t       Cycle theme (auto/dark/light)"),
        Line::from("  [ ]     Adjust pane split"),
        Line::from("  ↑↓      Navigate results"),
        Line::from("  PgUp/Dn Scroll preview"),
        Line::from("  r       Reset to defaults"),
        Line::from("  ?       Toggle this help"),
        Line::from("  q/Esc   Quit"),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .style(Style::default().bg(t.search_bg));

    frame.render_widget(help, help_area);
}

// === Helper functions (copied from main app) ===

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            result.push(String::new());
            continue;
        }

        let mut current_line = String::new();
        let mut current_width = 0;

        for word in line.split_whitespace() {
            let word_width = word.chars().count();

            if current_width == 0 {
                if word_width > max_width {
                    for chunk in word.chars().collect::<Vec<_>>().chunks(max_width) {
                        result.push(chunk.iter().collect());
                    }
                } else {
                    current_line = word.to_string();
                    current_width = word_width;
                }
            } else if current_width + 1 + word_width <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
                current_width += 1 + word_width;
            } else {
                result.push(current_line);
                if word_width > max_width {
                    for chunk in word.chars().collect::<Vec<_>>().chunks(max_width) {
                        result.push(chunk.iter().collect());
                    }
                    current_line = String::new();
                    current_width = 0;
                } else {
                    current_line = word.to_string();
                    current_width = word_width;
                }
            }
        }

        if !current_line.is_empty() {
            result.push(current_line);
        }
    }

    if result.is_empty() {
        result.push(String::new());
    }

    result
}

fn highlight_matches_owned(text: &str, query: &str) -> Vec<Span<'static>> {
    let t = theme();
    if query.is_empty() {
        return vec![Span::raw(text.to_owned())];
    }

    let lower_text = text.to_lowercase();
    let query_words: Vec<&str> = query.split_whitespace().filter(|w| !w.is_empty()).collect();
    if query_words.is_empty() {
        return vec![Span::raw(text.to_owned())];
    }

    let mut matches: Vec<(usize, usize)> = Vec::new();
    for word in &query_words {
        let lower_word = word.to_lowercase();
        for (match_start_lower, matched_str) in lower_text.match_indices(&lower_word) {
            let char_offset = lower_text[..match_start_lower].chars().count();
            let start = text.char_indices().nth(char_offset).map(|(i, _)| i).unwrap_or(text.len());
            let match_char_len = matched_str.chars().count();
            let end = text[start..].char_indices()
                .nth(match_char_len)
                .map(|(i, _)| start + i)
                .unwrap_or(text.len());
            matches.push((start, end));
        }
    }

    matches.sort_by_key(|m| m.0);
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in matches {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    let mut spans = Vec::new();
    let mut last_end = 0;

    for (start, end) in merged {
        if start > last_end {
            spans.push(Span::raw(text[last_end..start].to_owned()));
        }
        spans.push(Span::styled(
            text[start..end].to_owned(),
            Style::default()
                .fg(t.match_fg)
                .add_modifier(Modifier::BOLD),
        ));
        last_end = end;
    }

    if last_end < text.len() {
        spans.push(Span::raw(text[last_end..].to_owned()));
    }

    if spans.is_empty() {
        spans.push(Span::raw(text.to_owned()));
    }

    spans
}

fn truncate_snippet_around_match(snippet: &str, query: &str, max_width: usize) -> String {
    let chars: Vec<char> = snippet.chars().collect();
    if chars.len() <= max_width {
        return snippet.to_string();
    }

    let lower_snippet: String = chars.iter().collect::<String>().to_lowercase();
    let query_words: Vec<&str> = query.split_whitespace().collect();
    let mut best_match: Option<(usize, usize)> = None;

    for word in &query_words {
        let lower_word = word.to_lowercase();
        if let Some(byte_pos) = lower_snippet.find(&lower_word) {
            let char_pos = lower_snippet[..byte_pos].chars().count();
            let word_len = lower_word.chars().count();
            if best_match.is_none() || char_pos < best_match.unwrap().0 {
                best_match = Some((char_pos, word_len));
            }
        }
    }

    if let Some((match_char_pos, query_char_len)) = best_match {
        let half_width = max_width.saturating_sub(query_char_len) / 2;
        let start = match_char_pos.saturating_sub(half_width);
        let end = (start + max_width).min(chars.len());
        let start = if end == chars.len() {
            end.saturating_sub(max_width)
        } else {
            start
        };

        let mut result: String = chars[start..end].iter().collect();

        if start > 0 {
            result = format!("...{}", result.trim_start());
        }
        if end < chars.len() {
            result = format!("{}...", result.trim_end());
        }

        result
    } else {
        let truncated: String = chars.iter().take(max_width).collect();
        if chars.len() > max_width {
            format!("{}...", truncated.trim_end())
        } else {
            truncated
        }
    }
}

fn format_time_ago(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(timestamp);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{}d ago", duration.num_days())
    } else if duration.num_weeks() < 4 {
        format!("{}w ago", duration.num_weeks())
    } else {
        timestamp.format("%b %d").to_string()
    }
}
