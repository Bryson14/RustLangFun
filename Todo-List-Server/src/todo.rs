/// timestamp stored as rfc3339
struct date {
    timestamp: String,
    date_only: bool,
}

struct todo {
    msg: String,
    due_date: Option<date>,
    completed: bool,
}
