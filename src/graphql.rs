#[cfg(feature = "graphql")]
use async_graphql::{Context, Object, Result};
#[cfg(feature = "graphql")]
use std::sync::Arc;

#[cfg(feature = "graphql")]
use crate::client::InstagramClient;
#[cfg(feature = "graphql")]
use crate::types::{
    profile::{ProfileInfo, AuthState},
    thread::{Thread, User},
    story::{StoryReel, Story},
    message::{Message, ReactionEvent},
    post::{FeedInstance, Post},
};
#[cfg(feature = "graphql")]
use crate::auth::login::{LoginResult, TwoFactorInfo, CheckpointError};
#[cfg(feature = "graphql")]
use crate::direct::thread::MessagesResult;
#[cfg(feature = "graphql")]
use crate::notify::inbox::NewsInbox;

#[cfg(feature = "graphql")]
#[derive(async_graphql::SimpleObject)]
pub struct ThreadSearchResult {
    pub thread: Thread,
    pub score: f64,
}

#[cfg(feature = "graphql")]
#[derive(Default)]
pub struct QinstagramQuery;

#[cfg(feature = "graphql")]
#[Object]
impl QinstagramQuery {
    /// Retrieve the currently authenticated user's profile info
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
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

    /// Retrieve the news inbox
    async fn news_inbox(&self, ctx: &Context<'_>) -> Result<String> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_news_inbox().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        serde_json::to_string(&result).map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    /// Retrieve messages for a specific thread
    async fn messages(&self, ctx: &Context<'_>, thread_id: String, cursor: Option<String>) -> Result<MessagesResult> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_messages(&thread_id, cursor.as_deref()).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Retrieve the main timeline feed
    async fn timeline_feed(&self, ctx: &Context<'_>) -> Result<FeedInstance> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_timeline_feed().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Get stories for a specific user
    async fn stories_for_user(&self, ctx: &Context<'_>, user_id: String) -> Result<Vec<Story>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_stories_for_user(&user_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Search users by query
    async fn search_users(&self, ctx: &Context<'_>, query: String) -> Result<Vec<User>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.search_users(&query).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Search a specific user exact
    async fn search_user_exact(&self, ctx: &Context<'_>, username: String) -> Result<Option<User>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.search_user_exact(&username).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Search threads by title via Jaro-Winkler
    async fn search_threads_by_title(&self, ctx: &Context<'_>, query: String) -> Result<Vec<ThreadSearchResult>> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.search_threads_by_title(&query).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result.into_iter().map(|(thread, score)| ThreadSearchResult { thread, score }).collect())
    }

    /// Fetch profile info by PK
    async fn user_info_by_pk(&self, ctx: &Context<'_>, pk: String) -> Result<ProfileInfo> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_user_info(&pk).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Fetch profile info by username
    async fn user_profile_by_username(&self, ctx: &Context<'_>, username: String) -> Result<ProfileInfo> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let result = client.get_user_profile(&username).await
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

    /// Unsend an existing message
    async fn unsend_message(&self, ctx: &Context<'_>, thread_id: String, item_id: String) -> Result<bool> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        client.unsend_message(&thread_id, &item_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Ensure thread exists with users
    async fn ensure_thread(&self, ctx: &Context<'_>, user_pks: Vec<String>) -> Result<Thread> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let slices: Vec<&str> = user_pks.iter().map(|s| s.as_str()).collect();
        let result = client.ensure_thread(&slices).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Mark entire thread as seen
    async fn mark_thread_seen(&self, ctx: &Context<'_>, thread_id: String, item_id: String) -> Result<bool> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        client.mark_thread_as_seen(&thread_id, &item_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Send direct photo media
    async fn send_photo(&self, ctx: &Context<'_>, thread_id: String, file_path_str: String) -> Result<String> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let p = std::path::Path::new(&file_path_str);
        let result = client.send_photo(&thread_id, p).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    /// Send direct video media
    async fn send_video(&self, ctx: &Context<'_>, thread_id: String, file_path_str: String) -> Result<String> {
        let client = ctx.data::<Arc<InstagramClient>>()?;
        let p = std::path::Path::new(&file_path_str);
        let result = client.send_video(&thread_id, p).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(result)
    }

    // --- Auth Context Modifiers ---
    // We instantiate standalone client instances to bypass Arc immutability,
    // ensuring we can perform `&mut self` login flows perfectly.
    
    /// Normal login flow
    async fn login(&self, _ctx: &Context<'_>, username: String, password: String) -> Result<LoginResult> {
        let mut local_client = crate::client::ClientBuilder::new(&username).build().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let res = local_client.login(&username, &password).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(res)
    }

    /// Two Factor login flow
    async fn two_factor_login(&self, _ctx: &Context<'_>, code: String, identifier: String, username: String) -> Result<LoginResult> {
        let mut local_client = crate::client::ClientBuilder::new(&username).build().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let res = local_client.two_factor_login(&code, &identifier, &username).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(res)
    }

    /// Start Challenge flow
    async fn start_challenge(&self, _ctx: &Context<'_>, url: String, username: String) -> Result<bool> {
        let mut local_client = crate::client::ClientBuilder::new(&username).build().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        local_client.start_challenge(&url).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Send Challenge Code
    async fn send_challenge_code(&self, _ctx: &Context<'_>, url: String, code: String, username: String) -> Result<LoginResult> {
        let mut local_client = crate::client::ClientBuilder::new(&username).build().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        let res = local_client.send_challenge_code(&url, &code, &username).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(res)
    }

    /// Application Level static controls
    async fn switch_user(&self, _ctx: &Context<'_>, username: String) -> Result<bool> {
        crate::client::InstagramClient::switch_user(&username).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    async fn logout(&self, _ctx: &Context<'_>, username: Option<String>) -> Result<bool> {
        crate::client::InstagramClient::logout(username.as_deref()).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    async fn cleanup_sessions(&self, _ctx: &Context<'_>) -> Result<bool> {
        crate::client::InstagramClient::cleanup_sessions().await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}
