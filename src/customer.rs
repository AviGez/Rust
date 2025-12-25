#[derive(Clone, Debug)]
pub struct Customer {
    id: u32,
    name: String,
    age: u8,
}

impl Customer {
    pub fn new(id: u32, name: String, age: u8) -> Self {
        Self { id, name, age }
    }

    pub fn print(&self) {
        println!("Customer ID: {}, Name: {}, Age: {}", self.id, self.name, self.age);
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
