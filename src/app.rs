use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use strum::{Display, EnumIter, FromRepr};
use std::sync::{Arc, mpsc};
use std::thread;

use crate::{client::{Client, ManagementClient}, views::QueuesView};

#[derive(Debug)]
pub struct App
{
    pub selected_tab: SelectedTab,
    state: AppState,
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
        let (queue_tx, queue_rx) = mpsc::channel();
        let client = Arc::new(Client::new("http://localhost:15672", "admin", Some(String::from("admin"))));
        let thread_client = client.clone();
        let queues = QueuesView::new(client, queue_rx);
    
        thread::spawn(move || {

            loop {
                let queues_info = thread_client.get_queues_info();
    
                let _ = queue_tx.send(queues_info);

                thread::sleep(Duration::from_secs(5));
            }
        });
        
        Self {
            selected_tab: SelectedTab::default(), 
            state: AppState::default(),
            queues: queues,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state == AppState::Running {
            let _ = self.handle_events();
            terminal.draw(|frame| self.draw(frame))?;
        }
        
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                },
                _ => {},
            };
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            KeyCode::Left => self.previous_tab(),
            KeyCode::Right => self.next_tab(),
            _ => {},
        };
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();

    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn exit(&mut self) {
        self.state = AppState::Quitting;
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
