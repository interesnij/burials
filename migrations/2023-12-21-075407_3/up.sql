-- Your SQL goes here

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
