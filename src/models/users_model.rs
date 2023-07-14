use actix_web::{web};

use crate::utils::establish_connection;
use crate::schema::users;

use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::sql_types::Integer;


type DB = diesel::mysql::Mysql;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl QueryableByName<DB> for User {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
        row: &R,
    ) -> diesel::deserialize::Result<Self> {
        Ok(User {
            id: row.get("id")?,
            name: row.get("name")?,
            email: row.get("email")?,
            password: row.get("password")?,
        })
    }
}

impl User {
    pub fn get_all() -> String {
        let conn: MysqlConnection = establish_connection();

        let users: Vec<User> = sql_query("SELECT * FROM users;")
        .load(&conn)
        .unwrap();

        let json_str = serde_json::to_string(&users).unwrap();

        return json_str;
    }
    
    pub fn get_one(id: i32) -> String {
        let conn: MysqlConnection = establish_connection();

        let user: Vec<User> = sql_query("SELECT * FROM users WHERE id = ?;")
        .bind::<Integer, _>(id)
        .load(&conn)
        .unwrap();

        let json_str = serde_json::to_string(&user).unwrap();

        return json_str;
    }

    pub fn post(info: web::Json<User>) {
        let conn: MysqlConnection = establish_connection();

        diesel::sql_query(
            "INSERT INTO users (name, email, password) 
             VALUES (?, ?, ?)",)
        .bind::<Text, _>(info.name.to_string())
        .bind::<Text, _>(info.email.to_string())
        .bind::<Text, _>(info.password.to_string())
        .execute(&conn)
        .unwrap();
    }

    pub fn update(info: web::Json<User>) {
        let conn: MysqlConnection = establish_connection();

        diesel::sql_query(
            "
             UPDATE
                users
             SET
                name = ?,
                email = ?,
                password = ?
             WHERE
                id = ?
            ",)
        .bind::<Text, _>(info.name.to_string())
        .bind::<Text, _>(info.email.to_string())
        .bind::<Text, _>(info.password.to_string())
        .bind::<Text, _>(info.id.to_string())
        .execute(&conn)
        .unwrap();
    }

    pub fn delete(id: i32) {
        let conn = establish_connection();

        diesel::sql_query(
            "
             DELETE FROM users
             WHERE id = ?
            ",)
        .bind::<Text, _>(id.to_string())
        .execute(&conn)
        .unwrap();
    }
}