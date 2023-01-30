use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Mongo {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub database: String,
}

#[allow(dead_code)]
pub async fn generate_database(mongo: Mongo) -> mongodb::Database {
    let options = format!(
        "mongodb://{}:{}@{}:{}",
        mongo.username,
        mongo.password,
        mongo.ip.unwrap_or(String::from("localhost")),
        mongo.port.unwrap_or(String::from("27017"))
    );

    mongodb::Client::with_options(ClientOptions::parse(options).await.unwrap())
        .expect("could not connect to database")
        .database(&mongo.database)
}

impl Mongo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn username(mut self, username: &str) -> Self {
        self.username = String::from(username);
        self
    }
    #[allow(dead_code)]
    pub fn password(mut self, password: &str) -> Self {
        self.password = String::from(password);
        self
    }
    #[allow(dead_code)]
    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = Some(String::from(ip));
        self
    }
    #[allow(dead_code)]
    pub fn port(mut self, port: &str) -> Self {
        self.port = Some(String::from(port));
        self
    }
    #[allow(dead_code)]
    pub fn database(mut self, db: &str) -> Self {
        self.database = String::from(db);
        self
    }
    #[allow(dead_code)]
    pub async fn db_generate(self) -> mongodb::Database {
        generate_database(self).await
    }
}
