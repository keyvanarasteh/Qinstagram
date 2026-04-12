use qinstagram::ClientBuilder;

#[tokio::main]
async fn main() -> qinstagram::Result<()> {
    let username = "test_user";
    let password = "test_password";

    println!("Initializing Instagram Client for {}...", username);
    let mut client = ClientBuilder::new(username).build().await?;
    
    println!("Logging in...");
    match client.login(username, password).await {
        Ok(result) => {
            if result.success {
                println!("Successfully logged in as {:?}", result.username);
            } else if let Some(two_factor) = result.two_factor_info {
                println!("Two factor auth required to phone: {}", two_factor.obfuscated_phone_number);
            } else if let Some(err) = result.error {
                println!("Login failed: {}", err);
            }
        }
        Err(e) => {
            println!("Network/API error during login: {:?}", e);
        }
    }
    
    Ok(())
}
