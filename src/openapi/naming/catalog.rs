use actix::Addr;
use actix_web::{get, HttpResponse, Responder, Scope, web};
use actix_web::http::header;

use crate::naming::core::{NamingActor, NamingCmd, NamingResult};
use crate::naming::ops::ops_model::{OpsServiceDto, OpsServiceOptQueryListResponse, OpsServiceQueryListRequest};

pub(super) fn service() -> Scope {
    web::scope("/catalog")
        .service(query_opt_service_list)
}

#[get("/services")]
pub async fn query_opt_service_list(
    param: web::Query<OpsServiceQueryListRequest>,
    naming_addr: web::Data<Addr<NamingActor>>,
) -> impl Responder {
    let serivce_param = param.0.to_param().unwrap();
    match naming_addr
        .send(NamingCmd::QueryServiceInfoPage(serivce_param))
        .await
    {
        Ok(res) => {
            let result: NamingResult = res.unwrap();
            if let NamingResult::ServiceInfoPage((size, list)) = result {
                let service_list: Vec<OpsServiceDto> =
                    list.into_iter().map(OpsServiceDto::from).collect::<_>();
                let response = OpsServiceOptQueryListResponse::new(size as u64, service_list);
                let v = serde_json::to_string(&response).unwrap();
                HttpResponse::Ok()
                    .insert_header(header::ContentType(mime::APPLICATION_JSON))
                    .body(v)
            } else {
                HttpResponse::InternalServerError().body("naming result error")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("system error"),
    }
}