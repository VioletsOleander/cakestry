use async_openai::Client as OpenAIClient;
use async_openai::config::OpenAIConfig;
use async_openai::types::responses::{
    CreateResponse, CreateResponseArgs, EasyInputMessage, EasyInputMessageArgs, Response, Role,
};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use super::config::Provider;
use super::session::state::Exchange;

pub struct Service {
    client: OpenAIClient<OpenAIConfig>,
    model: String,
}

impl Service {
    pub fn new(provider: &Provider) -> Self {
        let openai_config = OpenAIConfig::default()
            .with_api_key(provider.api_key())
            .with_api_base(provider.base_url());

        let client = OpenAIClient::with_config(openai_config);

        Service {
            client,
            model: provider.model().to_string(),
        }
    }

    /// Make a request with content based on given `exchanges` and `user_input`.
    pub fn make_request(&self, exchanges: &[Exchange], user_input: &str) -> CreateResponse {
        // Each exchange 2 message + 1 system prompt + 1 user input.
        let mut messages = Vec::with_capacity(2 * exchanges.len() + 2);

        messages.push(self.make_message(Role::System, "You are a helpful assistant"));
        for exchange in exchanges {
            messages.push(self.make_message(Role::User, exchange.query()));
            messages.push(self.make_message(Role::Assistant, exchange.reply()));
        }
        messages.push(self.make_message(Role::User, user_input));

        CreateResponseArgs::default()
            .model(&self.model)
            .input(messages)
            .build()
            .expect("Given messages should be valid to build a request.")
    }

    /// Dispatch a task of sending `request` to `runtime`, and return a [`JoinHandle`] of the response.
    pub async fn send_request(
        &self,
        request: CreateResponse,
        runtime: &Runtime,
    ) -> JoinHandle<Response> {
        let client = self.client.clone();

        runtime.spawn(async move {
            client
                .responses()
                .create(request)
                .await
                .expect("The client should be able to create a response with given request.")
        })
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    fn make_message(&self, role: Role, content: &str) -> EasyInputMessage {
        EasyInputMessageArgs::default()
            .role(role)
            .content(content)
            .build()
            .expect("Given content and role should be valid to build an input message.")
    }
}
