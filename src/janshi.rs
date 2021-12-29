#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Janshi {
    id: usize,
    name: String,
    point: i64,
}

impl Janshi {
    pub fn new(id: usize, name: &str) -> Self {
        let name = name.to_string();
        Janshi { id, name, point: 0 }
    }
    pub fn add_point(&mut self, add: i64) {
        self.point += add;
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_point(&self) -> i64 {
        self.point
    }
    pub fn reset_point(&mut self) {
        self.point = 0
    }
}
