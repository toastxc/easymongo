///# Mongo
/// Mongo contains a simple and easy builder pattern for constructing databases
/// ## Example
///```
///async fn example() {
///   let db = easymongo::mongo::Mongo::new()
///      .username("hello")
///      .password("hello")
///      .db_generate()
///      .await;
///   println!("Database successfully connnected");
///}
///```
pub mod mongo;
