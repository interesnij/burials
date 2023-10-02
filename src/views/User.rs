config.route("/registry/", web::get().to(registry_page));
config.route("/login/", web::get().to(login_page));
config.route("/user/", web::get().to(user_page));