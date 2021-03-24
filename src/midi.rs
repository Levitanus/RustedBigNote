use super::note::Note;
use druid::Data;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiInputPorts};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Port {
    pub index: usize,
    pub name: Box<String>,
}
impl Data for Port {
    fn same(&self, other: &Self) -> bool {
        if other.index == self.index && other.name == self.name {
            return true;
        } else {
            return false;
        }
    }
}
impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Clone)]
struct MidiData {
    note: Option<Note>,
    ports_list: Vec<Port>,
}
impl MidiData {
    fn new() -> Self {
        MidiData {
            note: None,
            ports_list: Vec::new(),
        }
    }
}
impl Data for MidiData {
    fn same(&self, other: &Self) -> bool {
        if self.note == other.note && self.ports_list == other.ports_list {
            return true;
        } else {
            return false;
        }
    }
}

struct MidiHandler {
    client_name: String,
    selected_port: Option<Port>,
    midi_in: MidiInput,

    connection: Option<MidiInputConnection<Sender<Vec<u8>>>>,
    reciever: Option<Receiver<Vec<u8>>>,
}
impl MidiHandler {
    fn new(name: String) -> Self {
        MidiHandler {
            client_name: name.clone(),
            selected_port: None,
            midi_in: MidiInput::new(&name).unwrap(),
            connection: None,
            reciever: None,
        }
    }
    fn port_names(&self) -> Vec<Port> {
        let mut names: Vec<Port> = Vec::new();
        let midi_in = &self.midi_in;
        let ports_amount = midi_in.port_count();
        for port in 0..ports_amount {
            names.push(Port {
                index: port,
                name: Box::new(
                    midi_in
                        .port_name(midi_in.ports().get(port).unwrap())
                        .unwrap(),
                ),
            });
        }
        names.into()
    }
    fn connect(&mut self) {
        println!("init:");
        let ports = &self.midi_in.ports();
        let selected_port = self.selected_port.as_ref().unwrap();
        let port = ports.get(selected_port.index).unwrap();
        println!("in port name: {}", &self.midi_in.port_name(&port).unwrap());
        let (sender, reciever) = channel();
        self.reciever = Some(reciever);
        let midi_in = MidiInput::new(&self.client_name).unwrap();
        self.connection = Some(
            midi_in
                .connect(
                    &port,
                    "name",
                    |_stamp, message, sender| {
                        let mut packet = Vec::new();
                        packet.extend_from_slice(message);
                        sender.send(packet).unwrap();
                    },
                    sender,
                )
                .unwrap(),
        );
    }
}
