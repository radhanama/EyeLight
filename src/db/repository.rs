use crate::utils::errors::PhotoError;
use sqlx::{Pool, Postgres};
use super::Photo;

pub async fn insert(photo: Photo, db_pool: &Pool<Postgres>) -> Result<Photo, PhotoError> {
    let mut tx = db_pool.begin().await?;

    let photo_id: i32 = sqlx::query_scalar!(
        r#"INSERT INTO photos (data, analysis_result) VALUES ($1, $2) RETURNING id"#,
        photo.data,
        photo.analysis_result
    )
    .fetch_one(&mut *tx)
    .await?;

    let inserted_photo = Photo {
        id: photo_id as i32,
        ..photo
    };

    tx.commit().await?;
    Ok(inserted_photo)
}
