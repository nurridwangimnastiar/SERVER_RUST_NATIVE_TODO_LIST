pub fn not_found_page() -> String {
    let html = r#"HTTP/1.1 404 Not Found
Content-Type: text/html; charset=utf-8

<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 Tidak Ditemukan</title>
</head>
<body>
    <h1>404 Tidak Ditemukan</h1>
    <p>Halaman yang Anda cari tidak ditemukan.</p>
</body>
</html>"#;

    html.to_string()
}
