pub fn scripts() -> String {
    r#"
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
    "#.to_string()
}
