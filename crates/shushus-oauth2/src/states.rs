use std::sync::{ Arc, Mutex };
use uuid::Uuid;

pub struct States(Arc<Mutex<Vec<Uuid>>>);

impl States {
    pub fn create(&self) -> Uuid {
        let uuid = Uuid::new_v4();

        self.0.lock().unwrap().push(uuid);

        uuid
    }

    pub fn check(&self, uuid: &Uuid) -> bool {
        self.0.lock().unwrap().contains(uuid)
    }

    pub fn apply(&self, uuid: &Uuid) -> Option<Uuid> {
        let mut lock = self.0.lock().unwrap();
        let index = lock.iter().position(|x| x == uuid)?;

        Some(lock.remove(index))
    }
}

impl Default for States {
    fn default() -> Self {
        States(Arc::new(Mutex::new(Vec::new())))
    }
}
