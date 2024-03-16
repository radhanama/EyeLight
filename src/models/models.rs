pub trait ImageAnalyzer {
    fn analyze_image(&self, image_bytes: &[u8]) -> Result<ImageAnalysisResult, AnalyzeError>;
}

#[derive(Debug)]
pub struct ImageAnalysisResult {
    pub labels: Vec<String>,
    // You can add other relevant analysis information here (e.g., bounding boxes)
}

#[derive(Debug)]
pub enum AnalyzeError {
    RekognitionError(aws_sdk_rekognition::Error),
    InternalError(String),
}

