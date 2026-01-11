use crate::storage::{self, Prompt};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{error::Error, io};

const BANNER: &str = r#"
███████╗██╗      ██████╗ ██╗    ██╗██████╗ ██████╗  ██████╗ ███╗   ███╗██████╗ ████████╗
██╔════╝██║     ██╔═══██╗██║    ██║██╔══██╗██╔══██╗██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝
█████╗  ██║     ██║   ██║██║ █╗ ██║██████╔╝██████╔╝██║   ██║██╔████╔██║██████╔╝   ██║
██╔══╝  ██║     ██║   ██║██║███╗██║██╔═══╝ ██╔══██╗██║   ██║██║╚██╔╝██║██╔═══╝    ██║
██║     ███████╗╚██████╔╝╚███╔███╔╝██║     ██║  ██║╚██████╔╝██║ ╚═╝ ██║██║        ██║
╚═╝     ╚══════╝ ╚═════╝  ╚══╝╚══╝ ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═╝     ╚═╝╚═╝        ╚═╝
"#;

struct App {
    prompts: Vec<Prompt>,
    state: ListState,
}

impl App {
    fn new(prompts: Vec<Prompt>) -> App {
        let mut state = ListState::default();
        if !prompts.is_empty() {
            state.select(Some(0));
        }
        App { prompts, state }
    }

    fn next(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.prompts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.prompts.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.prompts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn run() -> Result<Option<String>, Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let prompts = storage::load_prompts();
    let mut app = App::new(prompts);
    let mut selected_alias = None;

    // Run loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(Some(alias)) = res {
        selected_alias = Some(alias);
    }

    Ok(selected_alias)
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
                .split(f.area());

            let banner = Paragraph::new(BANNER)
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(banner, chunks[0]);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);

            // Left: List
            let items: Vec<ListItem> = app
                .prompts
                .iter()
                .map(|p| {
                    let lines = vec![Line::from(Span::styled(
                        p.alias.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ))];
                    ListItem::new(lines).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Prompts "))
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, main_chunks[0], &mut app.state);

            // Right: Detail
            let selected_idx = app.state.selected();
            let detail_text = if let Some(i) = selected_idx {
                if let Some(prompt) = app.prompts.get(i) {
                    let mut text = vec![
                        Line::from(Span::styled(
                            format!("Alias: {}", prompt.alias),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )),
                        Line::from(Span::raw("")),
                        Line::from(Span::styled(
                            "Description:",
                            Style::default().fg(Color::Cyan),
                        )),
                        Line::from(Span::raw(&prompt.description)),
                        Line::from(Span::raw("")),
                        Line::from(Span::styled("Tags:", Style::default().fg(Color::Cyan))),
                        Line::from(Span::raw(prompt.tags.join(", "))),
                        Line::from(Span::raw("")),
                        Line::from(Span::styled("Template:", Style::default().fg(Color::Cyan))),
                    ];
                    for line in prompt.template.lines() {
                        text.push(Line::from(Span::raw(line)));
                    }
                    text
                } else {
                    vec![Line::from("No Prompt Selected")]
                }
            } else {
                vec![Line::from("No Prompts Found")]
            };

            let paragraph = Paragraph::new(detail_text)
                .block(Block::default().borders(Borders::ALL).title(" Preview "))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, main_chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                KeyCode::Enter => {
                    if let Some(i) = app.state.selected() {
                        if let Some(prompt) = app.prompts.get(i) {
                            return Ok(Some(prompt.alias.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
