use crate::domain::user::{User, UserRepository};

pub fn register_user<R: UserRepository>(repository: &R, name: &str, email: &str) -> Result<User, String> {
    let user = User {
        id: 0, // IDはDBで生成
        name: name.to_string(),
        email: email.to_string(),
    };

    repository.insert(&user)?;
    Ok(user)
}
