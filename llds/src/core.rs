#[derive(Debug)]
pub struct Packet {
    pub channel: u8,
    pub data: Vec<u8>
}

impl Packet {
    pub fn new(channel: u8) -> Packet {
        Packet {
            channel: channel,
            data: Vec::new()
        }
    }
}

pub struct Event {
    pub transformer: Box<dyn Fn(&mut Packet)>,
    pub connections: Vec<Box<dyn Fn(&Packet)>>,
    pub channels: Vec<(u8, Box<dyn Fn(&Packet, &mut Packet)>)>
}


impl Event {
    pub fn new() -> Event {
        Event {
            transformer: Box::from(|_: &mut _| {}),
            connections: Vec::new(),
            channels: Vec::new()
        }
    }

    pub fn add_channel(&mut self, channel: u8, closure: &'static dyn Fn(&Packet, &mut Packet)) {
        self.channels.push((channel, Box::from(closure)));
    }   

    pub fn add_listener(&mut self, closure: &'static dyn Fn(&Packet)) {
        self.connections.push(Box::from(closure));
    }

    pub fn set_transformer(&mut self, closure: &'static dyn Fn(&mut Packet) -> ()) {
        self.transformer = Box::from(closure);
    }

    pub fn fire_event(&mut self, packet: &mut Packet) -> Packet {
        let mut response_packet = Packet::new(packet.channel);
        let mut had_channel = false;

        self.transformer.as_ref()(packet);

        for (channel, listener) in &self.channels {
            if channel == &packet.channel {
                listener.as_ref()(packet, &mut response_packet);
                had_channel = true;
            }
        }

        if !had_channel {
            for listener in &self.connections {
                listener.as_ref()(packet);
            }
        }

        return response_packet;
    }
}