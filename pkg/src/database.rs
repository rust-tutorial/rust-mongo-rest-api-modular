use mongodb::{Client, Database};
use bson::doc;
use mongodb::options::ClientOptions;

pub async fn connect(name: String, uri: String, db: String) -> Database {
    let mut client_options = ClientOptions::parse(uri).await.expect("Error parsing URI");
    client_options.app_name = Some(name);
    let client = Client::with_options(client_options).expect("Error client option");
    client.database(&db).run_command(doc! {"ping": 1}, None).await.expect("Error connecting to database");
    // Get a handle to a database.
    client.database(&db)
}
