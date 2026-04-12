#[cfg(feature = "graphql")]
use async_graphql::{Context, Object, Result};
#[cfg(feature = "graphql")]
use std::sync::Arc;

#[cfg(feature = "graphql")]
use crate::client::InstagramClient;
#[cfg(feature = "graphql")]
use crate::types::{
    profile::{ProfileInfo, AuthState},
    thread::Thread,
    story::{StoryReel, Story},
    message::{Message, ReactionEvent},
};

#[cfg(feature = "graphql")]
#[derive(Default)]
pub struct QinstagramQuery;

#[cfg(feature = "graphql")]
#[Object]
impl QinstagramQuery {
    /// Retrieve the currently authenticated user's profile info
    async fn current_user(&self, ctx: &Context<'_>) -> Result<ProfileInfo> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let user = client.get_current_user().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(user)
    }

    /// Retrieve the direct messaging threads for the user
    async fn direct_threads(&self, ctx: &Context<'_>) -> Result<Vec<Thread>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_threads(None).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result.threads)
    }

    /// Search for a thread by exact username
    async fn thread_by_username(&self, ctx: &Context<'_>, username: String) -> Result<Option<Thread>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.search_thread_by_username(&username).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Retrieve the reels tray for the user (who has posted stories)
    async fn reels_tray(&self, ctx: &Context<'_>) -> Result<Vec<StoryReel>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_reels_tray().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }
}

#[cfg(feature = "graphql")]
#[derive(Default)]
pub struct QinstagramMutation;

#[cfg(feature = "graphql")]
#[Object]
impl QinstagramMutation {
    /// Send a text message to a specific thread
    async fn send_message(&self, ctx: &Context<'_>, thread_id: String, text: String) -> Result<bool> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        client.send_message(&thread_id, &text).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Send a reaction emoji to a message
    async fn send_reaction(&self, ctx: &Context<'_>, thread_id: String, item_id: String, emoji: String) -> Result<bool> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        client.send_reaction(&thread_id, &item_id, &emoji).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Send a seen receipt for a thread item
    async fn mark_item_seen(&self, ctx: &Context<'_>, thread_id: String, item_id: String) -> Result<bool> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        client.mark_item_as_seen(&thread_id, &item_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}
