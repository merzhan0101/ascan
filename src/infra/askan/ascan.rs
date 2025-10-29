use actix_web::{get, post, web, HttpResponse, Responder};
use crate::structures::ascan_struct::{DataAscan,DataFilter, SelectWhee};
use crate::data_base::data_base_ascan::ascan_db as db;
#[get("/ascan")]
async fn ascan_get_data()->impl Responder{
    
    let data = DataAscan::new().await.unwrap();

    // println!("1");
    HttpResponse::Ok().json(data)
}

#[post("/api/wheel")]
async fn ascan_wheel(data: web::Json<DataFilter>)->impl Responder{
    // println!("{:?}", data);
    let wheel_list =  if data.malting.is_none(){
        db::data_wheel_for_batch(data.into_inner().clone()).await.unwrap()
    }else{
        db::data_wheel_for_malting(data.into_inner()).await.unwrap()
    };

    HttpResponse::Ok().json(wheel_list)
}

// #[post("/api/wheel/data")]
// async fn ascan_data_wheel(data: web::Json<SelectWhee>)->impl Responder{
//     // println!("1");

//     let wheel_data = if data.malting_number.is_none(){
//         db::data_for_wheel_select_batch(data.into_inner()).await.unwrap()
//     }else{
//         db::data_for_wheel_select_malting(data.into_inner()).await.unwrap()
//     };
    
//     // db::data_for_wheel_select(data.into_inner()).await.unwrap();
//     // let wheel_data = db::data_wheel_test(data.into_inner()).await.unwrap();

//     // println!("{:?}", wheel_data);
//     HttpResponse::Ok().json(wheel_data)
// }

pub fn conf_ascan(cfg: &mut web::ServiceConfig){
    cfg.service(ascan_get_data);
    cfg.service(ascan_wheel);
    // cfg.service(ascan_data_wheel);
}

