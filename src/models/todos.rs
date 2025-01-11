table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        done -> Bool,
    }
}

#[derive(Queryable, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}
