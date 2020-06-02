use crate::prefs::{DevicePrefs, DeviceRole, PortDir, UserPrefs};
use crate::tui::event::AppEvents;
use alsa::seq::{Addr, PortSubscribe};
use alsa::seq::{PortCap, PortType};
use alsa::{seq, Seq};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::CStr;

// #[derive(Debug, PartialEq, Clone)]
// struct ConnectedDevice {
//     config: Option<MidiDevice>,
//     addr: Addr,
// }

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Sub {
    sender: Addr,
    dest: Addr,
}

pub struct AlsaMidi {
    seq: Seq,
    pub ports: HashMap<String, Addr>,
    pub subs: HashSet<Sub>,
}

pub const SYSTEM_CLIENT: i32 = 0;
pub const SYSTEM_TIMER_PORT: i32 = 0;
pub const SYSTEM_ANNOUNCE_PORT: i32 = 1;

impl AlsaMidi {
    pub fn new() -> Result<Self, alsa::Error> {
        let seq = seq::Seq::open(None, None, false)?;

        let mut subs: HashSet<Sub> = HashSet::new();
        let mut ports = HashMap::new();

        for client in seq::ClientIter::new(&seq) {
            if client.get_client() == SYSTEM_CLIENT {
                continue;
            }
            for p in seq::PortIter::new(&seq, client.get_client()) {
                // nameless device? ignored!
                if let Ok(name) = p.get_name() {
                    let name = name.to_owned();
                    ports.insert(name, Addr { client: p.get_client(), port: p.get_port() });
                }

                for s in seq::PortSubscribeIter::new(
                    &seq,
                    seq::Addr { client: p.get_client(), port: p.get_port() },
                    seq::QuerySubsType::WRITE,
                ) {
                    subs.insert(Sub { sender: s.get_sender(), dest: s.get_dest() });
                }
            }
        }
        Ok(AlsaMidi { seq, ports, subs })
    }
}

fn broadcast(
    port: (&String, &Addr),
    other: (&String, &Addr),
    oconfig: Option<&DevicePrefs>,
    subs: &mut HashSet<Sub>,
) {
    // don't broadcast to input devices
    if let Some(oconfig) = oconfig {
        if oconfig.port_dir == PortDir::Input {
            return;
        }
    }
    subs.insert(Sub { sender: *port.1, dest: *other.1 });
}

fn monitor(
    port: (&String, &Addr),
    other: (&String, &Addr),
    oconfig: Option<&DevicePrefs>,
    subs: &mut HashSet<Sub>,
) {
    if let Some(oconfig) = oconfig {
        // don't monitor broadcasters or output-only devices
        // TODO make this configurable?
        if oconfig.roles.contains(&DeviceRole::Broadcast) || oconfig.port_dir == PortDir::Output {
            return;
        }
    }
    subs.insert(Sub { dest: *port.1, sender: *other.1 });
}

impl AlsaMidi {
    pub fn watch(&self, event: &AppEvents) -> Result<(), Box<dyn Error>> {
        self.seq.create_simple_port(
            CStr::from_bytes_with_nul("".as_bytes())?,
            PortCap::WRITE,
            PortType::APPLICATION,
        )?;
        Ok(())
    }

    pub fn update_subs(&self, user: &UserPrefs) -> Result<(), alsa::Error> {
        let mut expected_subs: HashSet<Sub> = HashSet::new();
        for port in &self.ports {
            if let Some(pconfig) = user.get_port_prefs(port.0) {
                for other in &self.ports {
                    if other == port {
                        continue;
                    }
                    let oconfig = user.get_port_prefs(other.0);
                    for role in &pconfig.roles {
                        match role {
                            DeviceRole::Broadcast => {
                                broadcast(port, other, oconfig, &mut expected_subs)
                            }
                            DeviceRole::Monitor => {
                                monitor(port, other, oconfig, &mut expected_subs)
                            }
                        }
                    }
                }
            }
        }

        for s in expected_subs.difference(&self.subs) {
            let ps = new_port_sub(s.sender, s.dest)?;
            self.seq.subscribe_port(&ps)?
        }

        for s in self.subs.difference(&expected_subs) {
            if let Err(err) = self.seq.unsubscribe_port(s.sender, s.dest) {
                eprintln!("Could not unsubscribe {:?} from {:?}: {}", s.sender, s.dest, err)
            }
        }
        Ok(())
    }
}

pub fn new_port_sub(sender: Addr, dest: Addr) -> Result<PortSubscribe, alsa::Error> {
    let ps = PortSubscribe::empty()?;
    ps.set_sender(sender);
    ps.set_dest(dest);
    Ok(ps)
}
