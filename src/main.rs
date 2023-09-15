use actix_web::{get,web, web::ServiceConfig , HttpResponse, Responder};
use shuttle_actix_web::ShuttleActixWeb;


fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[get("/fibonacci/{n}")]
async fn calculate_fibonacci(path: web::Path<u32>) -> impl Responder {
    let n = path.into_inner();
    let result = fibonacci(n);
    HttpResponse::Ok().body(format!("Fibonacci({}) = {}", n, result))
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(calculate_fibonacci);
    };

    Ok(config.into())
}
