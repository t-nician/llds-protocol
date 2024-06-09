use super::packet::*;

pub enum Emit {
    Get,
    Post
}

pub struct Event {
    getters: Vec<(u8, &'static dyn Fn(&Packet) -> Packet)>,
    posters: Vec<(u8, &'static dyn Fn(&Packet))>
}

impl Event {
    pub fn new() -> Event {
        Event {
            getters: Vec::new(),
            posters: Vec::new()
        }
    }

    fn get_getter_by_channel(&self, channel: &u8) -> Option<&&'static dyn Fn(&Packet) -> Packet> {
        for (getter_channel, closure) in self.getters.iter() {
            if getter_channel == channel {
                return Some(closure);
            }
        } 

        return None;
    }

    fn is_getter_channel_occupied(&self, channel: &u8) -> bool {
        for (getter_channel, _) in &self.getters {
            if getter_channel == channel {
                return true;
            }
        }

        return false;
    }

    pub fn get(&mut self, channel: u8, closure: &'static dyn Fn(&Packet) -> Packet) {
        if self.is_getter_channel_occupied(&channel) {
            panic!("Attempted to add a second 'get' event to channel {:?}!", channel);
        }

        self.getters.push((channel, closure));
    }

    pub fn post(&mut self, channel: u8, closure: &'static dyn Fn(&Packet)) {
        self.posters.push((channel, closure));
    }

    pub fn emit(&mut self, emit: Emit, channel: u8, packet: &mut Packet) -> Option<Packet> {        
        match emit {
            Emit::Post => {
                for (event_channel, closure) in &self.posters {
                    if event_channel == &channel {
                        closure(packet);
                    }
                }
            }
            
            Emit::Get => { 
                let result = self.get_getter_by_channel(&channel);

                if result.is_some() {
                    return Some(result.unwrap()(packet));
                }
            }
        }

        return None;
    }
}
