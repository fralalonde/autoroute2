use std::io;
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::ports::{new_port_sub, SYSTEM_ANNOUNCE_PORT, SYSTEM_CLIENT};
use alsa::seq;
use alsa::seq::{Addr, PortSubscribe};
use std::error::Error;
use std::ffi::CString;
use std::sync::mpsc::SendError;
use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    KeyPressed(Key),
    MidiPortsChanged(Vec<String>),
}

pub struct AppEvents {
    sources: Vec<thread::JoinHandle<()>>,
    rx: mpsc::Receiver<Event>,
}

pub type EventSource = fn(mpsc::Sender<Event>) -> Result<(), Box<dyn Error>>;

pub fn keyboard(tx: mpsc::Sender<Event>) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    for evt in stdin.keys() {
        if let Ok(key) = evt {
            tx.send(Event::KeyPressed(key))?;
        }
    }
    Ok(())
}

pub fn alsa_announce(tx: mpsc::Sender<Event>) -> Result<(), Box<dyn Error>> {
    // err = snd_seq_open(&seq, "autoroute2-watch", SND_SEQ_OPEN_DUPLEX, 0);
    // err = snd_seq_set_client_name(seq, "aseqdump");
    let seq = seq::Seq::open(None, None, false)?;

    let port_name = CString::new("Autoroute2 System Announce Monitor")?;
    let listen_port = seq
        .create_simple_port(
            &port_name,
            seq::PortCap::WRITE | seq::PortCap::SUBS_WRITE,
            seq::PortType::MIDI_GENERIC | seq::PortType::APPLICATION,
        )
        .expect("Announce monitor port");

    // for (i = 0; i < port_count; ++i) {
    //     err = snd_seq_connect_from(seq, 0, ports[i].client, ports[i].port);
    let announce = Addr { client: SYSTEM_CLIENT, port: SYSTEM_ANNOUNCE_PORT };
    let listen = Addr { client: seq.client_id()?, port: listen_port };
    let ps = new_port_sub(announce, listen)?;

    seq.subscribe_port(&ps)?;

    // err = snd_seq_nonblock(seq, 1);
    let mut input = seq.input();

    loop {
        // snd_seq_poll_descriptors(seq, pfds, npfds, POLLIN);
        // if (poll(pfds, npfds, -1) < 0) break;
        // err = snd_seq_event_input(seq, &event);
        match input.event_input()?.get_type() {
            seq::EventType::PortChange | seq::EventType::PortExit | seq::EventType::PortStart => {
                let mut ports = vec![];
                for client in seq::ClientIter::new(&seq) {
                    if client.get_client() == SYSTEM_CLIENT {
                        continue;
                    }
                    for p in seq::PortIter::new(&seq, client.get_client()) {
                        if let Ok(name) = p.get_name() {
                            ports.push(name.to_owned());
                        }
                    }
                }
                tx.send(Event::MidiPortsChanged(ports))?;
            }
            _ => {}
        }
    }
    Ok(())
}

impl AppEvents {
    pub fn with_sources(event_sources: Vec<EventSource>) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut src_handles = vec![];
        for evsrc in event_sources.into_iter() {
            let tx = tx.clone();
            src_handles.push(thread::spawn(move || {
                if let Err(e) = (evsrc)(tx) {
                    eprintln!("{:?}", e);
                };
            }))
        }
        AppEvents { sources: src_handles, rx }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.rx.recv()
    }
}
