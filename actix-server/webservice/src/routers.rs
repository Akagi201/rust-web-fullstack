use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
  cfg
  // .service(web::scope("/courses"))
  .route("/courses/", web::post().to(new_course))
  .route("/courses/{user_id}", web::get().to(get_courses_for_teacher))
  .route("/courses/{user_id}/{course_id}", web::get().to(get_course_detail));
}