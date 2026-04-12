use serde_json::json;
use qinstagram::types::message::Message;
use qinstagram::direct::parser::parse_message_item;

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
