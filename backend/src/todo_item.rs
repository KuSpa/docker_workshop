use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, serde_derive::Serialize)]
pub struct TodoItem {
    pub id: i32,
    pub content: String,
}

impl From<(&i32, &String)> for TodoItem{
    fn from(value: (&i32, &String)) -> Self {
        Self{id:*value.0, content: value.1.clone()}
    }
}

pub static TODOS: Lazy<Arc<RwLock<HashMap<i32, String>>>> = Lazy::new(|| {
    let todos: HashMap<i32, String> = HashMap::new();
    Arc::new(RwLock::new(todos.clone()))
});