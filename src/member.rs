#[derive(Clone, Debug)]
pub struct Member {
    id: u32,
    name: String,
    is_active: bool,
}

impl Member {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn print(&self) {
        let status = if self.is_active { "Active" } else { "Inactive" };
        println!("Member ID: {}, Name: {}, Status: {}", self.id, self.name, status);
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
