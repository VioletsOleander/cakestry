use anyhow::Result;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::responses::{CreateResponseArgs, InputParam, ResponseStreamEvent, Tool};
use crossbeam_channel::Sender;
use futures::stream::StreamExt;
use tokio::runtime::{Builder, Runtime};

use crate::state::State;

pub struct ResponseEventService {
    client: Client<OpenAIConfig>,
    sender: Sender<Result<ResponseStreamEvent, OpenAIError>>,
    runtime: Runtime,
}

impl ResponseEventService {
    pub fn build(
        state: &State,
        sender: Sender<Result<ResponseStreamEvent, OpenAIError>>,
    ) -> Result<Self> {
        let provider = &state.providers[state.provider_index];

        let openai_config = OpenAIConfig::default()
            .with_api_key(provider.api_key.clone())
            .with_api_base(provider.base_url.clone());

        let client = Client::with_config(openai_config);
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(Self {
            client,
            sender,
            runtime,
        })
    }

    pub fn create_responses(&self, input: InputParam, tools: Vec<Tool>, model: String) {
        // Request build error implies code logic error and therefore is unrecoverable.
        let request = CreateResponseArgs::default()
            .model(model)
            .input(input)
            .tools(tools)
            .stream(true)
            .build()
            .expect("model, input and tools should be enough to make CreaseReponse");

        tracing::debug!("the request to be sent is: {:#?}", request);

        let client = self.client.clone();
        let sender = self.sender.clone();

        self.runtime.spawn(async move {
            let result = client.responses().create_stream(request).await;

            match result {
                Ok(mut stream) => {
                    while let Some(result) = stream.next().await {
                        tracing::debug!("received result: {:#?}", result);

                        if let Err(err) = sender.send(result) {
                            tracing::debug!(
                                "response stream event receiver unexpectedly disconnected due to {}",
                                err
                            );
                            break;
                        }
                    }
                }
                Err(err) => {
                    let _ = sender.send(Err(err));
                }
            }
        });
    }
}
