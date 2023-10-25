mod post_schema;
mod user_schema;
use mongododm::f;
use mongododm::mongo::{bson::doc, options::ClientOptions, Client};
use mongododm::ToRepository;
use mongowner::owned_by;
use post_schema::Post;
use user_schema::User;
use user_schema::UserB;

#[tokio::main]
async fn main() -> mongododm::mongo::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    let db = client.database("test1");
    let test_user = UserB {
        username: 123,
        first_name: "Bob".to_string(),
        last_name: "Alice".to_string(),
    };

    let test_user_a = User {
        username: "Alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Bob".to_string(),
        age: 2,
        email: "alice_bob".to_string(),
    };

    let post = Post {
        text: "Hello this is a post".to_string(),
        posted_by: test_user_a.clone(), // This should probably ref an id or something
        date: "12th March 2023".to_string(),
    };

    // enforces that the repository i.e. collection is of type User
    let repository = db.repository::<user_schema::User>();
    repository.insert_one(&test_user_a, None).await?;

    let found_user = repository
        .find_one(doc! { f!(username in User): "Alice" }, None)
        .await?
        .unwrap();

    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}
