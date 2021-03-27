use actix_files as fs;
use actix_web::web;

use crate::routes::test::test_route;

pub fn setup(config: &mut web::ServiceConfig) {
    let static_folder = std::env::var("STATIC_FOLDER").unwrap_or("./frontend/dist".to_owned());

    config
        .route("/api/test", web::get().to(test_route))
        .service(fs::Files::new("/", &static_folder).index_file("index.html"));
}
