use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};


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