mod post_schema;
mod user_schema;
use mongododm::f;
use mongododm::mongo::{bson::doc, options::ClientOptions, Client};
use mongododm::ToRepository;
// use mongowner::owned_by;
use post_schema::Post;
use user_schema::User;
// use user_schema::UserB;
use std::env;
extern crate dotenv;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> mongododm::mongo::error::Result<()> {
    // Create a .env file in the "server" folder. Should only have one line, which is:
    // MONGOURI=mongodb+srv://[XYZ YOUR MONGO DB URI HERE]
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    let db = client.database("test1");


    // enforces that the repository i.e. collection is of type User and type Post
    let user_repository = db.repository::<user_schema::User>();
    let post_repository = db.repository::<post_schema::Post>();    

    // Clean users and posts repositories to begin
    user_repository.delete_many(doc! {}, None).await?;
    post_repository.delete_many(doc! {}, None).await?;

    // Show we have empty repositories
    println!("Repository start state (no text if empty):");
    let mut user_cursor = user_repository.find(None, None).await?;
    while user_cursor.advance().await? {
        println!("{:?}", user_cursor.deserialize_current()?);
    }
    let mut post_cusor = post_repository.find(None, None).await?;
    while post_cusor.advance().await? {
        println!("{:?}", post_cusor.deserialize_current()?);
    }


    let test_user_a = User {
        username: "alice".to_string(),
        first_name: "Alice".to_string(),
        last_name: "of Wonderland".to_string(),
        age: 0,
        email: "alice_of_wonderland@brown.edu".to_string(),
    };

    let test_user_b = User {
        username: "bob".to_string(),
        first_name: "Bob".to_string(),
        last_name: "Ross".to_string(),
        age: 0,
        email: "bob_ross@brown.edu".to_string(),
    };

    let post_a = Post {
        text: "Hello this is a post".to_string(),
        posted_by_username: test_user_a.username.to_string().clone(), 
        date: "12th March 2023".to_string(),
    };

    let post_b = Post {
        text: "Hello this is another post".to_string(),
        posted_by_username: test_user_a.username.to_string().clone(), 
        date: "12th March 2023".to_string(),
    };

    let post_c = Post {
        text: "Hello this is a third post".to_string(),
        posted_by_username: test_user_b.username.to_string().clone(), 
        date: "12th March 2023".to_string(),
    };

    // add users and posts 
    user_repository.insert_one(&test_user_a, None).await?;
    user_repository.insert_one(&test_user_b, None).await?;

    post_repository.insert_one(&post_a, None).await?;
    post_repository.insert_one(&post_b, None).await?;
    post_repository.insert_one(&post_c, None).await?;

    println!("New set of users and posts:");
    let mut user_cursor = user_repository.find(None, None).await?;
    while user_cursor.advance().await? {
        println!("{:?}", user_cursor.deserialize_current()?);
    }
    let mut post_cusor = post_repository.find(None, None).await?;
    while post_cusor.advance().await? {
        println!("{:?}", post_cusor.deserialize_current()?);
    }



    // println!("Test user a username {:?}", test_user_a.username);

    // let found_post = post_repository
    //     .find_one(doc! { f!(text in Post): "Hello this is a post" }, None)
    //     .await?
    //     .unwrap();
    
    // println!("Found post: {:?}", found_post);


    Ok(())
}
