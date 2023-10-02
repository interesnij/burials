use diesel::prelude::*;
use chrono::NaiveDate;

// Структура для хранения данных об усопшем
pub struct Deceased {
    pub id: i32,                   // Уникальный идентификатор
    pub place_id: i32,             // Идентификатор места захоронения
    pub first_name: String,        // Имя усопшего
    pub last_name: String,         // Фамилия усопшего
    pub middle_name: Option<String>,// Отчество усопшего (может быть отсутствовать)
    pub birth_date: NaiveDate,     // Дата рождения усопшего
    pub death_date: NaiveDate,     // Дата смерти усопшего
    pub photo_link: Option<String>, // Ссылка на фото усопшего (может быть отсутствовать)
    pub data: Option<String>,       // Дополнительные данные (description) (может быть отсутствовать)
    pub memory_words: Option<String>// Слова памяти (комментарии) (может быть отсутствовать)
}

// Структура для создания новых записей об усопших
pub struct NewDeceased {
    pub place_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub birth_date: NaiveDate,
    pub death_date: NaiveDate,
    pub photo_link: Option<String>,
    pub data: Option<String>,
    pub memory_words: Option<String>
}
