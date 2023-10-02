config.route("/deceased{id}/", web::get().to(deceased_page));
config.route("/all_deceased_place{id}/", web::get().to(all_deceased_place_page));