# autoroute2
USB MIDI automatic patchbay / router for Linux. 
Virtually "connects" MIDI devices together as they are plugged in, according to configuration.
Leverages the ALSA sequencer for low-latency routing between devices. 
Designed for Raspberry Pi + USB Hub, runs fine on bigger Linux computers.
Runs unattended, but can be accessed live through command-line or HTTP for configuration.

## Version 2 vs Version 1
This is version 2 of _autoroute_. 
It is a complete rewrite, in a different language (Rust). 
It does more (way more), faster, more reliably yet is just as easy to use as the old version.
It uses a native binary rather than a script for higher speed and less dependencies. 
It does _not_ use the same configuration files or commands as Version 1 (although there are similarities)
Version 1 can still be found [here](https://github.com/fralalonde/autoroute)
 
## Usage

`autoroute2 (devices [--config=configfile] | connect [--config=configfile] )`

_Autoroute2_ is simple to use:
- `autoroute2 list` shows all available USB MIDI device ports
- `autoroute2 connect` wires devices together according to the config file (`./autoroute.conf` is used by default)
- `autoroute2 systemd-unit --configuration=[config_file] --state=[state_dir]` generates a systemd unit file to be installed.

_Autoroute_ requires python 3.5. Built-in service installer requires `systemd`.

## Installation

At a terminal, from the raspberrypi where `autoroute2` is intended to run:

```
wget [RELEASE]
unzip [FILE]

# get device names
./autoroute2 devices

# set up udev to rerun autoroute everytime USB MIDI config changes
sudo python3 install.py

# reload udev (or just reboot)
sudo udevadm control --reload-rules && udevadm trigger
```

## Configuration

The included `autoroute.yaml` is a sample, and needs to edited with entries from your own setup.

Config file entries can be of two types:

Lines starting with `#` are comments.
Device names have to match exactly the ones reported by the `autoroute list` command, excluding the `[device,port]`. 
If a configured device is not currently connected it is simply ignored.

```
devices:
  - port-name: Pyramid MIDI USB MIDI 1
    alias: Pyramid
    roles:
      # sends to everyone
      - Broadcast
      # receives from everyone
      - Monitor
  - port-name: GS-10 MIDI
    # default is duplex
    port-dir: Input
    alias: OctaSeq
    roles:
      - Broadcast
  - port-name: BCF2000 MIDI 1
    alias: BCF2000
  - port-name: Arturia BeatStep MIDI 1
    alias: BeatStep
    roles:
      - Broadcast
  - port-name: RD-8 MIDI 1
    alias: RD-8
  - port-name: USB Uno MIDI Interface MIDI 1
    alias: "Micro Q"
  - port-name: Neutron(1) MIDI 1
    alias: Neutron
  - port-name: USB Uno MIDI Interface MIDI 1
    alias: Sub37
```

## TODO
filter notes, vel, CC, sysex, N/RPN, clock, etc

## Thanks
Adapted from https://neuma.studio/rpi-as-midi-host.html to handle multi-port devices and fixed config.

## Licence
Autoroute2 is licensed under the terms of the third edition of the Gnu General Public License (GPL v3.0).
See the LICENSE file for details.

## FAQ
Why choose that name?
- _Autoroute_ is french for _Highway_ and translates to german as _Autobahn_, which is a great [album](https://en.wikipedia.org/wiki/Autobahn_%28album%29).
