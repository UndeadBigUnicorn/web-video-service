use {
    actix_web::{middleware, web, App, HttpServer},
    std::{env, net::SocketAddr},
    web_service::video_client::VideoClient,
    tera::Tera,
    env_logger,
};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    std::fs::create_dir_all("./tmp").unwrap();

    let addr = env::args().nth(1).unwrap_or("localhost:8092".to_string());
    println!("Server is running on http://{}", addr);

    // address of video-service
    let remote_adr: SocketAddr = env::args()
        .nth(2)
        .unwrap_or("127.0.0.1:8091".into())
        .parse()
        .expect("Remote adress structure is not valid");

    // create new client
    let video_client = web::Data::new(VideoClient::new(remote_adr));

    // TODO: implement those error handlers
    // let error_handlers = middleware::errhandlers::ErrorHandlers::new()
    //         .handler(
    //             http::StatusCode::INTERNAL_SERVER_ERROR,
    //             api::internal_server_error,
    //         )
    //         .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
    //         .handler(http::StatusCode::NOT_FOUND, api::not_found);

    HttpServer::new(move || {

        let tera =
        Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();


        App::new()
            .data(tera)
            .wrap(middleware::Logger::default())
            // TODO: uncomment once implement error handlers
            // .wrap(error_handlers)
            .service(
                web::scope("/video")
                    // dependency injection of video client
                    .app_data(video_client.clone())
                    .route("/upload", web::post().to(api::save_file))
                    .route("/", web::get().to(api::show_video))
                    .route("/{filename}", web::get().to(api::get_file)))
            .service(
                web::resource("/")
                    .route(web::get().to(api::index)),   
            )
    })
    .bind(addr)?
    .start()
    .await
}