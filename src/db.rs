// src/db.rs
extern crate sqlite;

use sqlite::{Connection, Error};

pub fn init_db() -> Result<Connection, Error> {
    let connection = sqlite::open("todolist.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0
        )",
    )?;

    Ok(connection)
}

pub fn add_todo(db: &Connection, task: &str) {
    let query = format!("INSERT INTO todos (task) VALUES ('{}')", task);
    db.execute(&query).unwrap();
}

pub fn complete_todo(db: &Connection, id: i64) {
    let query = format!("UPDATE todos SET completed = 1 WHERE id = {}", id);
    db.execute(&query).unwrap();
}

pub fn delete_todo(db: &Connection, id: i64) {
    let query = format!("DELETE FROM todos WHERE id = {}", id);
    db.execute(&query).unwrap();
}

pub fn edit_todo(db: &Connection, id: i64, task: &str) {
    let query = format!("UPDATE todos SET task = '{}' WHERE id = {}", task, id);
    db.execute(&query).unwrap();
}
