use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename="userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
    .get("https://jsonplaceholder.typicode.com/todos?userId=1")
    .send()
    .await?
    .json()
    .await?;

    println!("{:#?}", todos);
    
    println!("\n\n########\n");

    // let new_todo = Todo {
    //     user_id: 1,
    //     id: None,
    //     title: "la vida es puro dolor".to_string(),
    //     completed: true,
    // };

    let new_todo: serde_json::Value = reqwest::Client::new()
    .post("https://jsonplaceholder.typicode.com/todos")
    // .json(&new_todo)
    .json(&serde_json::json!({
        "userId":1,
        "title":"la vida es puro dolor".to_owned(),
        "completed": true
    }))
    .send()
    .await?
    .json()
    .await?;
 
    println!("{:#?}", new_todo);
    
    Ok(())
}
