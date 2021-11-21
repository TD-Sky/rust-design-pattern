pub trait Transport {
    fn act(&self);
}

#[allow(dead_code)]
pub enum TransportSort {
    Car,
    Plane,
    Ship,
}

struct Car;

impl Transport for Car {
    fn act(&self) {
        println!("rush! rush! rush!");
    }
}

struct Plane;

impl Transport for Plane {
    fn act(&self) {
        println!("up! up! up!");
    }
}

struct Ship;

impl Transport for Ship {
    fn act(&self) {
        println!("swim! swim! swim!");
    }
}

pub struct TransportFactory;

#[allow(dead_code)]
impl TransportFactory {
    pub fn make(&self, sort: TransportSort) -> Box<dyn Transport> {
        match sort {
            TransportSort::Car => Box::new(Car),
            TransportSort::Plane => Box::new(Plane),
            TransportSort::Ship => Box::new(Ship),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TransportFactory, TransportSort};

    #[test]
    fn main_test() {
        let factory = TransportFactory;
        let transports = (
            factory.make(TransportSort::Car),
            factory.make(TransportSort::Plane),
            factory.make(TransportSort::Ship),
        );

        transports.0.act();
        transports.1.act();
        transports.2.act();
    }
}

