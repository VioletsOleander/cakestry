use anyhow::Result;

use async_openai::Client as OpenAIClient;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::responses::{
    CreateResponse, CreateResponseArgs, EasyInputMessageArgs, Response, Role,
};

use super::config::Provider;
use super::session::state::Exchange;

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

    /// Make a request with content based on given `exchanges` and `user_input`.
    pub fn make_request(
        &self,
        exchanges: &[Exchange],
        user_input: String,
    ) -> Result<CreateResponse, OpenAIError> {
        // Each exchange 2 message + 1 system prompt + 1 user input.
        let num_messages = 2 * exchanges.len() + 2;
        let mut messages = Vec::with_capacity(num_messages);

        messages.push(
            EasyInputMessageArgs::default()
                .role(Role::System)
                .content("You are a helpful assistant.")
                .build()?,
        );

        for exchange in exchanges {
            messages.push(
                EasyInputMessageArgs::default()
                    .role(Role::User)
                    .content(exchange.query())
                    .build()?,
            );
            messages.push(
                EasyInputMessageArgs::default()
                    .role(Role::Assistant)
                    .content(exchange.reply())
                    .build()?,
            );
        }

        messages.push(
            EasyInputMessageArgs::default()
                .role(Role::User)
                .content(user_input)
                .build()?,
        );

        CreateResponseArgs::default()
            .model(&self.model)
            .input(messages)
            .build()
    }

    /// Send `request` and return the response future.
    pub async fn send_request(&self, request: CreateResponse) -> Result<Response, OpenAIError> {
        self.openai_client.responses().create(request).await
    }

    pub fn model(&self) -> &str {
        &self.model
    }
}
