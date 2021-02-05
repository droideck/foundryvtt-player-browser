use warp::{Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    action: i32,
}

#[derive(Clone)]
struct FoundryDB {
  actors_list: Arc<RwLock<Items>>
}

impl FoundryDB {
    fn new() -> Self {
        FoundryDB {
            actors_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn get_actors_list(
    foundrydb: FoundryDB
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let mut result = HashMap::new();
        foundrydb.actors_list.write().insert("foo".to_string(), 5);
        let r = foundrydb.actors_list.read();
    
        for (key,value) in r.iter() {
            result.insert(key, value);
        }

        Ok(warp::reply::json(
            &result
        ))
}

#[tokio::main]
async fn main() {
    let foundrydb = FoundryDB::new();
    let foundrydb_filter = warp::any().map(move || foundrydb.clone());

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("actors"))
        .and(warp::path::end())
        .and(foundrydb_filter.clone())
        .and_then(get_actors_list);
    
    let routes = get_items;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
