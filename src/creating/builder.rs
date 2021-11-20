#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    salary: u64,
}

#[derive(Default)]
struct UserBuilder {
    id: u32,
    name: String,
    salary: u64,
}

#[allow(dead_code)]
impl User {
    fn builder(id: u32, name: String) -> UserBuilder {
        UserBuilder::new(id, name)
    }
}

#[allow(dead_code)]
impl UserBuilder {
    fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            salary: u64::default(),
        }
    }

    fn salary(mut self, salary: u64) -> Self {
        self.salary = salary;
        self
    }

    fn build(self) -> User {
        match self {
            Self { id, name, salary } => User { id, name, salary },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UserBuilder;

    #[test]
    fn main_test() {
        let user = UserBuilder::new(18, "yorina".to_string())
            .salary(50000)
            .build();

        println!("{:?}", user);
    }
}
