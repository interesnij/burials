-- Создание таблицы для хранения существующих записей об усопших
CREATE TABLE IF NOT EXISTS deceased (
    id SERIAL PRIMARY KEY,              -- Уникальный идентификатор записи
    place_id INT,                       -- Идентификатор места
    first_name VARCHAR(255) NOT NULL,   -- Имя усопшего (обязательное поле)
    last_name VARCHAR(255) NOT NULL,    -- Фамилия усопшего (обязательное поле)
    middle_name VARCHAR(255),           -- Отчество усопшего
    birth_date DATE NOT NULL,           -- Дата рождения (обязательное поле)
    death_date DATE NOT NULL,           -- Дата смерти (обязательное поле)
    photo_link VARCHAR(255),            -- Ссылка на фотографию усопшего
    data TEXT,                          -- Дополнительные данные
    memory_words TEXT                   -- Памятные слова
);

--______________________________________________________________________________________

-- Создание таблицы для хранения стран (Countries)
CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,              -- Уникальный идентификатор
    name VARCHAR(255) NOT NULL,         -- Название страны (обязательное поле)
    geo_id INT NOT NULL                -- Идентификатор географического местоположения (обязательное поле)
);

-- Создание таблицы для хранения регионов (Regions)
CREATE TABLE IF NOT EXISTS regions (
    id SERIAL PRIMARY KEY,              -- Уникальный идентификатор
    name VARCHAR(255) NOT NULL,         -- Название региона (обязательное поле)
    geo_id INT NOT NULL,                -- Идентификатор географического местоположения (обязательное поле)
    country_id INT NOT NULL,            -- Идентификатор страны, к которой принадлежит регион (обязательное поле)
    FOREIGN KEY (country_id) REFERENCES countries(id) -- Ссылка на таблицу countries
);

-- Создание таблицы для хранения городов (Citys)
CREATE TABLE IF NOT EXISTS citys (
    id SERIAL PRIMARY KEY,              -- Уникальный идентификатор
    name VARCHAR(255) NOT NULL,         -- Название города (обязательное поле)
    geo_id INT NOT NULL,                -- Идентификатор географического местоположения (обязательное поле)
    region_id INT NOT NULL,             -- Идентификатор региона, к которому принадлежит город (обязательное поле)
    country_id INT NOT NULL,            -- Идентификатор страны, к которой принадлежит город (обязательное поле)
    lat DOUBLE PRECISION NOT NULL,      -- Широта местоположения
    lon DOUBLE PRECISION NOT NULL,      -- Долгота местоположения
    FOREIGN KEY (region_id) REFERENCES regions(id), -- Ссылка на таблицу regions
    FOREIGN KEY (country_id) REFERENCES countries(id) -- Ссылка на таблицу countries
);

--______________________________________________________________________________________

-- Создание таблицы "organizations" для хранения данных об организации
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,            -- Уникальный идентификатор организации
    name VARCHAR(255) NOT NULL,       -- Название организации
    description TEXT NOT NULL,        -- Описание организации
    director VARCHAR(255) NOT NULL,   -- Руководитель организации
    phone_number VARCHAR(20) NOT NULL,-- Номер телефона организации
    place_id INTEGER NOT NULL,        -- Идентификатор места организации
    city_id INTEGER NOT NULL,         -- Идентификатор города организации
    region_id INTEGER NOT NULL,       -- Идентификатор региона организации
    country_id INTEGER NOT NULL,      -- Идентификатор страны организации
    service_ids INTEGER[] NOT NULL,   -- Идентификаторы предоставляемых услуг
    working_hours VARCHAR(100) NOT NULL, -- Время работы организации
    website VARCHAR(255),             -- Веб-сайт организации (может быть пустым)
    photo_link VARCHAR(255),          -- Ссылка на фотографию организации (может быть пустой)
    messenger_links TEXT[] NOT NULL,  -- Ссылки на мессенджеры организации
    location_coordinates POINT         -- Координаты расположения организации
);

-- Создание индекса для ускорения поиска по идентификатору организации
CREATE INDEX idx_organization_id ON organizations (id);

