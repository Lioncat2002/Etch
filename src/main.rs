use rocket::get;
use rocket::launch;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
#[get("/bruh")]
fn index() -> &'static str {
    println!("YOoo");
    "Hello World!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
    //3rocket::build().mount("/", routes![index])
}
