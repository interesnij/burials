use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use chrano::NaiveDate;
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};
use crate::utils::DataOrganizationsPlace;


#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String, 
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/{}/{}/{}/",
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        // вариант для https
        let create_path = format_folder.replace("./", "/my/");
        // вариант для debug
        //let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeceasedForms {
    pub first_name:   String,
    pub middle_name:  Option<String>,
    pub last_name:    String,
    pub birth_date:   NaiveDate,
    pub death_date:   NaiveDate,
    pub image:        Option<String>,
    pub memory_words: Option<String>
    pub lat           f64,
    pub lon           f64,
}
// форма для элементов 
pub async fn deceased_form(payload: &mut Multipart, owner_id: i32) -> DeceasedForms {
    let mut form: DeceasedForms = DeceasedForms {
        first_name:   "".to_string(),
        middle_name:  None,
        last_name:    "".to_string(),
        birth_date:   NaiveDate::from_ymd(2021, 1, 1),
        death_date:   NaiveDate::from_ymd(2021, 1, 1),
        image:        None,
        memory_words: None,
        lat:          0.0,
        lon:          0.0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["first_name", "last_name","middle_name", "memory_words"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    } else if field.name() == "last_name" {
                        form.last_name = data_string;
                    } else if field.name() == "middle_name" {
                        form.middle_name = Some(data_string);
                    } else if field.name() == "memory_words" {
                        form.memory_words = Some(data_string);
                    }
                }
            }
        }
        else if name == "lat" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: f64 = s.parse().unwrap();
                    form.lat = Some(_int);
                }
            }
        }
        else if name == "lon" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: f64 = s.parse().unwrap();
                    form.lon = Some(_int);
                }
            }
        }
        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
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
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//---------------------------------ФОРМА КЛАДБИЩЬ-----------------
#[derive(Deserialize, Serialize, Debug)] 
pub struct PlaceForms {
    pub city_id:     i32,
    pub region_id:   Option<i32>,
    pub country_id:  i32,
    pub title:       String,
    pub description: Option<String>,
    pub hours:       Option<String>,
    pub image:       Option<String>,
    pub address:     Option<String>,
    pub director:    Option<String>,
    pub phone:       Option<String>,
    pub lat          f64,
    pub lon          f64,
}
// форма для элементов 
pub async fn place_form(payload: &mut Multipart, owner_id: i32) -> PlaceForms {
    let mut form: PlaceForms = PlaceForms {
        city_id:     0,
        region_id:   None,
        country_id:  0,
        title:       "".to_string(),
        description: None,
        hours:       None,
        image:       None,
        address:     "".to_string(),
        director:    None,
        phone:       None,
        lat:         0.0,
        lon:         0.0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "address", "director", "hours", "description", "phone"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "address" {
                        form.address = Some(data_string);
                    } else if field.name() == "director" {
                        form.director = Some(data_string);
                    } else if field.name() == "hours" {
                        form.hours = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "phone" {
                        form.phone = Some(data_string);
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
                    form.region_id = Some(_int);
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

        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
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
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//------------------------------ФОРМА ОРГАНИЗАЦИЙ----------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct OrganizationForms {
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub places:      Vec<DataOrganizationsPlace>
}
// форма для элементов 
pub async fn organization_form(payload: &mut Multipart, owner_id: i32) -> OrganizationForms {
    let mut form: OrganizationForms = OrganizationForms {
        name:        "".to_string(),
        description: "".to_string(),
        director:    "".to_string(),
        phone:       "".to_string(),
        hours:       "".to_string(),
        website:     None, 
        image:       None,
        places:      Vec::new(),  
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name","director", "website", "hours", "description", "phone"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    } else if field.name() == "director" {
                        form.director = data_string;
                    } else if field.name() == "website" {
                        form.website = Some(data_string);
                    } else if field.name() == "hours" {
                        form.hours = data_string;
                    } else if field.name() == "description" {
                        form.description = data_string;
                    } else if field.name() == "phone" {
                        form.phone = data_string;
                    }
                }
            }
        }
        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
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
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
        else if name == "places" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    form.places.push(s);
                }
            }
        }
    }
    form
}

//------------------------------ФОРМА УСЛУГ-----------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceForms {
    pub city_id:     i32,
    pub title:       String,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub price:       i32,
}
// форма для элементов 
pub async fn service_form(payload: &mut Multipart, owner_id: i32) -> ServiceForms {
    let mut form: ServiceForms = ServiceForms {
        city_id:     0,
        title:       "".to_string(),
        description: None,
        image:       None,
        price:       0,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "description"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
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

        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
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
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilesForm {
    pub files: Vec<String>,
}
pub async fn files_form(payload: &mut Multipart) -> FilesForm {
    let mut form: FilesForm = FilesForm {
        files: Vec::new(),
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
                form.files.push(file.path.clone().replace("./","/"));
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
