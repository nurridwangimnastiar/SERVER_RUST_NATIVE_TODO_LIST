use crate::views::escape_html;

pub fn todo_item(id: i64, task: &str, completed: bool) -> String {
    let completed_class = if completed { "completed" } else { "" };
    let safe_task = escape_html(task);

format!(
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
        completed_class, safe_task, id, id, id, safe_task
    )
}
