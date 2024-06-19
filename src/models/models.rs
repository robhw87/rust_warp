use chrono::NaiveDate;
use futures::TryStreamExt;
use std::error::Error;
use serde::Serialize;
use async_trait::async_trait;
use sqlx::{PgPool, Result,Row};



#[derive(Serialize, Debug)]
pub struct Users {
    pub data_id: i32,
    pub first_name: String, 
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub salary: i32,
}

//async fn create(pool: &PgPool, entity: &Entity) -> Result<Entity>;
    //async fn update(pool: &PgPool, entity: &Entity) -> Result<Entity>;
    //async fn delete(pool: &PgPool, id: Id) -> Result<()>;

#[async_trait]
pub trait DBProcess<Params> {
    async fn read(pool: &PgPool) -> Result<Vec<Params>, Box<dyn Error>>;
}

#[async_trait]
impl DBProcess<Users> for Users {
    async fn read(pool: &PgPool) -> Result<Vec<Users>, Box<dyn Error>> {
    // Implement SQL statement generation and execution for creating a Users
    let q = "SELECT first_name, last_name, birthdate, salary FROM users_list";
    let query = sqlx::query(q);
  
    let mut rows = query.fetch(pool);
    let mut datas = vec![];

     while let Some(row) = rows.try_next().await? {
       datas.push(Users {
         data_id: row.get("id"),
         first_name: row.get("first_name"),
         last_name: row.get("last_name"),
         birthdate: row.get("birthdate"),
         salary: row.get("salary"),
       });
    }
    Ok(datas)
}
}
