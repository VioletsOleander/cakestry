use anyhow::Result;

use async_openai::Client as OpenAIClient;
use async_openai::config::OpenAIConfig;
use async_openai::types::responses::CreateResponseArgs;

use super::config::Provider;

#[derive(Debug)]
pub struct Client {
    openai_client: OpenAIClient<OpenAIConfig>,
    model: String,
}

impl Client {
    pub fn new(provider: &Provider) -> Self {
        let openai_config = OpenAIConfig::default()
            .with_api_key(provider.api_key())
            .with_api_base(provider.base_url());
        let openai_client = OpenAIClient::with_config(openai_config);

        Client {
            openai_client,
            model: provider.model().to_string(),
        }
    }

    /// Send `content` as a request, waiting for the response and return it.
    pub async fn send_request(&self, content: &str) -> Result<Option<String>> {
        let request = CreateResponseArgs::default()
            .model(&self.model)
            .input(content)
            .build()?;

        let response = self.openai_client.responses().create(request).await?;

        Ok(response.output_text())
    }
}
