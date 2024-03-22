use crate::schema;
use crate::schema::{
    users,
    countries,
    regions,
    cities,
    districts,
    files,
    logs,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::errors::Error;


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id:          i32,
    pub username:    String,
    pub first_name:  String,
    pub last_name:   String, 
    pub phone:       String,
    pub email:       String,
    pub password:    String,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub perm:        i16,
    pub created:     chrono::NaiveDateTime,
}
impl User {
    pub fn get_full_name(&self) -> String {
        self.first_name + &" ".to_string() + &self.last_name;
    }
    pub fn edit (   
        &self,
        username:   String,
        first_name: String,
        last_name:  String,
        phone:      String,
        email:      String,
        image:      Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::users::username.eq(username),
                schema::users::first_name.eq(first_name),
                schema::users::last_name.eq(last_name),
                schema::users::phone.eq(phone),
                schema::users::email.eq(email),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        if image.is_some() {
            diesel::update(self)
                .set(schema::users::image.eq(image))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn get_suggested_stat(&self) -> (usize, usize, usize) {
        if self.is_admin() {
            let _connection = establish_connection();
            let org_count = schema::organizations::table
                .filter(schema::organizations::types.ne(2))
                .select(schema::organizations::id)
                .load::<i32>(&_connection)
                .expect("E.")
                .len();
            let place_count = schema::places::table
                .filter(schema::places::types.ne(2))
                .select(schema::places::id)
                .load::<i32>(&_connection)
                .expect("E.")
                .len(); 
            let deceased_count = schema::deceaseds::table
                .filter(schema::deceaseds::types.ne(2))
                .select(schema::deceaseds::id)
                .load::<i32>(&_connection)
                .expect("E.")
                .len();
            return (org_count, place_count, deceased_count);
        }
        return (0,0,0)
    }

    pub fn get_all(exclude_user_id: i32) -> Vec<User> {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::id.ne(exclude_user_id))
            .load::<User>(&_connection)
            .expect("E");
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn is_admin(&self) -> bool {
        return self.perm > 9;
    }
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(60))
            .execute(&_connection);
        return 1;
    }
    pub fn create_admin(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(10))
            .execute(&_connection);
        return 1;
    }
    pub fn remove_staff(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(1))
            .execute(&_connection);
        return 1;
    }
    pub fn next_count() -> String { 
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return (users
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() + 1)
            .to_string();
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username:    String,
    pub first_name:  String,
    pub last_name:   String,
    pub phone:       String,
    pub email:       String,
    pub password:    String,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub perm:        i16,
    pub created:     chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}




