use super::*;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct LocalDatabase {
    items: HashMap<i32, Meme>,
    path: String,
}

impl LocalDatabase {
    pub fn new(path: String) -> Result<Self, DatabaseError> {
        if !Path::new(&path).exists() {
            let init_data: HashMap<i32, Meme> = HashMap::new();
            let serialized = serde_json::to_string(&init_data).map_err(|err| {
                DatabaseError::Other(format!("Failed to serialize data: {}", err))
            })?;

            fs::write(&path, serialized)
                .map_err(|err| DatabaseError::Other(format!("Failed to create file: {}", err)))?;
        }

        let file_data = fs::read_to_string(&path)
            .map_err(|err| DatabaseError::Other(format!("Failed to read file: {}", err)))?;
        let items: HashMap<i32, Meme> = serde_json::from_str(&file_data)
            .map_err(|err| DatabaseError::Other(format!("Failed to deserialize data: {}", err)))?;

        Ok(LocalDatabase { items, path })
    }

    fn write_to_db(&self) -> Result<(), DatabaseError> {
        let serialized = serde_json::to_string(&self.items)
            .map_err(|err| DatabaseError::Other(format!("Failed to serialize data: {}", err)))?;
        fs::write(&self.path, serialized).map_err(|err| DatabaseError::Other(format!("Failed to write to file: {}", err)))?;
        Ok(())
    }

    fn read_from_db(&mut self) -> Result<(), DatabaseError> {
        let file_data = fs::read_to_string(&self.path).map_err(|err| DatabaseError::Other(format!("Failed to read file: {}", err)))?;
        self.items = serde_json::from_str(&file_data).map_err(|err| DatabaseError::Other(format!("Failed to deserialize: {}", err)))?;
        Ok(())
    }
}

impl Database for LocalDatabase {
    fn create_item(&mut self, meme: Meme) -> Result<i32, DatabaseError> {
        let mut free_id = 1;
        while self.items.contains_key(&free_id) {
            free_id += 1;
        }
        self.items.insert(free_id, meme);
        self.write_to_db()?;
        Ok(free_id)
    }

    fn get_item(&self, id: i32) -> Result<Option<Meme>, DatabaseError> {
        match self.items.get(&id) {
            Some(item) => Ok(Some(item.clone())),
            None => Ok(None),
        }
    }

    fn update_item(&mut self, id: i32, meme: Meme) -> Result<(), DatabaseError> {
        match self.items.get_mut(&id) {
            Some(item) => {
                *item = meme;
                self.write_to_db()?;
                Ok(())
            },
            None => Err(DatabaseError::NotFound)
        }
    }

    fn remove_item(&mut self, id: i32) -> Result<Meme, DatabaseError> {
        match self.items.remove(&id) {
            Some(item) => {
                self.write_to_db()?;
                Ok(item)
            },
            None => Err(DatabaseError::NotFound)
        }
    }

    fn get_items(&self) -> Vec<i32> {
        let items = self.items.keys().cloned().collect();
        items
    }
}
