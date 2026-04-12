use chrono::{TimeZone, Utc};
use serde_json::Value;

use crate::types::message::{
    BaseMessage, Link, LinkMessage, MediaMessage, MediaShareMessage, 
    Message, PlaceholderMessage, TextMessage,
};
use crate::types::post::Post;
use crate::types::media::MessageMedia;

pub fn parse_message_item(item: &Value, thread_id: &str, current_user_id: &str) -> Option<Message> {
    let item_id = item.get("item_id").and_then(|v| v.as_str())?.to_string();
    let user_id = item.get("user_id").and_then(|v| {
        if v.is_string() { v.as_str().map(|s| s.to_string()) }
        else if v.is_number() { Some(v.to_string()) }
        else { None }
    })?;
    
    let timestamp_u64 = item.get("timestamp").and_then(|t| {
        if t.is_number() { t.as_u64() }
        else if t.is_string() { t.as_str()?.parse().ok() }
        else { None }
    }).unwrap_or(0);
    
    let timestamp = Utc.timestamp_opt((timestamp_u64 / 1000) as i64, 0).single().unwrap_or_else(Utc::now);
    
    let item_type = item.get("item_type").and_then(|v| v.as_str()).unwrap_or("unknown");
    
    // Simplification for userCache mapping: "You" or "User_X"
    let username = if user_id == current_user_id { "You".to_string() } else { format!("User_{}", user_id) };
    
    let base = BaseMessage {
        id: item_id.clone(),
        timestamp,
        user_id: user_id.clone(),
        username,
        is_outgoing: user_id == current_user_id,
        thread_id: thread_id.to_string(),
        reactions: None, // Can be parsed if payload provides reactions obj
        replied_to: None, 
        item_id: Some(item_id),
        client_context: item.get("client_context").and_then(|v| v.as_str().map(|s| s.to_string())),
    };

    match item_type {
        "text" => {
            let text = item.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            Some(Message::Text(TextMessage { base, text }))
        },
        "media" => {
            if let Some(media_val) = item.get("media") {
                if let Ok(media) = serde_json::from_value::<MessageMedia>(media_val.clone()) {
                    return Some(Message::Media(MediaMessage { base, media }));
                }
            }
            Some(Message::Placeholder(PlaceholderMessage { base, text: "[Unsupported Media]".into() }))
        },
        "link" => {
            if let Some(link_obj) = item.get("link") {
                let text = link_obj.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let mut url = text.clone();
                if let Some(link_ctx) = link_obj.get("link_context") {
                    url = link_ctx.get("link_url").and_then(|v| v.as_str()).unwrap_or(&url).to_string();
                }
                Some(Message::Link(LinkMessage { base, link: Link { url, text } }))
            } else if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                Some(Message::Link(LinkMessage { base, link: Link { url: text.to_string(), text: text.to_string() } }))
            } else {
                Some(Message::Placeholder(PlaceholderMessage { base, text: "[Sent a link]".into() }))
            }
        },
        "like" => Some(Message::Placeholder(PlaceholderMessage { base, text: "[Sent a ❤️]".into() })),
        "media_share" | "xma_media_share" | "direct_media_share" => {
            let media_share = item.get("media_share").or_else(|| item.get("xma_media_share")).or_else(|| item.get("direct_media_share")).or_else(|| item.get("media"));
            if let Some(media_val) = media_share {
                if let Ok(post) = serde_json::from_value::<Post>(media_val.clone()) {
                    return Some(Message::MediaShare(MediaShareMessage { base, media_share_post: post, media_share_index: None }));
                }
            }
            Some(Message::Placeholder(PlaceholderMessage { base, text: "[Shared a post]".into() }))
        },
        "raven_media" | "reel_share" | "clip" => {
            Some(Message::Placeholder(PlaceholderMessage { base, text: "[Instagram CLI successfully blocked a brainrot]".into() }))
        },
        "action_log" => {
            if let Some(hidden) = item.get("hide_in_thread").and_then(|v| v.as_i64()) {
                if hidden == 1 { return None; }
            }
            let desc = item.get("action_log").and_then(|v| v.get("description")).and_then(|v| v.as_str()).unwrap_or("Action log").to_string();
            Some(Message::Placeholder(PlaceholderMessage { base, text: desc }))
        },
        other => {
            Some(Message::Placeholder(PlaceholderMessage { base, text: format!("[Unsupported Type: {}]", other) }))
        }
    }
}
