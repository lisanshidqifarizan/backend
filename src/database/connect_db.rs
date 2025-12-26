use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

pub async fn connect_db() -> mongodb::error::Result<Client> {
    let uri = std::env::var("MONGO_URI").expect("MONGO_URI env tidak ditemukan");

    let mut options = ClientOptions::parse(uri).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    options.server_api = Some(server_api);

    let client = Client::with_options(options)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    println!("MongoDB connected");
    Ok(client)
}
