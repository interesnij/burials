config.route("/", web::get().to(index_page));
config.route("/about/", web::get().to(about_page));
config.route("/faq/", web::get().to(faq_page));



config.route("/add_deceased_place{id}/", web::get().to(add_deceased_page));
config.route("/add_place_city{id}/", web::get().to(add_place_page));
config.route("/add_organization/", web::get().to(add_organization_page));
config.route("/add_service_organization{id}/", web::get().to(add_service_page));








