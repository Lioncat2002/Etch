use rocket::futures::lock::Mutex;
use rocket::get;
use rocket::launch;
use rocket::response::content;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use rocket::State;
use std::collections::HashMap;
#[derive(Debug)]
struct DB {
    db: Mutex<HashMap<String, String>>,
}
impl DB {
    fn new() -> Self {
        let db = Mutex::new(HashMap::new());
        Self { db }
    }
    async fn insert(&self, key: String, value: String) {
        let mut lock = self.db.lock().await;
        lock.insert(key, value);
    }
    async fn get(&self, key: String) -> Option<String> {
        let lock = self.db.lock().await;
        lock.get(&key).or(Some(&"404".to_string())).cloned()
    }
}
#[get("/set/<key>/<value>")]
async fn index(key: String, value: String, db: &State<DB>) -> content::RawJson<String> {
    db.insert(key, value).await;
    content::RawJson("{ 'status' : 'ok' }".to_string())
}

#[get("/get/<key>")]
async fn get(key: String, db: &State<DB>) -> content::RawJson<String> {
    let data = db.get(key).await.unwrap();
    content::RawJson(format!("{{ 'status' : 'ok', 'data' : '{data}' }}"))
}

#[launch]
fn rocket() -> Rocket<Build> {
    let db = DB::new();
    rocket::build().manage(db).mount("/", routes![index, get])
}
