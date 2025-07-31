use crate::{db::Database};
use chrono::{NaiveDateTime};
use diesel::{prelude::*, result::Error};
use uuid::Uuid;
use crate::utils::{hash_password, verify_password};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Define a trait for user operations
pub trait UserMethods {
    fn sign_up(&mut self, email: String, password: String) -> Result<String, Error>;
    fn sign_in(&mut self, input_email: String, input_password: String) -> Result<User, Error>;
    fn delete_user(&self, user_id: &str) -> String;
    fn get_user(&self, user_id: &str) -> String;
}

// Implement the trait for Database
impl UserMethods for Database {
    fn sign_up(&mut self, email: String, password: String) -> Result<String, Error> {
        use crate::schema::user::table as user_table;

        // Check if the email already exists
        let existing_user = user_table
            .filter(crate::schema::user::email.eq(&email))
            .first::<User>(&mut self.connection)
            .optional()?;

        if existing_user.is_some() {
            return Err(Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("Email already exists".to_string()),
            ));
        }

        // Hash the password using the utility function
        let hashed_password = hash_password(password).expect("Failed to hash password");

        // New user instance to be inserted into the database
        let new_user = User {
            id: Uuid::new_v4().to_string(),
            email: email,
            password: hashed_password,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        // Insert the new user into the database and return the ID
        let result = diesel::insert_into(user_table)
            .values(&new_user)
            .returning(crate::schema::user::id)
            .get_result(&mut self.connection)
            .expect("Error inserting new user");

        // Return the ID of the newly created user
        Ok(result)
    }

    fn sign_in(&mut self, input_email: String, input_password: String) -> Result<User, Error> {
       use crate::schema::user::dsl::*;

        // Check if the user exists with the provided email and password
        let result = user
            .filter(email.eq(input_email))
            .select(User::as_select())
            .first(&mut self.connection)
            .expect("Error loading user");

        // Verify the provided password against the stored hashed password
        let is_valid = verify_password(input_password, &result.password);

        if is_valid {
            Ok(result)
        } else {
            Err(Error::NotFound)
        }
    }

    fn delete_user(&self, user_id: &str) -> String {
        // Simulate deleting a user by ID
        format!("deleted user: {}", user_id)
    }

    fn get_user(&self, user_id: &str) -> String {
        // Simulate fetching a user by ID
        format!("user: {}", user_id)
    }
}