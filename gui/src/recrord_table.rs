use std::collections::HashMap;
use quad_storage::LocalStorage;

use super::Difficulty;

const RECORDS_COUNT: usize = 5;
const DIFFICULTIES: [Difficulty; 2] = [Difficulty::Normal, Difficulty::Unreal];

pub struct RecordTable {
    pub names: [String; RECORDS_COUNT],
    pub scores: [usize; RECORDS_COUNT],
}

impl RecordTable {
    fn insert(&mut self, name: String, score: usize) {
        let mut v: Vec<(usize, String)> = std::iter::zip(self.scores, self.names.clone()).collect();
        v.push((score, name));
        v.sort_by_key(|x| x.0);
        for i in 0..RECORDS_COUNT {
            self.names[i] = v[i + 1].1.to_string();
            self.scores[i] = v[i + 1].0;
        };
    }

    fn new(difficulty: &Difficulty) -> Self {
        match difficulty {
            Difficulty::Normal => {
                RecordTable {
                    names: ["Peace".into(), "100TOHH".into(), "Pillow".into(), "ApTinyPle".into(), "God".into()],
                    scores: [200, 1000, 2000, 5000, 25000],
                }
            },
            Difficulty::Practice => {
                RecordTable {
                    names: ["Practice".into(), "Practice".into(), "Practice".into(), "Practice".into(), "Practice".into()],
                    scores: [500, 1000, 5000, 9000, 25000],
                }
            },
            Difficulty::Unreal => {
                RecordTable {
                    names: ["Anton".into(), "Slava".into(), "Renat".into(), "Daniilka".into(), "God".into()],
                    scores: [200, 500, 1000, 3000, 5000],
                }
            },
        }
    }
}

pub struct RecordTables(HashMap<Difficulty, RecordTable>);

impl RecordTables {
    pub fn new() -> Self {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        let mut res = HashMap::new();
        for d in DIFFICULTIES {
            let mut rt = RecordTable::new(&d);
            for i in 0..RECORDS_COUNT {
                if let Some(nickname) = storage.get(format!("{}_nickname_{}", d.as_str(), i).as_str()) {
                    rt.names[i] = nickname;
                };
                if let Some(score) = storage.get(format!("{}_score_{}", d.as_str(), i).as_str()) {
                    rt.scores[i] = score.parse().unwrap();
                }
            }
            res.insert(d, rt);
        }
        res.insert(Difficulty::Practice, RecordTable::new(&Difficulty::Practice));
        Self(res)
    }

    pub fn set_default(&mut self) {
        let mut res = HashMap::new();
        res.insert(Difficulty::Normal, RecordTable::new(&Difficulty::Normal));
        res.insert(Difficulty::Unreal, RecordTable::new(&Difficulty::Unreal));
        res.insert(Difficulty::Practice, RecordTable::new(&Difficulty::Practice));
        self.0 = res;

        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        for difficulty in &DIFFICULTIES {
            self.save_to_storage(difficulty, storage);
        }
    }

    pub fn get_table(&self, difficulty: &Difficulty) -> &RecordTable {
        self.0.get(&difficulty).unwrap()
    }
    
    pub fn insert(&mut self, difficulty: &Difficulty, name: String, score: usize) {
        if difficulty != &Difficulty::Unreal && difficulty != &Difficulty::Normal {
            return;
        }
        self.0.get_mut(difficulty).unwrap().insert(name, score);

        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        self.save_to_storage(difficulty, storage);
    }

    fn save_to_storage(&self, difficulty: &Difficulty, storage: &mut std::sync::MutexGuard<LocalStorage>) {
        let rt = self.0.get(difficulty).unwrap();
        for i in 0..RECORDS_COUNT {
            storage.set(format!("{}_nickname_{}", difficulty.as_str(), i).as_str(), &rt.names[i]);
            storage.set(format!("{}_score_{}", difficulty.as_str(), i).as_str(), &rt.scores[i].to_string());
        }
    }
}