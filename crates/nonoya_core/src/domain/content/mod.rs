use crate::domain::{content::value_objects::ContentId, value_objects::ApiUrl};

pub mod value_objects;

pub struct Content {
    pub id: ContentId,
    pub content_type: ContentType,
    pub notify_channels: Vec<NotifyChannel>,
}

pub enum ContentType {
    ApiContent(ApiContent),
}

pub struct ApiContent {
    pub api_url: ApiUrl,
    pub response_struct: serde_json::Map<String, serde_json::Value>,
    pub selected_fields: Vec<String>,
}

pub enum NotifyChannel {
    Discord(DiscordNotifyChannel),
}

pub struct DiscordNotifyChannel {
    pub target: String,
}
