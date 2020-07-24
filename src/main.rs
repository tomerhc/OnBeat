use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

mod beat;

#[derive(Serialize)]
struct Doc {
    name: String,
    bla_field: HashMap<String, i32>,
    email: String,
    age: u8,
}

fn main() {
    let mut my_hash: HashMap<String, i32> = HashMap::new();
    my_hash.insert(String::from("Love"), 122);
    let my_doc = Doc {
        name: String::from("time_stamp"),
        bla_field: my_hash,
        email: String::from("blabla@gmail.com"),
        age: 1,
    };
    let index = "http://localhost:9200/test-index/_doc";
    let mtx = Arc::new(RwLock::new(my_doc));
    let my_beat = beat::Beat::new(index, mtx, Some(String::from("%d-%m-%Y %r"))).unwrap();
    println!("{}", my_beat.send().unwrap());
}

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_hash: HashMap<String, i32> = HashMap::new();
    my_hash.insert(String::from("Love"), 122);
    let my_doc = Doc {
        name: String::from("in module"),
        bla_field: my_hash,
        email: String::from("blabla@gmail.com"),
        age: 1,
    };
    let mtx = Arc::new(RwLock::new(my_doc));
    let index = "http://localhost:9200/test-index/_doc";
    let mut times = 10;
    let time_format = Some(String::from("%d-%m-%Y %r"));
    let t = beat::monitor(
        mtx.clone(),
        Duration::from_secs(1),
        index,
        time_format,
        times,
    );
    let m = thread::spawn(move || {
        while times > 0 {
            thread::sleep(Duration::from_secs(1));
            let mut lock = mtx.write().unwrap();
            lock.age += 1;
            times -= 1;
        }
    });

    t.join().unwrap();
    m.join().unwrap();
    Ok(())
}
