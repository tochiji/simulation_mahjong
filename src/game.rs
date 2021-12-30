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
    pub fn new(id_names: Vec<(usize, &str, u64)>) -> Self {
        let janshis: Vec<Janshi> = id_names
            .iter()
            .copied()
            .map(|v| Janshi::new(v.0, v.1, v.2))
            .collect();

        Game {
            janshis,
            kind: GameKind::Yonma,
        }
    }
    fn janshi_len(&self) -> usize {
        self.janshis.len()
    }
    fn get_participation_degree_sum(&self) -> u64 {
        self.janshis
            .iter()
            .map(|j| j.get_participation_degree())
            .sum()
    }
    fn get_random_player_id(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.janshi_len())
    }
    fn get_player_id_with_participation_degree(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(1..=self.get_participation_degree_sum()) as i64;

        self.janshis
            .iter()
            .find(|&v| {
                r -= v.get_participation_degree() as i64;
                if r <= 0 {
                    true
                } else {
                    false
                }
            })
            .unwrap()
            .get_id()
    }
    fn get_random_player_ids(&self) -> Vec<usize> {
        let size = match self.kind {
            GameKind::Yonma => 4,
        };

        let mut player_ids: HashSet<usize> = HashSet::new();

        while player_ids.len() < size {
            let roll = self.get_player_id_with_participation_degree();
            player_ids.insert(roll);
        }

        player_ids.iter().cloned().collect()
    }
    fn play_hanchan(&mut self) {
        let points = [60, 15, -15, -60];
        let player_ids = self.get_random_player_ids();

        player_ids.iter().copied().enumerate().for_each(|(i, v)| {
            self.janshis[v].play_hanchan(points[i]);
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
            .map(|j| {
                format!(
                    "{:7}:{:5}pt({}半荘)",
                    j.get_name(),
                    j.get_point(),
                    j.get_play_count()
                )
            })
            .collect();
        println!("{:#?}", result);
    }
    pub fn reset_points(&mut self) {
        self.janshis.iter_mut().for_each(|p| {
            p.reset_year();
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
