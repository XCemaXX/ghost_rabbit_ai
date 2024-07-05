use std::collections::HashMap;
use super::Difficulty;

const RECORDS_COUNT: usize = 5;

pub struct RecordTable {
    pub names: [String; RECORDS_COUNT],
    pub scores: [usize; RECORDS_COUNT],
}

impl RecordTable {
    pub fn insert(&mut self, name: String, score: usize) {
        let mut v: Vec<(usize, String)> = std::iter::zip(self.scores, self.names.clone()).collect();
        v.push((score, name));
        v.sort_by_key(|x| x.0);
        for i in 0..RECORDS_COUNT {
            self.names[i] = v[i+1].1.to_string();
            self.scores[i] = v[i+1].0;
        };
    }
}

pub type RecordTables = HashMap<Difficulty, RecordTable>;

pub fn get_default_records() -> RecordTables {
    let mut res = HashMap::new();
    res.insert(Difficulty::Normal, RecordTable {
        names: ["Peace".into(), "100TOHH".into(), "Pillow".into(), "ApTinyPle".into(), "God".into()],
        scores: [200, 1500, 3000, 10000, 25000],
    });
    res.insert(Difficulty::Unreal, RecordTable {
        names: ["Anton".into(), "Slava".into(), "Renat".into(), "Daniilka".into(), "God".into()],
        scores: [500, 1000, 5000, 9000, 25000],
    });
    res.insert(Difficulty::Practice, RecordTable {
        names: ["Practice".into(), "Practice".into(), "Practice".into(), "Practice".into(), "Practice".into()],
        scores: [500, 1000, 5000, 9000, 25000],
    });
    res
}