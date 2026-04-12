use qinstagram::ClientBuilder;
use qinstagram::auth::session::SessionManager;

#[tokio::main]
async fn main() -> qinstagram::Result<()> {
    let username = "my_username";
    
    let session_manager = SessionManager::new(username).await?;
    
    let mut client = ClientBuilder::new(username).build().await?;
        
    println!("Restoring session for {}...", username);
    
    if let Ok(res) = client.login_by_session(&session_manager).await {
        if res.success {
            println!("Session restored. Fetching inbox...");
            let inbox = client.get_threads(None).await?;
            for thread in inbox.threads {
                println!("Thread: {} - {} messages unread? {}", thread.title, thread.users.len(), thread.unread);
            }
        } else {
            println!("No active session found. Please run the login example first.");
        }
    } else {
        println!("Error restoring session.");
    }
    
    Ok(())
}
