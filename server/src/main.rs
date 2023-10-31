mod post;
mod user;
use std::str::FromStr;

use user::User;
use post::Post;
use mongododm::{f, SafeDelete};
use mongododm::mongo::{bson::doc, options::ClientOptions, Client};
use mongododm::ToRepository;

#[tokio::main]
async fn main() -> mongododm::mongo::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    let db = client.database("social");

    let test_user_a = User {
        id: None,
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 2,
        email: "alice_bob".to_string(),
    };
    test_user_a.safe_delete();

    let post = Post {
        text: "Hello this is a post".to_string(),
        posted_by: 0,
        date: "12th March 2023".to_string(),
    };

    // enforces that the repository i.e. collection is of type User
    let repository = db.repository::<User>();
    repository.insert_one(&test_user_a, None).await?;

    let found_user = repository
        .find_one(doc! { f!(username in User): "Alice" }, None)
        .await?
        .unwrap();

    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}
