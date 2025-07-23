use crate::db::user::create_user;
use crate::utils::password::generate_random_password;
use crate::utils::hash::hash_password;

use crate::models::user::NewUser;

pub async fn init_admin(pool: &sqlx::SqlitePool) {
    let exists = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE username = 'admin'")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if exists == 0 {
        let password = generate_random_password(8);
        let password_hash = hash_password(&password);

        let new_admin = NewUser {
            username: "admin".into(),
            password_hash,              // ← ХЕШ ПАРОЛЯ, а не просто пароль
            role: "admin".into(),
        };

        match create_user(pool, new_admin).await {
            Ok(_) => {
                println!("\n--- Hostnexus Admin Created ---");
                println!("Username: admin");
                println!("Password: {password}");
                println!("-------------------------------\n");
            }
            Err(e) => eprintln!("Failed to create admin user: {:?}", e),
        }
    }
}
