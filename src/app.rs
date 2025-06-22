use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use strum::{Display, EnumIter, FromRepr};
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, interval};

use crate::{client::Client, views::QueuesView};

#[derive(Debug)]
pub struct App
{
    pub selected_tab: Mutex<SelectedTab>,
    state: Mutex<AppState>,
    pub queues: QueuesView<Client>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Overview")]
    Overview,
    #[strum(to_string = "Exchanges")]
    Exchanges,
    #[strum(to_string = "Queues")]
    Queues,
}

impl App
{
    pub fn new() -> Self {
        let client = Arc::new(Client::new("http://localhost:15672", "admin", Some(String::from("admin"))));
        let queues = QueuesView::new(client);

        Self {
            selected_tab: Mutex::new(SelectedTab::default()),
            state: Mutex::new(AppState::default()),
            queues: queues,
        }
    }

    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut ticker = interval(Duration::from_secs(5));

        while self.is_running() {
            tokio::select! {
                _ = ticker.tick() => {
                    self.update().await;
                }
                _ = self.handle_events() => {}
            }
            
            terminal.draw(|frame| self.draw(frame))?;
        }
        
        Ok(())
    }

    fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();

        *state == AppState::Running
    }

    async fn update(&self) {
        let tab = {
            let selected_tab = self.selected_tab.lock().unwrap();

            (*selected_tab).clone()
        };

        match tab {
            SelectedTab::Overview => {},
            SelectedTab::Exchanges => {},
            SelectedTab::Queues => self.queues.update().await,
        };
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    async fn handle_events(&self) {
        let key_event = tokio::task::spawn_blocking(|| {
            loop {
                match event::read().unwrap() {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => return key_event,
                    _ => {},
                };
            }
        
        }).await.unwrap();

        self.handle_key_event(key_event);
    }

    fn handle_key_event(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.previous_tab(),
            KeyCode::Right => self.next_tab(),
            _ => {},
        };
    }

    pub fn next_tab(&self) {
        let mut selected_tab = self.selected_tab.lock().unwrap();

        *selected_tab = (*selected_tab).next();
    }

    pub fn previous_tab(&self) {
        let mut selected_tab = self.selected_tab.lock().unwrap();

        *selected_tab = (*selected_tab).previous();
    }

    fn exit(&self) {
        let mut state = self.state.lock().unwrap();

        *state = AppState::Quitting;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}
