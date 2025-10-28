use std::sync::atomic::AtomicI64;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub items: Vec<Entry>,
    pub firstname_: String,
    pub lastname_: String,
    pub filter: String,
    pub selected: usize,
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
    pub fn create(&mut self) {
        self.items.push(Entry::new(
            self.firstname_.clone(),
            self.lastname_.clone(),
        ));
        self.firstname_.clear();
        self.lastname_.clear();
    }
    pub fn update(&mut self) {
        if !self.firstname_.is_empty() {
            self.items[self.selected].firstname.clone_from(&self.firstname_);
        }
        if !self.lastname_.is_empty() {
            self.items[self.selected].lastname.clone_from(&self.lastname_);
        }
        self.firstname_.clear();
        self.lastname_.clear();
    }
    pub fn delete(&mut self) {
        self.items.swap_remove(self.selected);
    }
}
