use urlencoding::decode;

pub fn escape_html(input: &str) -> String {
    let decoded = decode(input).unwrap_or_else(|_| input.to_string().into()).replace('+', " ");
    decoded.replace("&", "&amp;")
           .replace("<", "&lt;")
           .replace(">", "&gt;")
           .replace("\"", "&quot;")
           .replace("'", "&#39;")
}