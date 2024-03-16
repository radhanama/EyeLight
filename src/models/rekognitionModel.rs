use aws_config::from_env;
use aws_sdk_rekognition::{Client, config::Region, model::DetectLabelsRequest};

pub struct RekognitionModel {
    client: Client,
}

impl RekognitionModel {
    pub fn new() -> Result<RekognitionModel, aws_config::ConfigError> {
        let config = from_env().region(Region::default())?;
        Ok(RekognitionModel { client: Client::new(&config) })
    }
}

impl ImageAnalyzer for RekognitionModel {
    fn analyze_image(&self, image_bytes: &[u8]) -> Result<ImageAnalysisResult, AnalyzeError> {
        let request = DetectLabelsRequest::builder()
            .image(Some(image_bytes.to_owned()))
            .build()?;

        let response = self.client.detect_labels(request).await?;
        let mut labels = Vec::new();
        for label in response.labels {
            labels.push(label.name.unwrap_or_default().to_owned());
        }

        Ok(ImageAnalysisResult { labels })
    }
}

