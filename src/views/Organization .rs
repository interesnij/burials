config.route("/all_organizatios_place{id}/", web::get().to(all_organizations_place_page));
config.route("/all_organizatios_city{id}/", web::get().to(all_organizations_city_page));
config.route("/all_organizatios_region{id}/", web::get().to(all_organizations_region_page));
config.route("/all_organizatios_country{id}/", web::get().to(all_organizations_country_page));


config.route("/all_service_organization{id}/", web::get().to(all_service_organizations_page));
config.route("/all_service_place{id}/", web::get().to(all_service_place_page));