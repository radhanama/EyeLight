use mime::Mime;

pub fn validate_image_format(data: &[u8]) -> Result<(), PhotoError> {
    let mime_type = mime::from_slice(data)?;
    if !mime_type.is_nontext() || !mime_type.is_image() {
        Err(PhotoError::Validation("Invalid image format".to_string()))
    } else {
        Ok(())
    }
}
