use rocket::Rocket;
use rocket::Build;
use rocket::get;
use rocket::launch;
use rocket::routes;
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
