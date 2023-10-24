pub mod user_schema;
use mongodm::mongo::{bson::doc, options::ClientOptions, Client};
use mongodm::ToRepository;
use user_schema::UserB;
#[tokio::main]
async fn main() -> mongodm::mongo::error::Result<()> {
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

    let db = client.database("test1");
    let test_user = UserB {
        username: 123,
        first_name: "Bob".to_string(),
        last_name: "Alice".to_string(),
    };

    let test_user_a = user_schema::User {
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 2,
        email: "alice_bob".to_string(),
    };
    let repository = db.repository::<user_schema::User>();
    repository.insert_one(&test_user, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}
