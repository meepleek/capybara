use bevy::prelude::*;

pub trait MessageReaderExt<T> {
    fn read_only_last(&mut self) -> Option<&T>;
    fn clear_any(&mut self) -> bool;
}

impl<'w, 's, T: Message> MessageReaderExt<T> for MessageReader<'w, 's, T> {
    fn read_only_last(&mut self) -> Option<&T> {
        let mut res = None;
        for ev in self.read() {
            res = Some(ev)
        }
        res
    }

    fn clear_any(&mut self) -> bool {
        if !self.is_empty() {
            self.clear();
            true
        } else {
            false
        }
    }
}
