# openrouter-provider

Type definitions for the [OpenRouter Provider API](https://openrouter.ai/docs/guides/for-providers).

```toml
[dependencies]
openrouter-provider = "0.1"
```

Example

```rust
use openrouter_provider::{
    ListModelsResponse, Model, Pricing, InputModality, OutputModality, Quantization
};

let response = ListModelsResponse {
    data: vec![Model {
        id: "my-org/my-model".to_string(),
        name: "My Model".to_string(),
        created: 1700000000,
        input_modalities: vec![InputModality::Text],
        output_modalities: vec![OutputModality::Text],
        quantization: Quantization::Fp16,
        context_length: 128000,
        max_output_length: 4096,
        pricing: Pricing::new("0.000001", "0.000002"),
        supported_sampling_parameters: vec![],
        supported_features: vec![],
        openrouter: None,
        hugging_face_id: None,
        description: None,
        datacenters: None,
    }],
};

let json = serde_json::to_string(&response).unwrap();
```
