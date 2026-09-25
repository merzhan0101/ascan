use actix_web::{get, post, web, HttpResponse, Responder};
use crate::structures::ascan_struct::{DataAscan, DataFilter};
use crate::data_base::data_base_ascan::ascan_db as db;

/// Списки партий и плавок. Всегда отвечает 200: недоступные линии перечислены в `errors`.
#[get("/ascan")]
async fn ascan_get_data()->impl Responder{
    HttpResponse::Ok().json(DataAscan::new().await)
}

/// Колёса по партии или плавке. Всегда отвечает 200 (кроме неверного запроса):
/// у каждой линии свой результат — данные или текст ошибки.
#[post("/api/wheel")]
async fn ascan_wheel(data: web::Json<DataFilter>)->impl Responder{
    let filter = data.into_inner();
    let (by_malting, value) = match (filter.malting, filter.batch_number) {
        (Some(m), _) if !m.trim().is_empty() => (true, m),
        (_, Some(b)) if !b.trim().is_empty() => (false, b),
        _ => return HttpResponse::BadRequest().body("Нужно указать batch_number или malting"),
    };

    HttpResponse::Ok().json(db::wheels(&filter.place, by_malting, value.trim()).await)
}

pub fn conf_ascan(cfg: &mut web::ServiceConfig){
    cfg.service(ascan_get_data);
    cfg.service(ascan_wheel);
}