#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Countrie { 
    pub id:           i32,
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub cord:         Option<String>,
}
impl Countrie {
    pub fn get_all() -> Vec<Countrie> {
        let _connection = establish_connection();
        return schema::countries::table
            .load::<Countrie>(&_connection)
            .expect("E");
    }
    pub fn create ( 
        name: String,
        cord: Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCountrie {
            name:         name,
            geo_id:       None,
            continent_id: None,
            timezone_id:  None,
            phone:        None,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::countries::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        name: String,
        cord: Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::countries::name.eq(name),
                schema::countries::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::countries::table.filter(schema::countries::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="countries"]
pub struct NewCountrie { 
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub cord:         Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Region { 
    pub id:          i32,
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub cord:        Option<String>,
}
impl Region {
    pub fn get_country_all(id: i32) -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .filter(schema::regions::country_id.eq(id))
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn create (
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewRegion {
            name:         name,
            geo_id:       None,
            country_id:   country_id,
            timezone_id:  None,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::regions::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::regions::name.eq(name),
                schema::regions::country_id.eq(country_id),
                schema::regions::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::regions::table.filter(schema::regions::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="regions"]
pub struct NewRegion { 
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub cord:        Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Citie { 
    pub id:         i32,
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}
impl Citie {
    pub fn get_all() -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_region_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::region_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::country_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCitie {
            name:         name,
            geo_id:       None,
            region_id:    region_id,
            country_id:   country_id,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::cities::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::cities::name.eq(name),
                schema::cities::region_id.eq(region_id),
                schema::cities::country_id.eq(country_id),
                schema::cities::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::cities::table.filter(schema::cities::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="cities"]
pub struct NewCitie { 
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct District { 
    pub id:         i32,
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}
impl District {
    pub fn get_region_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::region_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::country_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
        lon:        Option<f64>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewDistrict {
            name:         name,
            region_id:    region_id,
            country_id:   country_id,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::districts::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::districts::name.eq(name),
                schema::districts::region_id.eq(region_id),
                schema::districts::country_id.eq(country_id),
                schema::districts::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::districts::table.filter(schema::districts::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="districts"]
pub struct NewDistrict { 
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}


/*
    Файлы для прикрепления к объектам. Наприммер, универсально подойдет для галереи покойника.
    
    object_types:
    | 1 Организация
    | 2 Кладбище
    | 3 Покойник
*/

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct File { 
    pub id:           i32,
    pub object_id:    i32,
    pub object_types: i16,
    pub src:          String,
}

impl File {
    pub fn get_images_ids_for_object(&self) -> Vec<i32> {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.object_id))
            .filter(schema::files::object_types.eq(self.object_types))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_prev_next_images(&self) -> (Option<File>, Option<File>) {
        let _connection = establish_connection();
        let ids = schema::files::table
            .filter(schema::files::object_id.eq(self.object_id))
            .filter(schema::files::object_types.eq(self.object_types))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
        let _images_len = ids.len();
        let mut prev: Option<File> = None;
        let mut next: Option<File> = None;

        for (i, obj) in ids.iter().enumerate().rev() {
            if obj == &self.id { 
                if (i + 1) != _images_len {
                    let _next = Some(&ids[i + 1]);
                    next = Some(schema::files::table
                        .filter(schema::files::id.eq(_next.unwrap()))
                        .first::<File>(&_connection)
                        .expect("E"));
                };
                if i != 0 {
                    let _prev = Some(&ids[i - 1]);
                    prev = Some(schema::files::table
                        .filter(schema::files::id.eq(_prev.unwrap()))
                        .first::<File>(&_connection)
                        .expect("E"));
                };
                break;
            }
        };
        return (prev, next);
    }

    pub fn create (
        object_id:    i32,
        object_types: i16,
        images:       Vec<String>,
    ) -> i16 {
        if images.len() > 0 {
            let _connection = establish_connection();

            for i in images.into_iter() {
                let new_form = NewFile {
                    object_id:    object_id,
                    object_types: object_types,
                    src:          i,
                };
                let _new = diesel::insert_into(schema::files::table)
                    .values(&new_form)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        diesel::delete(files.filter(schema::files::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="files"]
pub struct NewFile { 
    pub object_id:    i32,
    pub object_types: i16,
    pub src:          String,
}


/*
    object_id - id объекта лога (кладбище, покойник и тд)
    types ↓                 |   verb ↓
    1. Профиль              |   1. Создал
    2. Организация          |   2. Изменил
    3. Кладбище             |   3. Удалил
    4. Покойник             |   4. Одобрил (например, предложенное кладбище)
    5. Отзыв                |   5. Добавил на стену памяти
    6. Локация офиса        |   6. Удалил со стены памяти
*/

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Log { 
    pub id:        i32,
    pub user_id:   i32,
    pub object_id: i32,
    pub types:     i16,
    pub verb:      i16,
    pub created:   chrono::NaiveDateTime,
}
pub struct LogResp {
    pub user:    User,
    pub text:    String,
    pub created: chrono::NaiveDateTime,
}
impl Log {
    pub fn delete(&self) -> i16 {
        use crate::schema::logs::dsl::logs;

        let _connection = establish_connection();
        diesel::delete(logs.filter(schema::logs::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
    pub fn get_text (&self) -> String {
        let mut text = String::new();
        let verb: String = match self.verb {
            1 => "создал(а) ".to_string(),
            2 => "изменил(а) ".to_string(),
            3 => "удалил(а) ".to_string(),
            4 => "одобрил(а) ".to_string(),
            5 => "добавил(а) на стену памяти ".to_string(),
            6 => "удалил(а) со стены памяти ".to_string(),
        };
        let types: String = match self.types {
            1 => "профиль".to_string(),
            2 => {
                let obj = crate::get_organization(self.object_id).expect("E.");
                "организацию ".to_string() + &"<a href='/organization/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.name + &"</a>".to_string();
            },
            3 => {
                let obj = crate::get_place(self.object_id).expect("E.");
                "кладбище ".to_string() + &"<a href='/place/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.title + &"</a>".to_string();
            },
            4 => {
                let obj = crate::get_deceased(self.object_id).expect("E.");
                "покойника ".to_string() + &"<a href='/deceased/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.get_full_name() + &"</a>".to_string();
            },
            5 => {
                let obj = crate::get_review(self.object_id).expect("E.");
                "отзыв ".to_string() + &"<a href='/review/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.name + &"</a>".to_string();
            },
            6 => {
                let obj = crate::get_organization_loc(self.object_id).expect("E.");
                "офис ".to_string();
            },
        };
        text.push_str(verb);
        text.push_str(types);
        return text;
    }
    pub fn create (
        user_id:   i32,
        object_id: i32,
        types:     i16,
        verb:      i16,
    ) -> i16 {
        let new_form = NewLog {
            user_id:   user_id,
            object_id: object_id,
            types:     types,
            verb:      verb,
            created:   chrono::Local::now().naive_utc(),
        };
        let _new = diesel::insert_into(schema::logs::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");

        return 1;
    }
    pub fn get_all (
        user_id: i32,
        limit:   i64,
        offset:  i64,
    ) -> Vec<LogResp> {
        use crate::utils::get_user;
        let _connection = establish_connection();
        let _user = crate::get_user(user_id).expect("E.");
        let mut stack = Vec::new();
        if _user.perm > 9 {
            let list = logs
                .order(schema::logs::created.desc())
                .limit(limit)
                .offset(offset)
                .load::<Log>(&_connection)
                .expect("E.");
            
            for i in list.into_iter() {
                stack.push( LogResp { 
                    text:    i.get_text(),
                    created: i.created,
                });
            }

            return stack;
        }
        else {
            return Vec::new();
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="logs"]
pub struct NewLog { 
    pub user_id:   i32,
    pub object_id: i32,
    pub types:     i16,
    pub verb:      i16,
    pub created:   chrono::NaiveDateTime,
}