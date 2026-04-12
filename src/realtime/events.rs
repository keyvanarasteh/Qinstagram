use crate::types::message::{Message, ReactionEvent, SeenEvent};

pub enum RealtimeEvent {
    NewMessage(Message),
    Reaction(ReactionEvent),
    ReadReceipt(SeenEvent),
    StatusChange(String),
}
