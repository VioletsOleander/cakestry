use async_openai::types::responses::{
    EasyInputMessage, FunctionCallOutputItemParam, FunctionToolCall, InputItem, Item, OutputStatus,
    ReasoningItem, ReasoningItemContent, ReasoningTextContent, Role, SummaryPart,
    SummaryTextContent, WebSearchCallStatus, WebSearchToolCall, WebSearchToolCallAction,
};

pub enum Record {
    Prompt(String),
    Response {
        status: ResponseStatus,
        content: ResponseContent,
    },
}

#[derive(Clone, Copy)]
pub enum ResponseStatus {
    InProgress,
    InComplete,
    Completed,
}

pub enum ResponseContent {
    Reason {
        content: Option<String>,
        // Vector of summary parts.
        summary: Vec<String>,
    },
    Answer(String),
    ToolInvoke {
        id: String,
        content: ToolInvokeContent,
    },
    // Currently only for function result.
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
        action: Option<WebSearchToolCallAction>,
        status: WebSearchCallStatus,
    },
}

impl From<&Record> for InputItem {
    fn from(value: &Record) -> Self {
        match value {
            Record::Prompt(content) => EasyInputMessage {
                role: Role::User,
                content: content.clone().into(),
                ..Default::default()
            }
            .into(),
            Record::Response { status, content } => from_response(status, content),
        }
    }
}

fn from_response(status: &ResponseStatus, content: &ResponseContent) -> InputItem {
    match content {
        ResponseContent::Reason { content, summary } => Item::from(ReasoningItem {
            content: content.clone().map(|v| {
                vec![ReasoningItemContent::ReasoningText(ReasoningTextContent {
                    text: v,
                })]
            }),
            summary: summary
                .iter()
                .map(|v| SummaryPart::SummaryText(SummaryTextContent { text: v.clone() }))
                .collect(),
            ..Default::default()
        })
        .into(),
        ResponseContent::Answer(content) => EasyInputMessage {
            role: Role::Assistant,
            content: content.clone().into(),
            ..Default::default()
        }
        .into(),
        ResponseContent::ToolInvoke { id, content } => from_tool_invoke(&id, &content),
        // Function call is managed locally so use the status type defined by us instaed of openai sdk.
        ResponseContent::ToolResult { id, content } => from_tool_result(&id, &content, status),
    }
}

fn from_tool_invoke(id: &String, content: &ToolInvokeContent) -> InputItem {
    match content {
        ToolInvokeContent::Function { name, arguments } => Item::from(FunctionToolCall {
            call_id: id.clone(),
            name: name.clone(),
            arguments: arguments.clone(),
            ..Default::default()
        })
        .into(),
        ToolInvokeContent::WebSearch { action, status } => Item::from(WebSearchToolCall {
            id: id.clone(),
            action: action.clone(),
            status: status.clone(),
        })
        .into(),
    }
}

fn from_tool_result(id: &String, content: &String, status: &ResponseStatus) -> InputItem {
    Item::from(FunctionCallOutputItemParam {
        call_id: Some(id.clone()),
        output: content.clone().into(),
        status: Some(status.clone().into()),
        id: None,
        namespace: None,
        caller: None,
        name: None,
    })
    .into()
}

impl From<ResponseStatus> for OutputStatus {
    fn from(value: ResponseStatus) -> Self {
        match value {
            ResponseStatus::InProgress => OutputStatus::InProgress,
            ResponseStatus::InComplete => OutputStatus::Incomplete,
            ResponseStatus::Completed => OutputStatus::Completed,
        }
    }
}
