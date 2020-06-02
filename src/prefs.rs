use alsa::seq::Addr;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum PortDir {
    Duplex,
    Input,
    Output,
}

impl Default for PortDir {
    fn default() -> Self {
        PortDir::Duplex
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum DeviceRole {
    Broadcast,
    Monitor,
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct DevicePrefs {
    pub port_name: String,
    pub port_dir: PortDir,
    pub alias: Option<String>,
    #[serde(default)]
    pub roles: Vec<DeviceRole>,
}

impl DevicePrefs {
    pub fn from_port(port_name: String) -> Self {
        let mut dev = DevicePrefs::default();
        dev.port_name = port_name;
        dev
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Default)]
pub struct PrefsModel {
    pub devices: Vec<DevicePrefs>,
}

#[derive(Debug, Clone, Default)]
pub struct UserPrefs {
    pub prefs_model: PrefsModel,
    known_ports: HashMap<String, DevicePrefs>,
    port_alias: HashMap<String, String>,
}

impl UserPrefs {
    pub fn load_from<R: Read>(read: R) -> Result<Self, Box<dyn Error>> {
        let prefs: PrefsModel = serde_yaml::from_reader(read)?;
        let ports = prefs.devices.iter().map(|d| (d.port_name.clone(), d.clone())).collect();
        let alias = prefs
            .devices
            .iter()
            .filter_map(|d| d.alias.as_ref().map(|a| (a.clone(), d.port_name.clone())))
            .collect();
        Ok(UserPrefs { prefs_model: prefs, known_ports: ports, port_alias: alias })
    }

    pub fn save_to<W: Write>(&self, write: &mut W) -> Result<(), Box<dyn Error>> {
        Ok(serde_yaml::to_writer(write, &self.prefs_model)?)
    }

    pub fn get_port_prefs(&self, port_name: &str) -> Option<&DevicePrefs> {
        self.known_ports.get(port_name)
    }

    pub fn resolve_to_alias(&self, name: &str) -> String {
        self.known_ports.get(name).and_then(|pconf| pconf.alias.clone()).unwrap_or(name.to_string())
    }

    pub fn resolve_to_portname(&self, name: &str) -> String {
        self.port_alias.get(name).map(|port_name| port_name.clone()).unwrap_or(name.to_string())
    }
}
