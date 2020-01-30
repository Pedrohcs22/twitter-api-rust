extern crate actix_web;

use actix_web::{HttpServer, App, web, HttpRequest, HttpResponse};
use mongodb::{Client, options::ClientOptions, options::FindOptions};
use bson::{doc, bson};

// Here is the handler,
// we are returning a json response with an ok status
// that contains the text Hello World
fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("Hello world!")
}

fn connectToDatabase() -> Result<(), mongodb::error::Error> {
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

    /*
     List Databases
     */
    let db = client.database("mydb");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None)? {
        println!("{}", collection_name);
    }


    Ok(())
}

fn main() {
    connectToDatabase().unwrap();

    // We are creating an Application instance and
    // register the request handler with a route and a resource
    // that creates a specific path, then the application instance
    // can be used with HttpServer to listen for incoming connections.
    HttpServer::new(|| App::new().service(
        web::resource("/").route(web::get().to_async(index))))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}