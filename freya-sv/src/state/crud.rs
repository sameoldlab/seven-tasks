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
            firstname: firstname,
            lastname: lastname,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.lastname, self.firstname)
    }
}

