
-- Таблица для хранения данных о пользователях
CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(100) NOT NULL,
    email       VARCHAR(100) NOT NULL,
    password    VARCHAR(100) NOT NULL,
    description TEXT,
    image       VARCHAR(100),
    perm        SMALLINT NOT NULL
); 

-- Создание таблицы для хранения существующих записей об усопших
/*
types
1  покойник предложен
2  покойник одобрен
*/
CREATE TABLE deceaseds (
    id           SERIAL PRIMARY KEY,     -- Уникальный идентификатор записи
    user_id      INT NOT NULL,
    place_id     INT NOT NULL,           -- Идентификатор места
    first_name   VARCHAR(100) NOT NULL,  -- Имя усопшего (обязательное поле)
    middle_name  VARCHAR(100),           -- Отчество усопшего
    last_name    VARCHAR(100) NOT NULL,  -- Фамилия усопшего (обязательное поле)
    birth_date   DATE NOT NULL,          -- Дата рождения (обязательное поле)
    death_date   DATE NOT NULL,          -- Дата смерти (обязательное поле)
    image        VARCHAR(100),           -- Ссылка на фотографию усопшего
    memory_words VARCHAR(500),           -- Памятные слова
    lat          FLOAT NOT NULL,
    lon          FLOAT NOT NULL,
    types        INT NOT NULL
);

/*
    Файлы для прикрепления к объектам. Наприммер, универсально подойдет для галереи покойника.
    
    object_types:
    | 1 Организация
    | 2 Кладбище
    | 3 Покойник
*/
CREATE TABLE files (
    id           SERIAL PRIMARY KEY,
    object_id    INT NOT NULL,
    object_types SMALLINT NOT NULL,
    src          VARCHAR(100) NOT NULL
);

--______________________________________________________________________________________

CREATE TABLE countries (
    id           SERIAL PRIMARY KEY,
    name         VARCHAR(100) NOT NULL,
    geo_id       INT,
    continent_id INT, 
    timezone_id  INT,
    phone        VARCHAR(10),
    lat          FLOAT,
    lon          FLOAT
); 
CREATE INDEX countries_continent_idx ON countries (continent_id);
----------------------------

CREATE TABLE regions (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    geo_id      INT,
    country_id  INT NOT NULL,
    timezone_id INT,
    lat         FLOAT,
    lon         FLOAT
);
CREATE INDEX regions_country_idx ON regions (country_id);
----------------------------

CREATE TABLE cities (
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    geo_id     INT,
    region_id  INT, 
    country_id INT NOT NULL,
    lat        FLOAT,
    lon        FLOAT
);
CREATE INDEX cities_country_idx ON cities (country_id);
CREATE INDEX cities_region_idx ON cities (region_id);

CREATE TABLE districts (
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(100) NOT NULL,
    region_id  INT, 
    country_id INT NOT NULL,
    lat        FLOAT,
    lon        FLOAT
); 

--______________________________________________________________________________________

-- Создание таблицы "organizations" для хранения данных об организации
/*
types
1  организация предложена
2  организация одобрена
*/
CREATE TABLE organizations ( 
    id          SERIAL PRIMARY KEY,     -- Уникальный идентификатор организации
    name        VARCHAR(100) NOT NULL,  -- Название организации
    description VARCHAR(500) NOT NULL,  -- Описание организации
    director    VARCHAR(255) NOT NULL,  -- Руководитель организации
    phone       VARCHAR(15) NOT NULL,   -- Номер телефона организации
    hours       VARCHAR(100) NOT NULL,  -- Время работы организации
    website     VARCHAR(100),           -- Веб-сайт организации (может быть пустым)
    image       VARCHAR(100),           -- Ссылка на фотографию организации (может быть пустой)
    user_id     INT NOT NULL,
    types       INT NOT NULL
);

-- Создание индекса для ускорения поиска по идентификатору организации
CREATE INDEX idx_organization_id ON organizations (id);


CREATE TABLE organizations_places (
    id              SERIAL PRIMARY KEY,
    organization_id INT NOT NULL,
    city_id         INT NOT NULL,
    region_id       INT, 
    country_id      INT NOT NULL,
    address2        VARCHAR(500) NOT NULL
);

--______________________________________________________________________________________

-- Создание таблицы "Places" для хранения данных о местах
/*
types
1  кладбище предложено
2  кладбище одобрено
*/
CREATE TABLE places (
    id           SERIAL PRIMARY KEY,
    user_id      INT NOT NULL,
    city_id      INT,
    district_id  INT,
    region_id    INT,
    country_id   INT NOT NULL,
    title        VARCHAR(100) NOT NULL,
    description  VARCHAR(500),
    hours        VARCHAR(100),
    image        VARCHAR(100),
    address      VARCHAR(255),
    count        SMALLINT NOT NULL,
    director     VARCHAR(255),
    phone        VARCHAR(15),
    lat          FLOAT NOT NULL,
    lon          FLOAT NOT NULL,
    types        INT NOT NULL
);

CREATE TABLE reviews (
    id         SERIAL PRIMARY KEY,
    service_id INT NOT NULL,
    user_id    INT NOT NULL,
    content    VARCHAR(1000) NOT NULL,
    created    TIMESTAMP NOT NULL
);

CREATE TABLE services (
    id              SERIAL PRIMARY KEY,
    user_id         INT NOT NULL,
    organization_id INT NOT NULL,
    title           VARCHAR(100) NOT NULL,
    description     VARCHAR(1000) NOT NULL,
    image           VARCHAR(100),
    price           INT NOT NULL
);
