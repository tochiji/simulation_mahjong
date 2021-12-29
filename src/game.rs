use std::collections::HashSet;

use rand::Rng;

use super::*;

enum GameKind {
    Yonma,
}

pub struct Game {
    janshis: Vec<Janshi>,
    kind: GameKind,
}

impl Game {
    pub fn new(id_names: Vec<(usize, &str)>) -> Self {
        let janshis: Vec<Janshi> = id_names
            .iter()
            .copied()
            .map(|v| Janshi::new(v.0, v.1))
            .collect();

        Game {
            janshis,
            kind: GameKind::Yonma,
        }
    }
    fn janshi_len(&self) -> usize {
        self.janshis.len()
    }
    fn get_random_player_id(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.janshi_len())
    }
    fn get_random_player_ids(&self) -> Vec<usize> {
        let size = match self.kind {
            GameKind::Yonma => 4,
        };

        let mut player_ids: HashSet<usize> = HashSet::new();

        while player_ids.len() < size {
            let roll = self.get_random_player_id();
            player_ids.insert(roll);
        }

        player_ids.iter().cloned().collect()
    }
    fn play_hanchan(&mut self) {
        let points = [60, 15, -15, -60];
        let player_ids = self.get_random_player_ids();

        player_ids.iter().copied().enumerate().for_each(|(i, v)| {
            self.janshis[v].add_point(points[i]);
        });
    }
    pub fn play_hanchans(&mut self, count: usize) {
        for _ in 0..count {
            self.play_hanchan();
        }
    }
    pub fn print_points(&mut self) {
        let janshis = &mut self.janshis;
        janshis.sort_by_key(|v| v.get_point());
        janshis.reverse();
        let result: Vec<String> = janshis
            .iter()
            .map(|j| format!("{}:{:5}", j.get_name(), j.get_point()))
            .collect();
        println!("{:#?}", result);
    }
    pub fn reset_points(&mut self) {
        self.janshis.iter_mut().for_each(|p| {
            p.reset_point();
        })
    }
    pub fn get_max_point(&self) -> i64 {
        self.janshis
            .iter()
            .max_by_key(|&v| v.get_point())
            .unwrap()
            .get_point()
    }
}
