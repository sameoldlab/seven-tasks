use std::sync::atomic::AtomicI64;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
}

static ID: AtomicI64 = AtomicI64::new(0);

impl Entry {
    pub fn new(firstname: String, lastname: String) -> Self {
        let id = ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id,
            firstname,
            lastname,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.lastname, self.firstname)
    }
}

pub struct Crud {
    items: Vec<Entry>,
    firstname_: String,
    lastname_: String,
    filter: String,
    selected: usize,
}

impl Default for Crud {
    fn default() -> Self {
        Self {
            items: vec![
                Entry::new("Hans".to_string(), "Emil".to_string()),
                Entry::new("Max".to_string(), "Musterman".to_string()),
                Entry::new("Roman".to_string(), "Tisch".to_string()),
            ],
            firstname_: String::with_capacity(20),
            lastname_: String::with_capacity(20),
            filter: String::with_capacity(20),
            selected: 0,
        }
    }
}

impl Crud {
    fn next(&mut self) {
        self.selected = (self.selected + 1).max(self.items.len() - 1)
    }
    fn prev(&mut self) {
        self.selected = (self.selected - 1).min(0)
    }
    fn create(&mut self, entry: Entry) {
        self.items.push(entry)
    }
    fn update(&mut self) {
        if !self.firstname_.is_empty() {
            self.items[self.selected].firstname = self.firstname_.clone();
        }
        if !self.lastname_.is_empty() {
            self.items[self.selected].lastname = self.lastname_.clone();
        }
    }
    fn delete(&mut self) {
        self.items.swap_remove(self.selected);
    }
}