--______________________________________________________________________________________

-- Создание таблицы "Places" для хранения данных о местах
CREATE TABLE IF NOT EXISTS Places (
    id SERIAL PRIMARY KEY,            -- Уникальный идентификатор места
    city_id INT NOT NULL,             -- Идентификатор города, в котором находится место
    region_id INT NOT NULL,           -- Идентификатор региона, к которому принадлежит место
    country_id INT NOT NULL,          -- Идентификатор страны, к которой принадлежит место
    cemetery_name TEXT NOT NULL,      -- Название кладбища (места)
    description TEXT,                 -- Описание кладбища
    summer_hours TEXT,                -- Часы работы в летнее время
    winter_hours TEXT,                -- Часы работы в зимнее время
    photo_links TEXT,                 -- Ссылки на фотографии кладбища
    address TEXT,                     -- Адрес кладбища
    deceased_id INT NOT NULL,         -- Идентификатор покойников
    organization_id INT NOT NULL,     -- Идентификатор организаций
    burial_count INT NOT NULL,        -- Количество захоронений
    cemetery_director TEXT,           -- Руководитель кладбища
    cemetery_phone_number TEXT        -- Номер телефона кладбища
);

--______________________________________________________________________________________
-- Создание таблицы "Review" для хранения данных об отзывах
CREATE TABLE IF NOT EXISTS Review (
    id SERIAL PRIMARY KEY,             -- Уникальный идентификатор отзыва
    service_id INT NOT NULL,           -- Идентификатор услуги, на которую составлен отзыв
    user_id INT NOT NULL,              -- Идентификатор автора отзыва
    review_date DATE NOT NULL,         -- Дата отзыва (используется DATE для хранения даты без времени)
    review_text TEXT NOT NULL          -- Текст отзыва
);

--______________________________________________________________________________________
-- Создание таблицы "Service" для хранения данных о услугах
CREATE TABLE IF NOT EXISTS Service (
    id SERIAL PRIMARY KEY,               -- Уникальный идентификатор услуги
    organization_id INT NOT NULL,        -- Идентификатор организации, предоставляющей услугу
    description TEXT NOT NULL,           -- Описание услуги
    photo_link TEXT,                     -- Ссылка на фотографию услуги
    price DOUBLE PRECISION NOT NULL,     -- Цена за услугу (используется DOUBLE PRECISION для хранения числа с плавающей точкой)
    city_id INT NOT NULL,                -- Идентификатор города, где предоставляется услуга
    review_ids INT[]                    -- Массив идентификаторов отзывов об услуге
);
--______________________________________________________________________________________
-- Создание таблицы "User" для хранения данных о пользователях
CREATE TABLE IF NOT EXISTS "User" (
    id SERIAL PRIMARY KEY,               -- Уникальный идентификатор
    first_name TEXT NOT NULL,            -- Имя пользователя
    last_name TEXT NOT NULL,             -- Фамилия пользователя
    middle_name TEXT,                    -- Отчество пользователя
    email TEXT NOT NULL UNIQUE,          -- Адрес электронной почты (уникальный)
    phone_number TEXT,                   -- Номер телефона пользователя
    description TEXT,                    -- Описание пользователя
    photo_link TEXT,                     -- Ссылка на фотографию пользователя
    deceased_ids INT[]                   -- Массив идентификаторов зарегистрированных усопших
);
--______________________________________________________________________________________
-- Создание таблицы "Comment" для хранения данных о комментариях
CREATE TABLE IF NOT EXISTS Comment (
    id SERIAL PRIMARY KEY,                  -- Уникальный идентификатор комментария
    deceased_id INT NOT NULL,               -- Идентификатор усопшего, кому предназначен комментарий
    user_id INT NOT NULL,                   -- Идентификатор автора комментария
    comment_text TEXT NOT NULL,             -- Текст комментария
    comment_date TIMESTAMP WITHOUT TIME ZONE NOT NULL -- Дата комментария (используется TIMESTAMP без временной зоны)
);
