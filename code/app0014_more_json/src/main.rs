use serde::{Deserialize, Serialize};
use serde_json;

use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename="userId")]
    user_id : u32,
    title: String,
    completed: bool,
}

fn main() {
    let mut file = File::create("output.txt").expect("Could not create file");
    let x: Vec<Todo> = vec![Todo{
        user_id:2u32,
        title:"Create a rust todo app".to_string(),
        completed:true
    }, Todo{
        user_id:2u32,
        title:"Create a rust todo webapp".to_string(),
        completed:false
    }, Todo{
        user_id:2u32,
        title:"Create a rust todo gui app".to_string(),
        completed:false
    }];
    let xs = serde_json::to_string(&x).unwrap();
    println!("Vec<Todo> {:#?} serializes into string {}", x, xs);
    file.write_all(&xs.into_bytes()).expect("Cannot write to the file");
    drop(file);

    let mut file = File::open("output.txt").expect("Could not open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file contents");
    let mut todos: Vec<Todo> = serde_json::from_str(&file_contents[0..]).unwrap();
    for todo in &mut todos {
        todo.completed = false;
        println!("{:#?}", todo);
    }
}
