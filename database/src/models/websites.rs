
use crate::{db::Database, models::users::User};
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
   pub id: String,
   pub url: String,
   pub user_id: String,
   pub created_at: NaiveDateTime,
   pub updated_at: NaiveDateTime,
}

pub struct  WebsiteWithUser {
    pub website: Website,
    pub user: User,
}

// Define a trait for website operations
pub trait WebsiteMethods {
    fn create_website(&mut self, url: String, user_id: String) -> Result<String, Error>;
    fn get_website(&mut self, website_id: &str) -> Result<WebsiteWithUser, Error>;
    fn delete_website(&mut self, website_id: &str) -> Result<String, Error>;
}

// Implement the trait for Database
impl WebsiteMethods for Database {
    fn create_website(&mut self, url: String, user_id: String) -> Result<String, Error> {
        use crate::schema::website::table as website_table;
        
        let website = Website {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            user_id,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        let result = diesel::insert_into(website_table)
            .values(&website)
            .returning(crate::schema::website::id)
            .get_result(&mut self.connection)?;

        Ok(result)
    }

    fn get_website(&mut self, website_id: &str) -> Result<WebsiteWithUser, Error> {
        use crate::schema::website::dsl::*;

        let (website_data, user_data) = website
            .inner_join(crate::schema::user::table)
            .filter(id.eq(website_id))
            .select((Website::as_select(), User::as_select()))
            .first::<(Website, User)>(&mut self.connection)
            .map_err(|_| Error::NotFound)?;

        Ok(WebsiteWithUser {
            website: website_data,
            user: user_data,
        })
    }

    fn delete_website(&mut self, website_id: &str) -> Result<String, Error> {
        use crate::schema::website::dsl::*;

        let result = diesel::delete(website.filter(id.eq(website_id)))
            .execute(&mut self.connection)
            .expect("Error deleting website");

        if result > 0 {
            Ok(website_id.to_string())
        } else {
            Err(Error::NotFound)
        }
    }
}