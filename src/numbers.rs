#[derive(Debug)]
pub struct Numbers {
    data: Vec<i32>,
}

impl Numbers {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn as_slice(&self) -> &[i32] {
        &self.data
    }

    // סכום האיברים באמצעות אינדקסים
    pub fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..self.data.len() {
            sum += self.data[i];
        }
        sum
    }

    // ערך מקסימלי ללא iterators וללא max()
    pub fn max(&self) -> i32 {
        let mut max = self.data[0];
        for i in 1..self.data.len() {
            if self.data[i] > max {
                max = self.data[i];
            }
        }
        max
    }
}
