use futures::stream::StreamExt;

use async_openai::Client as OpenAIClient;
use async_openai::config::OpenAIConfig;
use async_openai::types::responses::{
    CreateResponse, CreateResponseArgs, EasyInputContent, EasyInputMessage, EasyInputMessageArgs,
    ResponseStreamEvent, Role,
};
use crossbeam_channel::Sender;
use tokio::runtime::{Builder, Runtime};

use super::config::Provider;
use super::session::state::Exchange;

pub struct Service {
    client: OpenAIClient<OpenAIConfig>,
    model: String,
    runtime: Runtime,
}

pub enum ServiceEvent {
    StreamStart,
    StreamComplete,
    StreamFail,
    StreamInComplete,
    DeltaText(String),
}

impl Service {
    pub fn new(provider: &Provider) -> Self {
        let openai_config = OpenAIConfig::default()
            .with_api_key(provider.api_key())
            .with_api_base(provider.base_url());

        let client = OpenAIClient::with_config(openai_config);
        let model = provider.model().to_string();

        let runtime = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("The builder should be able to build a multi thread runtime");

        Service {
            client,
            model,
            runtime,
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
            .stream(true)
            .build()
            .expect("The builder should be able to build a response.")
    }

    pub fn make_responses(&self, request: CreateResponse, sender: Sender<ServiceEvent>) {
        let client = self.client().clone();

        // We have to use tokio runtime, because `async_openai` use `reqwest`, which uses futures
        // provided by `tokio`, which requires tokio drivers, which come from a tokio runtime.
        self.runtime.spawn(async move {
            let mut stream = client.responses().create_stream(request).await.expect(
                "The client should be able to create a streaming response with given request.",
            );

            while let Some(result) = stream.next().await {
                let event = result.expect("The item in the stream should be a valid event.");
                tracing::debug!("Received event: {:#?}", event);

                // Thread blocking do happen here.
                // However, technically no dead lock will happen because the receiver side is an
                // independent thread instead of a runtime scheduled task.
                match event {
                    ResponseStreamEvent::ResponseCreated(_) => {
                        let _ = sender.send(ServiceEvent::StreamStart);
                    }
                    ResponseStreamEvent::ResponseCompleted(_) => {
                        let _ = sender.send(ServiceEvent::StreamComplete);
                    }
                    ResponseStreamEvent::ResponseFailed(_) => {
                        let _ = sender.send(ServiceEvent::StreamFail);
                    }
                    ResponseStreamEvent::ResponseIncomplete(_) => {
                        let _ = sender.send(ServiceEvent::StreamInComplete);
                    }
                    ResponseStreamEvent::ResponseOutputTextDelta(event) => {
                        let _ = sender.send(ServiceEvent::DeltaText(event.delta));
                    }
                    _ => (),
                }
            }
        });
    }

    pub fn client(&self) -> &OpenAIClient<OpenAIConfig> {
        &self.client
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    fn make_message(&self, role: Role, content: impl Into<String>) -> EasyInputMessage {
        EasyInputMessageArgs::default()
            .role(role)
            .content(EasyInputContent::Text(content.into()))
            .build()
            .expect("Given content and role should be valid to build an input message.")
    }
}
