mod models {
    use parking_lot::RwLock;
    use std::sync::Arc;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    type Items = HashMap<String, i32>;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Id {
        pub name: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Item {
        pub name: String,
        pub quantity : i32,
    }

    #[derive(Clone)]
    pub struct Store {
        pub grocery_list: Arc<RwLock<Items>>
    }

    impl Store {
        pub fn new() -> Self {
            Store {
                grocery_list: Arc::new(RwLock::new(HashMap::new())),
                
            }
        }
    }
}

mod handlers {
    use warp::{http, Filter};
    use super::models::{Item, Store, Id};

    pub async fn add_grocery_list_item(
        item: Item,
        store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().insert(item.name, item.quantity);

        Ok(warp::reply::with_status(
            "Added items to the grocery list",
            http::StatusCode::CREATED,
        ))
    }

    pub async fn get_grocery_list(
        store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        use std::collections::HashMap;

        let mut result = HashMap::new();
        let r = store.grocery_list.read();

        for (key,value) in r.iter() {
            result.insert(key, value);
        }
        Ok(warp::reply::json(
            &result
        ))
    }

    /* 
        This is duplicate add_grocery_list_item 
        because of giving more example would help
        someone clearify the working of function.
    */
    pub async fn update_grocery_list_item(
        item: Item,
        store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().insert(item.name, item.quantity);

        Ok(warp::reply::with_status(
             "Added items to the grocery list", 
            http::StatusCode::CREATED,
        ))
    }

    pub async fn delete_grocery_list_item(
        id: Id,
        store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        store.grocery_list.write().remove(&id.name);

        Ok(warp::reply::with_status(
            "Removed item from grocery", 
            http::StatusCode::OK,
        ))
    }

    pub fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    pub fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

}

#[tokio::main]
async fn main() {
    use warp::{Filter};

    let store = models::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    #[allow(dead_code)]
    let _hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(handlers::json_body())
        .and(store_filter.clone())
        .and_then(handlers::add_grocery_list_item);

    let get_items = warp::get()
    .and(warp::path("v1"))
    .and(warp::path("groceries"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(handlers::get_grocery_list);

    let update_item = warp::put()
    .and(warp::path("v1"))
    .and(warp::path("groceries"))
    .and(warp::path::end())
    .and(handlers::json_body())
    .and(store_filter.clone())
    .and_then(handlers::update_grocery_list_item);

    let delete_item = warp::delete()
    .and(warp::path("v1"))
    .and(warp::path("groceries"))
    .and(warp::path::end())
    .and(handlers::delete_json())
    .and(store_filter.clone())
    .and_then(handlers::delete_grocery_list_item);

    let routes = add_items.or(_hello).or(get_items).or(update_item).or(delete_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

