pub mod user_schema;
use mongodb::{bson::doc, options::ClientOptions, Client};
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client
        .database("admin")
        .run_command(doc! { "ping": 1 }, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}
