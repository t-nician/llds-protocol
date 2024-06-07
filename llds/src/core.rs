pub struct Packet {}
pub struct Channel(u8, Box<dyn Fn(&Packet, &mut Packet) -> ()>);

pub struct Event {
    on_channel: Vec<Channel>,

    on_packet: Box<dyn Fn(&Packet, &mut Packet) -> ()>,
    on_transform: Box<dyn Fn(&mut Packet) -> ()>,
}

impl Event {
    pub fn new() -> Self {
        Self {
            on_packet: Box::from(|_ : &_, _: &mut _|{}),
            on_channel: Vec::new(),//Box::from(|_ : &_, _: &mut _|{}),
            on_transform: Box::from(|_: &mut _|{})
        }
    }

    pub fn on_packet(&mut self, closure: &'static dyn Fn(&Packet, &mut Packet)) {
        self.on_packet = Box::from(closure);
    }

    pub fn on_channel(&mut self, channel: u8, closure: &'static dyn Fn(&Packet, &mut Packet)) {
        self.on_channel.push(Channel(channel, Box::from(closure)));
    }

    pub fn on_transform(&mut self, closure: &'static dyn Fn(&mut Packet)) {
        self.on_transform = Box::from(closure);
    }


}