use anyhow::Result;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::responses::{CreateResponseArgs, EasyInputMessage, ResponseStreamEvent};
use crossbeam_channel::Sender;
use futures::stream::StreamExt;
use tokio::runtime::{Builder, Runtime};

use crate::config::Provider;

pub struct Service {
    model: String,
    client: Client<OpenAIConfig>,
    sender: Sender<Result<ResponseStreamEvent, OpenAIError>>,
    runtime: Runtime,
}

impl Service {
    pub fn build(
        provider: &Provider,
        sender: Sender<Result<ResponseStreamEvent, OpenAIError>>,
    ) -> Result<Self> {
        let config = OpenAIConfig::default()
            .with_api_key(provider.api_key())
            .with_api_base(provider.base_url());

        let client = Client::with_config(config);
        let model = provider.model().to_string();
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(Service {
            model,
            client,
            sender,
            runtime,
        })
    }

    pub fn launch_request(&self, messages: Vec<EasyInputMessage>) -> Result<()> {
        let request = CreateResponseArgs::default()
            .model(&self.model)
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
                        if let Ok(event) = &result {
                            tracing::debug!("received event: {:#?}", event);
                        }

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

    pub fn model(&self) -> &str {
        &self.model
    }
}
