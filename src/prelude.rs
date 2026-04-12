//! The Qinstagram Prelude
//!
//! Re-exports the core types and traits required to work with the library.

pub use crate::error::{InstagramError, Result};
pub use crate::client::{InstagramClient, ClientBuilder};

pub use crate::auth::session::SessionManager;
pub use crate::transport::device::DeviceInfo;

pub use crate::types::message::{Message, ReactionEvent, SeenEvent};
pub use crate::types::thread::Thread;
pub use crate::types::post::Post;
pub use crate::types::profile::ProfileInfo;
pub use crate::types::story::{Story, StoryReel};
pub use crate::types::media::MessageMedia;
