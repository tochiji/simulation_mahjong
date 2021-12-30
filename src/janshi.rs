#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Janshi {
    id: usize,
    name: String,
    participation_degree: u64, // 参加度。値が大きい方が参加しやすい。
    point: i64,
    play_count: usize,
}

impl Janshi {
    pub fn new(id: usize, name: &str, participation_degree: u64) -> Self {
        let name = name.to_string();
        Janshi {
            id,
            name,
            participation_degree,
            point: 0,
            play_count: 0,
        }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_participation_degree(&self) -> u64 {
        self.participation_degree
    }
    pub fn get_point(&self) -> i64 {
        self.point
    }
    pub fn get_play_count(&self) -> usize {
        self.play_count
    }
    pub fn play_hanchan(&mut self, point: i64) {
        self.point += point;
        self.play_count += 1;
    }
    pub fn reset_year(&mut self) {
        self.point = 0;
        self.play_count = 0;
    }
}
