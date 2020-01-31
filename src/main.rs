pub mod handlers;

extern crate actix_web;

use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use mongodb::{Client, options::ClientOptions, options::FindOptions, Database};
use bson::{doc, bson};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    uuid: String,
    userId: String,
    content: String
}

fn connectToDatabase() -> Result<Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")?;

    // Manually set an option.
    client_options.app_name = Some("twitter".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None)? {
        println!("{}", db_name);
    }

    /**
        Collections insert
    */
    let db = client.database("twitter_api");

    // Get a handle to a collection in the database.
    let collection = db.collection("tweets");

    let docs = vec![
        doc! { "uuid": "123", "content": "Some political stuff" },
        doc! { "uuid": "1234", "content": "Some cats stuff" },
        doc! { "uuid": "12345", "content": "More cats" },
    ];

    // Insert some documents into the "mydb.tweets" collection.
    collection.insert_many(docs, None)?;

    /**
        Collection retrieve
    */
    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "content": "Some political stuff" };
    let find_options = FindOptions::builder()
        .sort(doc! { "uuid": 1 })
        .build();
    let cursor = collection.find(filter, find_options)?;

    // Iterate over the results of the cursor.
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("content") {
                    println!("content: {}", title);
                }  else {
                    println!("no content found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(db)
}

/// This handler uses json extractor
fn create(item: web::Json<Tweet>) -> HttpResponse {
    println!("model: {:?}", &item);
    println!(" Creating new Tweet");

    let generatedUuid = Uuid::new_v4();

    // Get a handle to a collection in the database.
    //let collection = db.collection("tweets");

    let docs = vec![
        doc! { "uuid": "generatedUuid" ,"userId": item.userId ,"content": item.content }
    ];

    // Insert some documents into the "mydb.tweets" collection.
    //collection.insert_many(docs, None)?;

    HttpResponse::Ok().json(item.0)
}

fn main() {
    let db = connectToDatabase().unwrap();

    let sys = actix::System::new("mystore");

    HttpServer::new(
        || App::new()
            .service(
                web::resource("/rest/v1/tweet")
                    .route(web::post().to(create))
            ))
        .bind("127.0.0.1:8088").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}