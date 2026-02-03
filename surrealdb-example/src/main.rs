use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
    Response,
    sql::Thing,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<T>,
    pub username: String,
    pub password: String,
}

async fn insert_tests(db: Surreal<Client>) -> surrealdb::Result<()> {
    let _: User<Thing> = db.create("user").content(User::<Thing> {id: None, username: "User1".to_string(), password: "pwd1".to_string()}).await?;
    let _: User<Thing> = db.create("user").content(User::<Thing> {id: None, username: "User2".to_string(), password: "pwd2".to_string()}).await?;
    let _: User<Thing> = db.create("user").content(User::<Thing> {id: None, username: "User3".to_string(), password: "pwd3".to_string()}).await?;
    let _: User<Thing> = db.create("user").content(User::<Thing> {id: None, username: "User4".to_string(), password: "pwd4".to_string()}).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let host = "localhost:8000";
    
    /*
        Connect to the database
    */
    let db = Surreal::new::<Ws>(host).await.unwrap();
    
    /*
        Sign in
    */
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    
    /*
        Select the namespace and the database
    */
    db.use_ns("test1").use_db("test").await.unwrap();
    
    /*
        Select all users
    */
    //insert_tests(db.clone()).await;
    let users: Vec<User<Thing>> = db.select("user").await.unwrap();
    
    /*
        Select a user from a reponse using take
    */
    let mut response = db.query("SELECT * FROM user WHERE username = $username;").bind(("username", "User1")).await.unwrap();
    let users: Option<User<Thing>> = response.take(0).unwrap();
}
