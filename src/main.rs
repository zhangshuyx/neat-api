use warp::{http, Filter};
use parking_lot::RwLock;
use std::sync::Arc;

struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Item>>
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn add_grocery_list_item(
    item: Item,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().insert(item.name, item.quantity);

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
    
#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    warp::serve(add_items)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
