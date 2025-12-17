//! Type definitions for the [OpenRouter Provider API](https://openrouter.ai/docs/guides/for-providers).

use serde::{Deserialize, Serialize};

/// Response from the list models endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListModelsResponse {
    pub data: Vec<Model>,
}

/// A model available from the provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// e.g., "anthropic/claude-sonnet-4"
    pub id: String,
    pub name: String,
    /// Unix timestamp.
    pub created: i64,
    pub input_modalities: Vec<InputModality>,
    pub output_modalities: Vec<OutputModality>,
    pub quantization: Quantization,
    /// Max input tokens.
    pub context_length: u64,
    /// Max output tokens.
    pub max_output_length: u64,
    pub pricing: Pricing,
    pub supported_sampling_parameters: Vec<SamplingParameter>,
    pub supported_features: Vec<Feature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrouter: Option<OpenRouterInfo>,
    /// Required for Hugging Face models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugging_face_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<Datacenter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenRouterInfo {
    /// e.g., "anthropic/claude-sonnet-4"
    pub slug: String,
}

/// USD pricing as strings to prevent floating-point errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pricing {
    pub prompt: String,
    pub completion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cache_read: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cache_write: Option<String>,
}

impl Pricing {
    pub fn new(prompt: impl Into<String>, completion: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            completion: completion.into(),
            image: None,
            request: None,
            input_cache_read: None,
            input_cache_write: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InputModality {
    Text,
    File,
    Image,
    Audio,
    Video,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputModality {
    Text,
    Image,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Quantization {
    Int4,
    Int8,
    Fp4,
    Fp6,
    Fp8,
    Fp16,
    Bf16,
    Fp32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SamplingParameter {
    Temperature,
    TopP,
    TopK,
    RepetitionPenalty,
    FrequencyPenalty,
    PresencePenalty,
    Stop,
    Seed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    Tools,
    JsonMode,
    StructuredOutputs,
    WebSearch,
    Reasoning,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Datacenter {
    /// ISO 3166-1 alpha-2 code (e.g., "US", "DE").
    pub country_code: String,
}

impl Datacenter {
    pub fn new(country_code: impl Into<String>) -> Self {
        Self {
            country_code: country_code.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let original = ListModelsResponse {
            data: vec![Model {
                id: "org/model".to_string(),
                name: "Model".to_string(),
                created: 1700000000,
                input_modalities: vec![InputModality::Text, InputModality::Image],
                output_modalities: vec![OutputModality::Text],
                quantization: Quantization::Fp16,
                context_length: 128000,
                max_output_length: 4096,
                pricing: Pricing::new("0.001", "0.002"),
                supported_sampling_parameters: vec![SamplingParameter::Temperature],
                supported_features: vec![Feature::Tools],
                openrouter: None,
                hugging_face_id: None,
                description: Some("Test".to_string()),
                datacenters: Some(vec![Datacenter::new("US")]),
            }],
        };
        let json = serde_json::to_string(&original).unwrap();
        let parsed: ListModelsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_quantization_serialization() {
        assert_eq!(
            serde_json::to_string(&Quantization::Int4).unwrap(),
            "\"int4\""
        );
        assert_eq!(
            serde_json::to_string(&Quantization::Bf16).unwrap(),
            "\"bf16\""
        );
        assert_eq!(
            serde_json::to_string(&Quantization::Fp32).unwrap(),
            "\"fp32\""
        );
    }

    #[test]
    fn test_feature_serialization() {
        assert_eq!(
            serde_json::to_string(&Feature::JsonMode).unwrap(),
            "\"json_mode\""
        );
        assert_eq!(
            serde_json::to_string(&Feature::StructuredOutputs).unwrap(),
            "\"structured_outputs\""
        );
    }

    #[test]
    fn test_sampling_parameter_serialization() {
        assert_eq!(
            serde_json::to_string(&SamplingParameter::TopP).unwrap(),
            "\"top_p\""
        );
        assert_eq!(
            serde_json::to_string(&SamplingParameter::RepetitionPenalty).unwrap(),
            "\"repetition_penalty\""
        );
    }
}
