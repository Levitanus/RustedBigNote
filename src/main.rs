extern crate chrono;
extern crate iced;
extern crate midir;

use std::cell::RefCell;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::option::Option::Some;
use std::sync::mpsc::{channel, Receiver, Sender};

use iced::{
    button, executor, pick_list, scrollable, time, Align, Application, Button, Column, Command,
    Container, Element, Length, PickList, Scrollable, Settings, Space, Subscription, Text,
};
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiInputPorts};

pub fn main() -> iced::Result {
    BigNote::run(Settings::default())
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Port {
    pub index: usize,
    pub name: Box<String>,
}
impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let midi_in = MidiInput::new("MyBigNote").unwrap();
        write!(
            f,
            "{}",
            // midi_in
            //     .port_name(midi_in.ports().get(self.index).unwrap())
            //     .unwrap()
            &self.name
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    PortSelected(Port),
    Tick(chrono::DateTime<chrono::Local>),
}

struct BigNote {
    note: i32,
    need_init: bool,
    client_name: String,
    scroll: scrollable::State,
    ports_list: pick_list::State<Port>,
    selected_port: Option<Port>,
    midi_in: MidiInput,

    connection: Option<MidiInputConnection<Sender<Vec<u8>>>>,
    reciever: Option<Receiver<Vec<u8>>>,
}

impl BigNote {
    fn ports_names(&self) -> Vec<Port> {
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
                    |stamp, message, sender| {
                        // println!("{}: {:?} len = {}", stamp, message, message.len());
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

impl Application for BigNote {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (BigNote, Command<self::Message>) {
        (
            BigNote {
                note: -1,
                need_init: true,
                client_name: String::from("MyBigNote"),
                connection: Option::None,
                ports_list: pick_list::State::<Port>::default(),
                scroll: scrollable::State::default(),
                selected_port: None,
                midi_in: MidiInput::new("MyBigNote").unwrap(),
                reciever: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("MyBigNote by Levitanus")
    }

    fn view(&mut self) -> Element<Message> {
        let port_names = self.ports_names();
        let pick_list = PickList::new(
            &mut self.ports_list,
            port_names,
            self.selected_port.clone(),
            Message::PortSelected,
        );

        // let mut content = Scrollable::new(&mut self.scroll)
        //     .width(Length::Fill)
        //     .align_items(Align::Center)
        //     .spacing(10)
        //     .push(Space::with_height(Length::Units(600)))
        //     .push(Text::new("Which is your favorite String?"))
        //     .push(pick_list);

        // content = content.push(Space::with_height(Length::Units(600)));

        Container::new(pick_list)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PortSelected(port) => {
                println!("PortSelected");
                self.selected_port = Some(port);
                self.connect();
            }
            Message::Tick(_loc_time) => {
                if self.reciever.is_some() {
                    let reciever = &self.reciever.as_ref().unwrap();
                    loop {
                        let recv_result = reciever.try_recv();
                        if !recv_result.is_err() {
                            println!("{:?}", recv_result.as_ref().unwrap());
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let fps = 30;
        time::every(std::time::Duration::from_millis(1000 / fps))
            .map(|_| Message::Tick(chrono::Local::now()))
    }
}
