use std::io::{Read, Write};
use std::net::TcpStream;
use sqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::views::{render_todo_list, not_found_page};
use crate::models::{parse_task, parse_id};
use crate::db::{add_todo, complete_todo, delete_todo, edit_todo};

pub fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<Connection>>) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let db = db.lock().unwrap();

    if request.starts_with("GET /") {
        let response = render_todo_list(&db);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if request.starts_with("POST /add") {
        let task = parse_task(&request);
        add_todo(&db, &task);
        let response = render_todo_list(&db);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if request.starts_with("POST /complete") {
        let id = parse_id(&request);
        complete_todo(&db, id);
        let response = render_todo_list(&db);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if request.starts_with("POST /delete") {
        let id = parse_id(&request);
        delete_todo(&db, id);
        let response = render_todo_list(&db);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if request.starts_with("POST /edit") {
        let id = parse_id(&request);
        let task = parse_task(&request);
        edit_todo(&db, id, &task);
        let response = render_todo_list(&db);
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = not_found_page();
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
