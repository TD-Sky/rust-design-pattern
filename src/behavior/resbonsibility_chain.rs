pub trait Handler<'a> {
    fn handle(&self, req: &str);
    fn append(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
}

pub struct First<'a> {
    name: String,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for First<'a> {
    fn handle(&self, req: &str) {
        println!("{} is handling req {}", self.name, req);

        if let Some(next) = self.next {
            next.handle(req);
        }
    }

    fn append(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
}

pub struct Second<'a> {
    address: String,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for Second<'a> {
    fn handle(&self, req: &str) {
        println!("The req {} has been sent to {}", req, self.address);

        if let Some(next) = self.next {
            next.handle(req);
        }
    }

    fn append(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
}

#[allow(dead_code)]
impl<'a> First<'a> {
    pub fn new() -> Self {
        Self {
            name: "first server".to_string(),
            next: None,
        }
    }
}

#[allow(dead_code)]
impl<'a> Second<'a> {
    pub fn new() -> Self {
        Self {
            address: "second server".to_string(),
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Handler;
    use super::{First, Second};

    #[test]
    fn pseudo_client() {
        let mut first = First::new();
        let second = Second::new();

        first.append(&second);

        first.handle("Mayday! Mayday!");
    }
}
