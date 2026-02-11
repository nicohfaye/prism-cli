use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use kube::Client;
use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;

use crate::k8s::{self, DeploymentInfo, PodInfo};
use crate::ui;

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const TICK_RATE: Duration = Duration::from_millis(200);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Pods,
    Deployments,
}

pub struct App {
    pub active_panel: Panel,
    pub pods: Vec<PodInfo>,
    pub deployments: Vec<DeploymentInfo>,
    pub pods_state: TableState,
    pub deployments_state: TableState,
    pub error: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut pods_state = TableState::default();
        pods_state.select(Some(0));

        Self {
            active_panel: Panel::Pods,
            pods: Vec::new(),
            deployments: Vec::new(),
            pods_state,
            deployments_state: TableState::default(),
            error: None,
            should_quit: false,
        }
    }

    /// Main loop: poll K8s data and handle input.
    pub async fn run(&mut self, terminal: &mut DefaultTerminal, client: &Client) -> Result<()> {
        // initial data fetch
        self.refresh(client).await;

        let mut last_poll = std::time::Instant::now();

        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            // small timeout to handle input events
            if event::poll(TICK_RATE)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            if self.should_quit {
                break;
            }

            // refresh
            if last_poll.elapsed() >= POLL_INTERVAL {
                self.refresh(client).await;
                last_poll = std::time::Instant::now();
            }
        }

        Ok(())
    }

    /// Demo run loop
    pub async fn run_demo(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if event::poll(TICK_RATE)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab => self.toggle_panel(),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_down(),
            KeyCode::Up | KeyCode::Char('k') => self.scroll_up(),
            KeyCode::Char('r') => { /* handled in run loop via flag */ }
            _ => {}
        }
    }

    fn toggle_panel(&mut self) {
        match self.active_panel {
            Panel::Pods => {
                self.active_panel = Panel::Deployments;
                if self.deployments_state.selected().is_none() && !self.deployments.is_empty() {
                    self.deployments_state.select(Some(0));
                }
            }
            Panel::Deployments => {
                self.active_panel = Panel::Pods;
                if self.pods_state.selected().is_none() && !self.pods.is_empty() {
                    self.pods_state.select(Some(0));
                }
            }
        }
    }

    fn scroll_down(&mut self) {
        match self.active_panel {
            Panel::Pods => scroll(&mut self.pods_state, self.pods.len(), 1),
            Panel::Deployments => scroll(&mut self.deployments_state, self.deployments.len(), 1),
        }
    }

    fn scroll_up(&mut self) {
        match self.active_panel {
            Panel::Pods => scroll(&mut self.pods_state, self.pods.len(), -1),
            Panel::Deployments => scroll(&mut self.deployments_state, self.deployments.len(), -1),
        }
    }

    async fn refresh(&mut self, client: &Client) {
        match k8s::fetch_pods(client).await {
            Ok(pods) => {
                self.pods = pods;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("pods: {}", e)),
        }

        match k8s::fetch_deployments(client).await {
            Ok(deps) => {
                self.deployments = deps;
                if self.error.is_none() {
                    self.error = None;
                }
            }
            Err(e) => self.error = Some(format!("deployments: {}", e)),
        }
    }
}

fn scroll(state: &mut TableState, len: usize, delta: i32) {
    if len == 0 {
        return;
    }
    let current = state.selected().unwrap_or(0) as i32;
    let next = (current + delta).clamp(0, len as i32 - 1) as usize;
    state.select(Some(next));
}
