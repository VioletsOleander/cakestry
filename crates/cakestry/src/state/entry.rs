use anyhow::Error;
use async_openai::types::responses::{
    EasyInputMessage, FunctionToolCall, InputItem, Item, ReasoningItem, Role, SummaryPart,
    SummaryTextContent, WebSearchToolCall,
};

pub enum Entry {
    SystemPrompt(String),
    UserPrompt(String),
    AssistantResponse {
        stage: ResponseStage,
        content: AssistantResponseContent,
    },
    ToolResponse {
        stage: ResponseStage,
        content: ToolResponseContent,
    },
}

pub enum ResponseStage {
    Progress { tick: usize },
    Complete { success: bool },
}

pub enum AssistantResponseContent {
    Reason(String),
    Answer(String),
    Invoke { id: String, tool: Tool },
}

pub enum ToolResponseContent {
    Function { id: String, output: String },
    WebSearch {},
}

pub enum Tool {
    Local(LocalTool),
    Remote(RemoteTool),
}

pub enum LocalTool {
    Function { name: String, arguments: String },
}

pub enum RemoteTool {
    WebSearch {
        queries: Option<Vec<String>>,
        sources: Option<Vec<String>>,
    },
}

impl TryFrom<&Entry> for InputItem {
    type Error = Error;

    fn try_from(value: &Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::SystemPrompt(content) => Ok(make_message(Role::System, content.clone()).into()),
            Entry::UserPrompt(content) => Ok(make_message(Role::User, content.clone()).into()),
            Entry::AssistantResponse { stage, content } => match content {
                AssistantResponseContent::Reason(content) => {
                    Ok(Item::from(make_reasoning(content.clone())).into())
                }
                AssistantResponseContent::Answer(content) => {
                    Ok(make_message(Role::User, content.clone()).into())
                }
                AssistantResponseContent::Invoke { id, tool } => match tool {
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

fn make_search_call(
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
