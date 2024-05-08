use crate::models;
use sqlx::SqlitePool;


// pub async fn insert_client_info(
//     &pool: SqlitePool,
//     client_info: &models::ClientInfo,
// ) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
//         INSERT INTO client_info (client_id, name, token)
//         VALUES (?, ?, ?)
//         "#,
//         client_info.client_id,
//         client_info.name,
//         client_info.token,
//     )
//     .execute(&pool)
//     .await?;
//     Ok(())
// }
