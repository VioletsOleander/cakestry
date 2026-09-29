use anyhow::Result;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::responses::{CreateResponseArgs, EasyInputMessage, ResponseStreamEvent};
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
        let provider = state.active_provider();

        let openai_config = OpenAIConfig::default()
            .with_api_key(provider.api_key())
            .with_api_base(provider.base_url());

        let client = Client::with_config(openai_config);
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(Self {
            client,
            sender,
            runtime,
        })
    }

    pub fn create_responses(&self, messages: Vec<EasyInputMessage>, model: String) -> Result<()> {
        let request = CreateResponseArgs::default()
            .model(model)
            .input(messages)
            .stream(true)
            .build()?;

        let client = self.client.clone();
        let sender = self.sender.clone();

        self.runtime.spawn(async move {
            let result = client.responses().create_stream(request).await;

            match result {
                Ok(mut stream) => {
                    while let Some(result) = stream.next().await {
                        tracing::debug!("received result: {:#?}", result);

                        if sender.send(result).is_err() {
                            break;
                        }
                    }
                }
                Err(err) => {
                    let _ = sender.send(Err(err));
                }
            }
        });

        Ok(())
    }
}
