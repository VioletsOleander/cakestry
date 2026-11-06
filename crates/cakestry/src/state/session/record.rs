use anyhow::Error;
use async_openai::types::responses::{
    EasyInputMessage, FunctionToolCall, InputItem, Item, ReasoningItem, Role, SummaryPart,
    SummaryTextContent, WebSearchToolCall,
};

pub enum Record {
    Prompt(String),
    Response {
        status: ResponseStatus,
        content: ResponseContent,
    },
}

pub enum ResponseStatus {
    Progress,
    Complete,
}

pub enum ResponseContent {
    Reason(String),
    Answer(String),
    ToolInvoke {
        id: String,
        content: ToolInvokeContent,
    },
    ToolResult {
        id: String,
        content: String,
    },
}

pub enum ToolInvokeContent {
    Function {
        name: String,
        arguments: String,
    },
    WebSearch {
        queries: Option<Vec<String>>,
        sources: Option<Vec<String>>,
    },
}

impl TryFrom<&Record> for InputItem {
    type Error = Error;

    fn try_from(value: &Record) -> Result<Self, Self::Error> {
        match value {
            Record::SystemPrompt(content) => Ok(make_message(Role::System, content.clone()).into()),
            Record::UserPrompt(content) => Ok(make_message(Role::User, content.clone()).into()),
            Record::Assistant {
                status: stage,
                content,
            } => match content {
                AssistantContent::Reason(content) => {
                    Ok(Item::from(make_reasoning(content.clone())).into())
                }
                AssistantContent::Answer(content) => {
                    Ok(make_message(Role::User, content.clone()).into())
                }
                AssistantContent::Invoke { id, tool } => match tool {
                    Tool::Local(tool) => match tool {
                        LocalTool::Function { name, arguments } => Ok(Item::from(
                            make_function_call(id.clone(), name.clone(), arguments.clone()),
                        )
                        .into()),
                    },
                    Tool::Remote(tool) => match tool {
                        RemoteTool::WebSearch { queries, sources } => Ok(Item::from(value).into()),
                    },
                },
            },
        }
    }
}

fn make_message(role: Role, content: String) -> EasyInputMessage {
    EasyInputMessage {
        role,
        content: content.into(),
        ..EasyInputMessage::default()
    }
}

fn make_reasoning(content: String) -> ReasoningItem {
    ReasoningItem {
        summary: vec![SummaryPart::SummaryText(SummaryTextContent {
            text: content,
        })],
        id: None,
        content: None,
        encrypted_content: None,
        status: None,
    }
}

fn make_function_call(call_id: String, name: String, arguments: String) -> FunctionToolCall {
    FunctionToolCall {
        call_id,
        name,
        arguments,
        id: None,
        namespace: None,
        status: None,
        caller: None,
        r#async: None,
    }
}

fn make_websearch_call(
    id: String,
    status: WebSearchCallStatus,
    queries: Option<Vec<String>>,
    sources: Option<Vec<String>>,
) -> WebSearchToolCall {
    let sources = match sources {
        Some(v) => Some(
            v.into_iter()
                .map(|s| WebSearchActionSearchSource {
                    r#type: "url".to_string(),
                    url: s,
                })
                .collect(),
        ),
        None => None,
    };

    WebSearchToolCall {
        id,
        status,
        action: WebSearchToolCallAction::Search(WebSearchActionSearch {
            #[allow(deprecated)]
            query: None,
            queries,
            sources,
        }),
    }
}
