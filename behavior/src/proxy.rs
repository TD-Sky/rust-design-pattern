struct Subject;

#[allow(dead_code)]
struct Proxy {
    inner: Subject,
}

impl Subject {
    fn request(&self) {
        println!("sending the request");
    }
}

impl Proxy {
    fn check_auth(&self) -> bool {
        println!("checking");
        true
    }

    fn log(&self) {
        println!("write log");
    }
}

#[allow(dead_code)]
impl Proxy {
    pub fn new() -> Self {
        Self { inner: Subject }
    }

    pub fn request(&self) {
        if self.check_auth() {
            self.inner.request();
            self.log();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Proxy;

    #[test]
    fn main_test() {
        let subject = Proxy::new();
        subject.request();
    }
}
