use crate::ports::AlsaMidi;
use crate::prefs::UserPrefs;
use crate::tui::event::{AppEvents, Event};
use crate::tui::view;
use itertools::Itertools;
use std::error::Error;
use std::io;
use termion::event::Key;
use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Terminal;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct SingleSelectList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl SingleSelectList {
    pub fn with_items(items: Vec<String>) -> SingleSelectList {
        SingleSelectList { state: ListState::default(), items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn update_items(&mut self, items: Vec<String>) {
        // save any selection by value
        let saved: Option<String> = self.state.selected().map(|idx| self.items[idx].clone());
        self.items = items;
        if let Some(saved) = saved {
            // restore position by equal item or invalidate if selected item no longer there
            self.state.select(
                self.items
                    .iter()
                    .find_position(|item| item.as_bytes().eq(saved.as_bytes()))
                    .map(|z| z.0),
            )
        }
    }
}

pub struct Model<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub ports: SingleSelectList,
    pub prefs: UserPrefs,
}

impl<'a> Model<'a> {
    pub fn new(title: &'a str, ports: Vec<String>, prefs: UserPrefs) -> Model<'a> {
        Model {
            title,
            tabs: TabsState::new(vec!["Ports"]),
            ports: SingleSelectList::with_items(ports),
            prefs,
        }
    }

    pub fn refresh_ports(&mut self, ports: Vec<String>) {
        self.ports.update_items(ports);
    }

    pub fn run<B: Backend>(
        &mut self,
        events: AppEvents,
        mut terminal: Terminal<B>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|mut frame| view::draw_root(&mut frame, self))?;

            match events.next()? {
                Event::KeyPressed(key) => match key {
                    Key::Char(c) => match c {
                        'q' => return Ok(()),
                        _ => {}
                    },
                    Key::Up => self.ports.previous(),
                    Key::Down => self.ports.next(),
                    Key::Left => self.tabs.previous(),
                    Key::Right => self.tabs.next(),
                    _ => {}
                },
                Event::MidiPortsChanged(ports) => self.refresh_ports(ports),
            }
        }
    }
}
