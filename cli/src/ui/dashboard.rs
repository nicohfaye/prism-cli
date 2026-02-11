use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::app::{App, Panel};
use crate::ui::theme;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3), // header
        Constraint::Min(5),    // pods
        Constraint::Min(5),    // deployments
        Constraint::Length(1), // footer
    ])
    .split(f.area());

    render_header(f, chunks[0], app);
    render_pods(f, chunks[1], app);
    render_deployments(f, chunks[2], app);
    render_footer(f, chunks[3]);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let status = if app.error.is_some() {
        Span::styled(" ERROR ", theme::status_style("Failed"))
    } else {
        Span::styled(" Connected ", theme::status_style("Running"))
    };

    let title = Line::from(vec![
        Span::styled("  Prism", theme::header()),
        Span::styled(" │ ", ratatui::style::Style::default().fg(theme::BORDER)),
        Span::styled(
            "K8s Dashboard",
            ratatui::style::Style::default().fg(theme::TEXT_DIM),
        ),
        Span::raw("  "),
        status,
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(ratatui::style::Style::default().fg(theme::BORDER));

    let header = Paragraph::new(title).block(block);
    f.render_widget(header, area);
}

fn render_pods(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.active_panel == Panel::Pods;

    let rows: Vec<Row> = app
        .pods
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.name.clone()).style(theme::row_normal()),
                Cell::from(p.namespace.clone())
                    .style(ratatui::style::Style::default().fg(theme::TEXT_DIM)),
                Cell::from(p.status.clone()).style(theme::status_style(&p.status)),
                Cell::from(p.restarts.to_string()).style(theme::row_normal()),
                Cell::from(p.age.clone())
                    .style(ratatui::style::Style::default().fg(theme::TEXT_DIM)),
            ])
        })
        .collect();

    let header = Row::new(vec!["NAME", "NAMESPACE", "STATUS", "RESTARTS", "AGE"])
        .style(theme::table_header())
        .bottom_margin(1);

    let block = Block::default()
        .title(Span::styled(" Pods ", theme::header()))
        .borders(Borders::ALL)
        .border_style(theme::border(focused));

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(block)
    .row_highlight_style(theme::highlight());

    f.render_stateful_widget(table, area, &mut app.pods_state);
}

fn render_deployments(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.active_panel == Panel::Deployments;

    let rows: Vec<Row> = app
        .deployments
        .iter()
        .map(|d| {
            Row::new(vec![
                Cell::from(d.name.clone()).style(theme::row_normal()),
                Cell::from(d.namespace.clone())
                    .style(ratatui::style::Style::default().fg(theme::TEXT_DIM)),
                Cell::from(d.ready.clone()).style(theme::row_normal()),
                Cell::from(d.up_to_date.to_string()).style(theme::row_normal()),
                Cell::from(d.age.clone())
                    .style(ratatui::style::Style::default().fg(theme::TEXT_DIM)),
            ])
        })
        .collect();

    let header = Row::new(vec!["NAME", "NAMESPACE", "READY", "UP-TO-DATE", "AGE"])
        .style(theme::table_header())
        .bottom_margin(1);

    let block = Block::default()
        .title(Span::styled(" Deployments ", theme::header()))
        .borders(Borders::ALL)
        .border_style(theme::border(focused));

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(block)
    .row_highlight_style(theme::highlight());

    f.render_stateful_widget(table, area, &mut app.deployments_state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let keys = Line::from(vec![
        Span::styled(" Tab", ratatui::style::Style::default().fg(theme::ACCENT)),
        Span::styled(
            " switch  ",
            ratatui::style::Style::default().fg(theme::TEXT_DIM),
        ),
        Span::styled("j/k", ratatui::style::Style::default().fg(theme::ACCENT)),
        Span::styled(
            " scroll  ",
            ratatui::style::Style::default().fg(theme::TEXT_DIM),
        ),
        Span::styled("r", ratatui::style::Style::default().fg(theme::ACCENT)),
        Span::styled(
            " refresh  ",
            ratatui::style::Style::default().fg(theme::TEXT_DIM),
        ),
        Span::styled("q", ratatui::style::Style::default().fg(theme::ACCENT)),
        Span::styled(
            " quit",
            ratatui::style::Style::default().fg(theme::TEXT_DIM),
        ),
    ]);

    let footer = Paragraph::new(keys);
    f.render_widget(footer, area);
}
