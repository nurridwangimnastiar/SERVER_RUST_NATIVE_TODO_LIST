use std::net::TcpListener;
use std::thread;
use std::sync::{Arc, Mutex};

mod db;
mod handlers;
mod models;
mod views;

use db::init_db;
use handlers::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server mendengarkan di localhost:8000");

    let db = init_db().expect("Gagal inisialisasi database");
    let db = Arc::new(Mutex::new(db));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&db);
                thread::spawn(move || {
                    handle_connection(stream, db_clone);
                });
            }
            Err(e) => {
                eprintln!("Koneksi gagal: {}", e);
            }
        }
    }
}
