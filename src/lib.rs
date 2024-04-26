pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&mut self, key: String, value: String) {

    }

    pub fn get(&self, key: String) -> Option<String> {
        Some(String::from("test"))
        // Some(key)
    }

    pub fn remove(&mut self, key: String) {

    }
}