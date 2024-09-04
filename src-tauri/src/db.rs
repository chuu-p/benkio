use std::error::Error;

use log::warn;
use serde::{Deserialize, Serialize};

pub struct FlashcardDB {
    db_path: String,
}

impl Default for FlashcardDB {
    fn default() -> FlashcardDB {
        let db_path = match std::env::consts::OS {
            "android" => "/data/user/0/com.tauri.loglife/db/sled.db",
            _ => "/tmp/sled.db",
        };
        FlashcardDB {
            db_path: db_path.to_string(),
        }
    }
}

impl FlashcardDB {
    pub fn _get_db(&self) -> Result<sled::Db, sled::Error> {
        sled::open(&self.db_path)
    }

    pub fn insert<T>(&self, key: &str, entry: T) -> Result<(), sled::Error>
    where
        T: Serialize,
    {
        let db = &self._get_db()?;
        let value = serde_json::to_string(&entry).unwrap();
        warn!("insert value: {}", &value);
        let _ = &db.insert(key, &value[..])?;
        db.flush()?;
        Ok(())
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let db = &self._get_db()?;
        let v = &db.get(key)?.unwrap();
        let val = std::str::from_utf8(v)?;
        let record: T = serde_json::from_str(val)?;
        Ok(record)
    }

    pub fn id(&self) -> u64 {
        let db = &self._get_db().unwrap();
        loop {
            match db.generate_id() {
                Ok(id) => return id,
                Err(error) => warn!("Failed to generate ID: {}. Retrying...", error),
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::*;
    use crate::flashcard::Flashcard;

    const DB_PATH: &str = "/tmp/test_sled.db";

    #[test]
    fn clear_db() {
        let _ = fs::remove_dir_all(DB_PATH).unwrap();
    }

    #[test]
    fn test_import_smoke() -> Result<(), String> {
        // arrange
        clear_db();
        let db = FlashcardDB {
            db_path: DB_PATH.to_string(),
        };
        let expected = Flashcard::default();
        let key = &db.id().to_string();

        // act
        let expected_str = serde_json::to_string(&expected).map_err(|e| e.to_string())?;
        db.insert(&key, expected).map_err(|e| e.to_string())?;
        let actual: Flashcard = db.get(&key).map_err(|e| e.to_string())?;
        let actual_str = serde_json::to_string(&actual).map_err(|e| e.to_string())?;

        // assert
        assert_eq!(expected_str, actual_str);
        Ok(())
    }

    #[test]
    fn test_import_kana() -> Result<(), String> {
        // arrange
        #[derive(Serialize, Deserialize, Clone)]
        struct Kana {
            char_id: String,
            character: String,
            romanization: String,
        }
        let hiragana_data =
            fs::read_to_string("./resources/kana/hiragana.json").expect("Unable to read file");
        let katakana_data =
            fs::read_to_string("./resources/kana/katakana.json").expect("Unable to read file");
        let hiragana: Vec<Kana> = serde_json::from_str(&hiragana_data).expect("Unable to parse");
        let katakana: Vec<Kana> = serde_json::from_str(&katakana_data).expect("Unable to parse");
        let kanas = [hiragana, katakana].concat();

        clear_db();
        let db = FlashcardDB {
            db_path: DB_PATH.to_string(),
        };

        // act
        for kana in kanas.iter() {
            let key = &db.id().to_string();
            db.insert(&key, kana).map_err(|e| e.to_string())?;
        }

        // assert
        assert_eq!(220usize, db._get_db().unwrap().iter().count());
        Ok(())
    }
}
