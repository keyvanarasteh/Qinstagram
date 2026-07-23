use serde_json::json;
use qinstagram::types::message::{Message, TextMessage, BaseMessage};
use qinstagram::direct::parser::parse_message_item;
use qinstagram::direct::thread::MessagesResult;
use qinstagram::types::story::{StoryUser, StoryReel};
use qinstagram::types::post::PostUser;
use qinstagram::transport::device::DeviceInfo;
use qinstagram::constants::WEB_CHROME_VERSION;
use chrono::Utc;

#[test]
fn test_parse_text_message() {
    let mock_json = json!({
        "item_id": "12345",
        "user_id": "111",
        "timestamp": 1600000000000u64,
        "item_type": "text",
        "text": "Hello world!"
    });
    
    let parsed = parse_message_item(&mock_json, "thread_1", "999");
    assert!(parsed.is_some());
    if let Some(Message::Text(t)) = parsed {
        assert_eq!(t.text, "Hello world!");
        assert_eq!(t.base.id, "12345");
        assert_eq!(t.base.is_outgoing, false);
    } else {
        panic!("Parsed wrong type");
    }
}

#[test]
fn test_parse_link_message() {
    let mock_json = json!({
        "item_id": "67890",
        "user_id": "999",
        "timestamp": 1600000000000u64,
        "item_type": "link",
        "link": {
            "text": "Check this out",
            "link_context": {
                "link_url": "https://example.com"
            }
        }
    });

    let parsed = parse_message_item(&mock_json, "thread_1", "999");
    assert!(parsed.is_some());
    if let Some(Message::Link(m)) = parsed {
        assert_eq!(m.link.text, "Check this out");
        assert_eq!(m.link.url, "https://example.com");
        assert_eq!(m.base.is_outgoing, true);
    } else {
        panic!("Parsed wrong type");
    }
}

#[test]
fn test_blocked_brainrot_message() {
    let mock_json = json!({
        "item_id": "99999",
        "user_id": "111",
        "timestamp": 1600000005000u64,
        "item_type": "reel_share"
    });

    let parsed = parse_message_item(&mock_json, "thread_1", "999");
    assert!(parsed.is_some());
    if let Some(Message::Placeholder(p)) = parsed {
        assert_eq!(p.text, "[Instagram CLI successfully blocked a brainrot]");
    } else {
        panic!("Parsed wrong type");
    }
}

fn text_msg(id: &str) -> Message {
    Message::Text(TextMessage {
        base: BaseMessage {
            id: id.to_string(),
            timestamp: Utc::now(),
            user_id: "1".into(),
            username: "u".into(),
            is_outgoing: false,
            thread_id: "t".into(),
            reactions: None,
            replied_to: None,
            item_id: Some(id.to_string()),
            client_context: None,
        },
        text: id.to_string(),
    })
}

#[test]
fn test_take_latest_messages_selects_newest() {
    // Chronological oldest → newest: 10, 9, …, 0  (ids as labels for clarity)
    let messages: Vec<Message> = (0..=10)
        .rev()
        .map(|n| text_msg(&n.to_string()))
        .collect();
    // After reverse map: messages[0] is "10" (oldest), messages[10] is "0" (newest)
    assert_eq!(
        match &messages[0] {
            Message::Text(t) => t.text.as_str(),
            _ => "",
        },
        "10"
    );

    let result = MessagesResult {
        messages,
        has_more: false,
        next_cursor: Some("api_cursor".into()),
    };

    let limited = result.take_latest(5);
    assert_eq!(limited.messages.len(), 5);
    let ids: Vec<&str> = limited
        .messages
        .iter()
        .map(|m| match m {
            Message::Text(t) => t.text.as_str(),
            _ => "",
        })
        .collect();
    // Latest 5 of 10..0 are 4,3,2,1,0
    assert_eq!(ids, vec!["4", "3", "2", "1", "0"]);
    assert!(limited.has_more);
    assert_eq!(limited.oldest_item_id(), Some("4"));
    assert_eq!(limited.latest_item_id(), Some("0"));
}

#[test]
fn test_story_user_and_reel_parse_full_name_and_items_alias() {
    let reel_json = json!({
        "user": {
            "pk": 42,
            "username": "alice",
            "full_name": "Alice Example",
            "profile_pic_url": "https://example.com/a.jpg"
        },
        "items": []
    });

    let reel: StoryReel = serde_json::from_value(reel_json).expect("reel parse");
    assert_eq!(reel.user.username, "alice");
    assert_eq!(reel.user.full_name.as_deref(), Some("Alice Example"));
    assert!(reel.stories.is_empty());
}

#[test]
fn test_post_user_full_name() {
    let user: PostUser = serde_json::from_value(json!({
        "pk": 1,
        "username": "bob",
        "full_name": "Bob Builder",
        "profile_pic_url": null
    }))
    .expect("post user");
    assert_eq!(user.full_name.as_deref(), Some("Bob Builder"));

    // Missing full_name still deserializes
    let user2: StoryUser = serde_json::from_value(json!({
        "pk": 2,
        "username": "carol"
    }))
    .expect("story user without full_name");
    assert!(user2.full_name.is_none());
}

#[test]
fn test_device_web_user_agent_uses_chrome_131() {
    let device = DeviceInfo::generate("testuser");
    assert!(
        device.web_user_agent.contains(&format!("Chrome/{}", WEB_CHROME_VERSION)),
        "web UA should embed modern Chrome: {}",
        device.web_user_agent
    );
    assert!(!device.web_user_agent.contains("Chrome/70."));
    assert!(device.web_user_agent.contains(&device.device_string));

    // Old session without web_user_agent field gets a modern default
    let json = r#"{
        "device_id": "android-abc",
        "phone_id": "00000000-0000-4000-8000-000000000000",
        "uuid": "00000000-0000-4000-8000-000000000001",
        "device_string": "Instagram 416.0.0.47.66 Android (24/7.0; 640dpi; 1440x2560; samsung; SM-G930F; herolte; samsungexynos8890; en_US; 382206157)"
    }"#;
    let mut loaded: DeviceInfo = serde_json::from_str(json).expect("deserialize old device");
    assert!(loaded.web_user_agent.contains("Chrome/131"));
    loaded.web_user_agent = "Mozilla/5.0 Chrome/70.0.3538.110 Mobile Safari/537.36".into();
    loaded.ensure_modern_web_user_agent();
    assert!(loaded.web_user_agent.contains("Chrome/131"));
    assert!(!loaded.web_user_agent.contains("Chrome/70."));
}
