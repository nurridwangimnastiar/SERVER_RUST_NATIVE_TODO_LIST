use sqlite::Connection;
use sqlite::State;
use crate::views::components::{header, todo_item, scripts, styles};

pub fn render_todo_list(db: &Connection) -> String {
    let mut html = String::new();
    html.push_str(&header());
    html.push_str(&styles());
    html.push_str(
        r#"
    <body>
        <div class="container">
            <h1>Daftar Todo</h1>
            <form method="POST" action="/add">
                <label for="task">Tambah Tugas:</label>
                <input type="text" id="task" name="task" required>
                <button type="submit">Tambah</button>
            </form>
            <ul>"#
    );

    let mut statement = db.prepare("SELECT id, task, completed FROM todos").unwrap();
    while let Ok(State::Row) = statement.next() {
        let id: i64 = statement.read(0).unwrap();
        let task: String = statement.read(1).unwrap();
        let completed: i64 = statement.read(2).unwrap();
        html.push_str(&todo_item(id, &task, completed == 1));
    }

    html.push_str(
        r#"</ul>
        </div>"#
    );

    html.push_str(&delete_confirmation_modal());
    html.push_str(&delete_form());
    html.push_str(&scripts());
    html.push_str(
        r#"
    </body>
    </html>"#
    );

    html
}

fn delete_confirmation_modal() -> String {
    r#"
    <!-- Modal Konfirmasi Hapus -->
    <div class="modal" id="deleteConfirmationModal">
        <div class="modal-content">
            <h3>Anda yakin ingin menghapus tugas ini?</h3>
            <div class="modal-buttons">
                <button class="delete" onclick="deleteTask()">Hapus Permanen</button>
                <button class="cancel" onclick="hideDeleteConfirmation()">Batal</button>
            </div>
        </div>
    </div>
    "#.to_string()
}

fn delete_form() -> String {
    r#"
    <form method="POST" action="/delete" id="deleteForm">
        <input type="hidden" name="id" id="deleteId" value="">
    </form>
    "#.to_string()
}
