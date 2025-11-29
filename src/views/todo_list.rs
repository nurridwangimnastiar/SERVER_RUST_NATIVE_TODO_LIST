use sqlite::{Connection, State};

pub fn render_todo_list(db: &Connection) -> String {
    let mut html = String::from(
        r#"HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Daftar Todo</title>
    <style>
        body {
            font-family: 'Arial', sans-serif;
            background-color: #e6f9ff; /* Light Cyan-Blue */
            margin: 0;
            padding: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
        }
        .container {
            background-color: #fff;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            padding: 20px;
            width: 80%;
            max-width: 600px;
        }
        h1 {
            color: #3498db; /* Sky Blue */
            text-align: center;
            word-break: break-word; /* Memastikan teks panjang pecah ke baris baru */
        }
        form {
            display: flex;
            flex-direction: column;
            margin-bottom: 20px;
        }
        label {
            margin-bottom: 5px;
            font-weight: bold;
        }
        input[type="text"],
        button {
            padding: 10px;
            margin-bottom: 10px;
            border-radius: 5px;
            border: 1px solid #ddd;
            font-size: 14px; /* Ukuran font yang lebih kecil untuk tombol */
        }
        button {
            background-color: #3498db; /* Sky Blue */
            color: white;
            border: none;
            cursor: pointer;
            flex: 1; /* Membuat tombol mengisi ruang yang tersedia */
            margin: 2px; /* Jarak antar tombol */
        }
        button:hover {
            background-color: #2980b9;
        }
        ul {
            list-style: none;
            padding: 0;
        }
        li {
            padding: 10px;
            border-bottom: 1px solid #eee;
            display: flex;
            justify-content: space-between;
            align-items: center;
            word-break: break-word; /* Memastikan teks panjang pecah ke baris baru */
        }
        li:last-child {
            border-bottom: none;
        }
        .completed {
            text-decoration: line-through;
            color: #aaa;
        }
        .actions {
            display: flex;
            gap: 5px; /* Jarak antar tombol */
        }
        .actions button {
            margin-left: 0;
            background-color: #e74c3c; /* Coral */
            padding: 5px 10px; /* Padding lebih kecil */
            font-size: 12px; /* Ukuran font lebih kecil */
            line-height: 0.8; /* Mengurangi tinggi baris agar simbol terlihat pas */
        }
        .actions button:hover {
            background-color: #c0392b;
        }
        .actions button.complete-button {
            background-color: #2ecc71; /* Emerald */
        }
        .actions button.complete-button:hover {
            background-color: #27ae60;
        }
        /* Style untuk form edit */
        .edit-form {
            display: none; /* Sembunyikan form secara default */
            margin-top: 10px;
        }
        .editing .edit-form {
            display: flex; /* Tampilkan form saat dalam mode edit */
            flex-direction: column;
        }
        .editing span {
            display: none; /* Sembunyikan teks tugas saat dalam mode edit */
        }
        /* Style untuk modal */
        .modal {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.5);
            z-index: 1000;
        }
        .modal-content {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            background-color: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            text-align: center;
        }
        .modal-buttons {
            margin-top: 10px;
            display: flex;
            justify-content: center;
            gap: 10px;
        }
        .modal-buttons button {
            padding: 8px 16px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }
        .modal-buttons button.delete {
            background-color: #e74c3c;
            color: white;
        }
        .modal-buttons button.cancel {
            background-color: #3498db;
            color: white;
        }
    </style>
</head>
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

        let completed_class = if completed == 1 { "completed" } else { "" };

        html.push_str(&format!(
            r#"<li class="{}">
                <span>{}</span>
                <div class="actions">
                    <form method="POST" action="/complete">
                        <input type="hidden" name="id" value="{}">
                        <button type="submit" class="complete-button">✓</button>
                    </form>
                    <button onclick="showDeleteConfirmation(this.parentNode.parentNode, {})">×</button>
                    <button onclick="toggleEditForm(this.parentNode.parentNode)">✎</button>
                </div>
                <form class="edit-form" method="POST" action="/edit">
                    <input type="hidden" name="id" value="{}">
                    <label for="task">Edit Tugas:</label>
                    <input type="text" id="task" name="task" value="{}" required>
                    <button type="submit">Simpan</button>
                </form>
            </li>"#,
            completed_class, task, id, id, id, task
        ));
    }

    html.push_str(
        r#"</ul>
    </div>

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

    <form method="POST" action="/delete" id="deleteForm">
        <input type="hidden" name="id" id="deleteId" value="">
    </form>

    <script>
        let currentLi;

        function showDeleteConfirmation(li, id) {
            document.getElementById('deleteConfirmationModal').style.display = 'block';
            document.getElementById('deleteId').value = id;
            currentLi = li;
        }

        function hideDeleteConfirmation() {
            document.getElementById('deleteConfirmationModal').style.display = 'none';
        }

        function deleteTask() {
            document.getElementById('deleteForm').submit();
            hideDeleteConfirmation();
            currentLi.remove();
        }
        
        function toggleEditForm(li) {
            li.classList.toggle('editing');
        }
    </script>
</body>
</html>"#
    );

    html
}
