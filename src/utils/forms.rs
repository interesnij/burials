use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};
 
//----------------------------ФОРМА ПОКОЙНИКОВ--------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct DeceasedForms {
    pub first_name: String,        // Имя усопшего
    pub last_name: String,         // Фамилия усопшего
    pub middle_name: Option<String>,// Отчество усопшего (может быть отсутствовать)
    pub birth_date: String,     // Дата рождения усопшего
    pub death_date: String,     // Дата смерти усопшего
    pub main_image: Option<String>, // Ссылка на фото усопшего (может быть отсутствовать)
    pub description: Option<String>,       // Дополнительные данные (description) (может быть отсутствовать)
    pub memory_words: Option<String>// Слова памяти (комментарии) (может быть отсутствовать)
    pub slug:          String,
    pub position:      i16,
    pub types:         i16,
}
// форма для элементов 
pub async fn deceased_form(payload: &mut Multipart, owner_id: i32) -> DeceasedForms {
    let mut form: DeceasedForms = DeceasedForms {
        first_name:         "".to_string(),
        last_name:         "".to_string(),
        middle_name:         "".to_string(),
        birth_date: "".to_string(),
        death_date: "".to_string(),
        main_image:    None,
        description:   None,       
        memory_words: Vec::new(),
        slug:          "".to_string(),
        position:      0,
        types:         0,
    };

   

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["first_name", "last_name","middle_name", "birth_date", "death_date", "description", "memory_words", "slug"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    } else if field.name() == "last_name" {
                        form.last_name = Some(data_string);
                    } else if field.name() == "middle_name" {
                        form.middle_name = Some(data_string);
                    } else if field.name() == "birth_date" {
                        form.birth_date = data_string;
                    } else if field.name() == "death_date" {
                        form.death_date = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "memory_words" {
                        form.memory_words = (data_string);
                    } else if field.name() == "slug" {
                        form.slug = data_string;
                    }
                }
            }
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }

        else if name == "main_image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.main_image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//---------------------------------ФОРМА КЛАДБИЩЬ-----------------
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaceForms {
    pub cemetery_name: String,               // Название кладбища (места)
    pub address: String,             // Адрес кладбища
    pub cemetery_director: Option<String>,    // Руководитель кладбища
    pub summer_hours: Option<String>,        // Часы работы в летнее время
    pub winter_hours: Option<String>,        // Часы работы в зимнее время
    pub main_image: Option<String>, // Ссылки на фотографии кладбища
    pub description: Option<String>,         // Описание кладбища
    pub burial_count: i32,                   // Количество захоронений
    pub cemetery_phone_number: Option<String>,// Номер телефона кладбища
    pub city_id: i32,                        // Идентификатор города, в котором находится место
    pub region_id: Option<i32>,                      // Идентификатор региона, к которому принадлежит место
    pub country_id: i32,                     // Идентификатор страны, к которой принадлежит место
    pub slug:          String,
    pub position:      i16,
    pub types:         i16,
}
// форма для элементов 
pub async fn place_form(payload: &mut Multipart, owner_id: i32) -> DeceasedForms {
    let mut form: PlaceForms = PlaceForms {
        cemetery_name:         "".to_string(),
        address:         "".to_string(),
        cemetery_director: None,
        summer_hours: None,
        winter_hours: None,
        main_image:    None,
        description:   None,  
        burial_count:  0,    
        cemetery_phone_number: None,
        city_id: 0,
        region_id: None,
        country_id: 0,
        slug:          "".to_string(),
        position:      0,
        types:         0,
    };

   

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["cemetery_name", "address","cemetery_director", "summer_hours", "winter_hours", "description", "cemetery_phone_number", "slug"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "cemetery_name" {
                        form.cemetery_name = data_string;
                    } else if field.name() == "address" {
                        form.address = data_string;
                    } else if field.name() == "cemetery_director" {
                        form.cemetery_director = Some(data_string);
                    } else if field.name() == "summer_hours" {
                        form.summer_hours = Some(data_string);
                    } else if field.name() == "winter_hours" {
                        form.winter_hours = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "cemetery_phone_number" {
                        form.cemetery_phone_number = Some(data_string);
                    } else if field.name() == "slug" {
                        form.slug = data_string;
                    }
                }
            }
        }

        else if name == "burial_count" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.burial_count = _int;
                }
            }
        }

        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = _int;
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = _int;
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }

        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }

        else if name == "main_image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.main_image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//------------------------------ФОРМА ОРГАНИЗАЦИЙ----------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct OrganizationForms {
    pub name: String,               // Название организации
    pub description: Option<String>,         // Описание организации
    pub director: String,    // Руководитель организации
    pub phone_number: String,        // Номер телефона организации
    pub working_hours: Option<String>,        // Часы работы 
    pub main_image: Option<String>, // Ссылки на фотографию
    pub website: Option<String>,     // Веб-сайт организации (может быть пустым)
    pub messenger_links: Option<String>, // Ссылки на мессенджеры организации
    pub city_id: i32,                        // Идентификатор города, в котором находится организация
    pub region_id: Option<i32>,                      // Идентификатор региона, к которому принадлежит организация
    pub country_id: i32,                     // Идентификатор страны, к которой принадлежит место
    pub slug:          String,
    pub position:      i16,
    pub types:         i16,
}
// форма для элементов 
pub async fn organization_form(payload: &mut Multipart, owner_id: i32) -> DeceasedForms {
    let mut form: PlaceForms = PlaceForms {
        name:         "".to_string(),
        description:         None,
        director: "".to_string(),
        phone_number: "".to_string(),
        working_hours: None,
        main_image:    None,
        website:   None,  
        messenger_links:   None,  
        city_id: 0,
        region_id: None,
        country_id: 0,
        slug:          "".to_string(),
        position:      0,
        types:         0,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name", "messenger_links","director", "website", "working_hours", "description", "phone_number", "slug"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    } else if field.name() == "messenger_links" {
                        form.messenger_links = Some(data_string);
                    } else if field.name() == "director" {
                        form.director = data_string;
                    } else if field.name() == "website" {
                        form.website = Some(data_string);
                    } else if field.name() == "working_hours" {
                        form.working_hours = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "phone_number" {
                        form.phone_number = data_string;
                    } else if field.name() == "slug" {
                        form.slug = data_string;
                    }
                }
            }
        }


        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = _int;
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = _int;
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }

        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }

        else if name == "main_image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.main_image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//------------------------------ФОРМА УСЛУГ-----------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceForms {
    pub name_service: String,                // Название Услуги
    pub description: Option<String>,         // Описание услуги
    pub main_image: Option<String>,          // Ссылка на фотографию
    pub price: f64,                          // Цена за услугу
    pub organization_id: i32,                // Идентификатор организации, предоставляющей услугу
    pub city_id: i32,                        // Идентификатор города, в котором предоставляется услуга
    pub slug:          String,
    pub position:      i16,
    pub types:         i16,
}
// форма для элементов 
pub async fn service_form(payload: &mut Multipart, owner_id: i32) -> DeceasedForms {
    let mut form: PlaceForms = PlaceForms {
        name_service:         "".to_string(),
        description:         None,
        main_image:    None,
        price: 0,
        city_id: 0,
        slug:          "".to_string(),
        position:      0,
        types:         0,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name_service", "description", "slug"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name_service" {
                        form.name_service = data_string;
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "slug" {
                        form.slug = data_string;
                    }
                }
            }
        }


        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = _int;
                }
            }
        }
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: f64 = s.parse().unwrap();
                    form.region_id = _int;
                }
            }
        }

        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }

        else if name == "main_image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.main_image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}




