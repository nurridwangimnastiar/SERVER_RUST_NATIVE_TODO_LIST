pub fn parse_task(request: &str) -> String {
    if let Some(body_start) = request.find("\r\n\r\n") {
        let body = &request[body_start + 4..];
        for param in body.split('&') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 && parts[0] == "task" {
                return parts[1].to_string();
            }
        }
    }
    "".to_string()
}

pub fn parse_id(request: &str) -> i64 {
    if let Some(body_start) = request.find("\r\n\r\n") {
        let body = &request[body_start + 4..];
        for param in body.split('&') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 && parts[0] == "id" {
                if let Ok(id) = parts[1].parse::<i64>() {
                    return id;
                }
            }
        }
    }
    0
}
