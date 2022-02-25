#[derive(Debug)]
pub struct Subject<'a> {
    observers: Vec<&'a Observer>,
}

#[derive(Debug, PartialEq)]
pub struct Observer(u32);

impl Observer {
    fn update(&self, msg: &str) {
        println!("{:?} got the message: {}", self, msg);
    }
}

#[allow(dead_code)]
impl<'a> Subject<'a> {
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    pub fn add(&mut self, obs: &'a Observer) {
        self.observers.push(obs);
        self.notify("add");
    }

    pub fn remove(&mut self, obs: &'a Observer) {
        self.observers.drain_filter(|&mut kick| kick == obs);
        self.notify("remove");
    }

    pub fn notify(&self, msg: &str) {
        self.observers.iter().for_each(|&n| n.update(msg));
    }
}

#[cfg(test)]
mod tests {
    use super::{Observer, Subject};

    #[test]
    fn main_test() {
        let observers = [Observer(1), Observer(2), Observer(3)];
        let mut subject = Subject::new();

        (0..3).for_each(|i| subject.add(&observers[i]));
        subject.remove(&observers[1]);
    }
}