//-----------------------------------------------------------
//-----------------------------------------------------------
//-----------------------------------------------------------
//-----------------------------------------------------------
//-----------------------------------------------------------







//------------------------------ФОРМА ЗАКАЗОВ-----------------

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderForms {
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub description: Option<String>,
    pub email:       String,
    pub files:       Vec<String>,
    pub serve_list:  Vec<i32>,
}
// форма для заказов
pub async fn order_form(payload: &mut Multipart, owner_id: i32) -> OrderForms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: OrderForms = OrderForms {
        title:       "".to_string(),
        types:       0,
        object_id:   0,
        username:    "".to_string(),
        description: None,
        email:       "".to_string(),
        files:       Vec::new(),
        serve_list:  Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "email", "description", "username"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "email" {
                        form.email = data_string;
                    } else if field.name() == "username" {
                        form.username = data_string;
                    }
                }
            }
        }
        else if name == "object_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.object_id = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }
        else if name == "serve_list" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let v: Vec<&str> = data_string.split(",").collect();
                    for i in v.iter() {
                        let _int: i32 = i.parse().unwrap();
                        form.serve_list.push(_int);
                    }
                }
            }
        }
        else if name == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                files.push(file.clone());
                form.files.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}


//------------------------------ФОРМА ФАЙЛОВ-----------------

#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm {
    pub item_types: i16,      // блог, услуга ......
    pub types:      i16,      // фото, видео, аудио ......
    pub files:      Vec<String>,
}
pub async fn files_form(payload: &mut Multipart, owner_id: i32) -> FileForm {
    let mut _files: Vec<UploadedFiles> = Vec::new();

    let mut form: FileForm = FileForm {
        item_types: 0,
        types:      0,
        files:      Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                _files.push(file.clone());
                form.files.push(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "item_types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.item_types = _int;
                }
            }
        }
        else if field.name() == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }
    }
    form
}


//------------------------------ФОРМА ОБРАТНОЙ СВЯЗИ-----------------

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedbackForm {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
pub async fn feedback_form(payload: &mut Multipart) -> FeedbackForm {
    let mut form: FeedbackForm = FeedbackForm {
        username: "".to_string(),
        email:    "".to_string(),
        message:  "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "email" {
                    form.email = data_string
                } else if field.name() == "message" {
                    form.message = data_string
                }
            }
        }
    }
    form
} 
