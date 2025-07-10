#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    id: i32,
    firstname: String,
    lastname: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.lastname, self.firstname)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entries {
    entries: Vec<Entry>,
    max_idx: i32,
}

impl Default for Entries {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            max_idx: Default::default(),
        }
    }
}

impl Entries {
    pub fn create(&mut self, firstname: &str, lastname: &str) {
        let id = self.max_idx + 1;
        self.max_idx = id;

        let entry = Entry {
            id,
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
        };
        self.entries.push(entry);
        dbg!(&self.entries);
    }

    pub fn list(self) -> Vec<Entry> {
        self.entries
    }

    pub fn update(&mut self, id: i32, firstname: &str, lastname: &str) {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries[pos].firstname = firstname.to_string();
            self.entries[pos].lastname = lastname.to_string();
        }
    }
    pub fn delete(&mut self, id: i32) {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries.swap_remove(pos);
        };
    }
}
