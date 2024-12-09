#[derive(Debug, Clone, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(username: &str, email: &str, password_hash: &str) -> Self {
        Self {
            id: 0, // IDはデータベースで生成されるため0で初期化
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

pub trait UserRepository {
    fn insert(&self, user: &User) -> Result<(), String>;
    fn update(&self, user: &User) -> Result<(), String>;
    fn delete(&self, user_id: i32) -> Result<(), String>;
}
