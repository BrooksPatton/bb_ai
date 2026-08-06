use eyre::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

pub fn get_list_of_models(
    api_key: &str,
    limit: usize,
    offset: usize,
) -> Result<Vec<OpenRouterModel>> {
    let client = Client::new();
    let url =
        format!("{OPENROUTER_BASE_URL}/models?limit={limit}&offset={offset}&sort=most-popular");
    let token = format!("Bearer {api_key}");

    let response = client
        .get(url)
        .header("Authorization", token)
        .send()
        .context("getting paginated list of openrouter models")?
        .json::<OpenRouterModelResponse>()?;
    let models = response
        .data
        .into_iter()
        .map(|response| OpenRouterModel {
            context_length: response.context_length,
            id: response.id,
            name: response.name,
            input_price: response.pricing.prompt,
            output_price: response.pricing.completion,
        })
        .collect();

    Ok(models)
}

#[derive(Debug)]
pub struct OpenRouterModel {
    pub context_length: usize,
    pub id: String,
    pub name: String,
    pub input_price: String,
    pub output_price: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenRouterModelResponse {
    data: Vec<OpenRouterModelResponseData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenRouterModelResponseData {
    context_length: usize,
    id: String,
    name: String,
    pricing: OpenRouterModelResponseDataPricing,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenRouterModelResponseDataPricing {
    completion: String,
    prompt: String,
}
