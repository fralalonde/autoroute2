extern crate tui as tui_rs;

use structopt::StructOpt;

use alsa::seq::{Addr, PortSubscribe};
use alsa::{seq, Seq};

use crate::tui::event::{AppEvents, Event};
use crate::tui::model::Model;
use crate::tui::view;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Stdout;
use std::time::Duration;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use crate::ports::AlsaMidi;
use crate::prefs::{DevicePrefs, UserPrefs};
use tui_rs::backend::TermionBackend;
use tui_rs::Terminal;

mod ports;
mod prefs;
mod tui;

#[derive(StructOpt, Debug)]
#[structopt(name = "autoroute.py", about = "Automatically connect USB MIDI devices to each other")]
enum CmdAction {
    Connect { config_file: String },
    Ports { config_file: Option<String> },
    TUI { config_file: Option<String> },
}

// impl Default for CmdAction {
//     fn default() -> Self {
//         CmdAction::TUI { config_file: None }
//     }
// }

fn add_ports(seq: &AlsaMidi, mut prefs: UserPrefs) -> Result<(), Box<dyn Error>> {
    for p in &seq.ports {
        if prefs.get_port_prefs(p.0.as_str()) == None {
            prefs.prefs_model.devices.push(DevicePrefs::from_port(p.0.to_string()))
        }
    }
    serde_yaml::to_writer(io::stdout(), &prefs.prefs_model)?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cmd = CmdAction::from_args();

    match cmd {
        CmdAction::Connect { config_file } => {
            let file = File::open(config_file)?;
            let config = UserPrefs::load_from(&file)?;
            let ports = AlsaMidi::new()?;
            ports.update_subs(&config)?;
        }
        CmdAction::Ports { config_file } => {
            // if file is provided, it _must_ open successfully
            let mut prefs = match config_file.map(|f| File::open(f)) {
                Some(Err(e)) => return Err(e.into()),
                Some(Ok(f)) => UserPrefs::load_from(&f)?,
                None => UserPrefs::default(),
            };

            let ports = AlsaMidi::new()?;
            add_ports(&ports, prefs)?;
        }
        CmdAction::TUI { config_file } => {
            let mut prefs = match config_file.map(|f| File::open(f)) {
                Some(Err(e)) => return Err(e.into()),
                Some(Ok(f)) => UserPrefs::load_from(&f)?,
                None => UserPrefs::default(),
            };

            let events =
                AppEvents::with_sources(vec![tui::event::keyboard, tui::event::alsa_announce]);

            let stdout = io::stdout().into_raw_mode()?;
            // let stdout = MouseTerminal::from(stdout);
            let stdout = AlternateScreen::from(stdout);
            let backend = TermionBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;
            terminal.hide_cursor()?;

            let ports = AlsaMidi::new()?;
            let ports = ports.ports.keys().map(|k| k.to_string()).collect();
            let mut app = Model::new("USB MIDI Routing", ports, prefs);
            app.run(events, terminal)?;
        }
    }
    Ok(())
}
