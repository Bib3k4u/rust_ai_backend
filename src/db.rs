use mongodb::{Client, error::Error};

pub async fn connect_db() -> Result<Client, Error> {
    // For MongoDB 2.x
    let uri = "mongodb+srv://bibek:bibek1123@cluster0.gaaho0t.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
    
    // This is the correct async way to connect in MongoDB 2.x
    Client::with_uri_str(uri).await
}