use serde::Serialize;

#[derive(Serialize)]
pub struct Photo {
    pub id: i32,
    pub data: Vec<u8>,
    pub analysis_result: String,
}

impl Photo {
    pub fn new(data: bytes::Bytes, analysis_result: String) -> Self {
        Photo {
            id: 0,
            data: data.to_vec(),
            analysis_result,
        }
    }
}
