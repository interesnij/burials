config.route("/place{id}/", web::get().to(place_page));


config.route("/all_places_city{id}/", web::get().to(all_places_city_page));
config.route("/all_places_region{id}/", web::get().to(all_places_region_page));
config.route("/all_places_country{id}/", web::get().to(all_places_country_page));