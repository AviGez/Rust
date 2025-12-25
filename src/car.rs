#[derive(Debug)]
pub struct Car {
    color: String,
    speed: u32,
}

impl Car {
    pub fn new(color: impl Into<String>, speed: u32) -> Self {
        Self {
            color: color.into(),
            speed,
        }
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn accelerate(&mut self, delta: u32) {
        self.speed += delta;
    }
}
