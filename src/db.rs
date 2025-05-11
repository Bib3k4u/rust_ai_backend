use mongodb::{Client, options::ClientOptions};
use std::error::Error;

pub async fn connect_db() -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse("mongodb+srv://bibek:bibek1123@cluster0.gaaho0t.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
