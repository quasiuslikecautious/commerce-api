use diesel::{ pg::PgConnection, prelude::*, sql_types::*, sql_function };
use dotenvy::dotenv;
use log::trace;
use std::env;

/// establish a connection to the database
/// 
/// This function creates a pgconnection to access data from the commerce database. The url is
/// grabbed from the .env file under the DATABASE_URL name.
/// 
/// # Panics
/// This function will panic if the .env file or DATABASE_URL field is missing. Additionally, this
/// will panic if it is unable to connect to the specified database
pub fn establish_connection() -> PgConnection {
    trace!("Starting connection to PostgreSQL...");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

sql_function! {
    /// The decryption plugin function used for password verification
    /// 
    /// This SQL function is from the pgcrypto extension for postgreSQL. It uses a basic salt
    /// decryption to verify that user passwords can only be retrieved / verified if the matching
    /// password has been supplied as an arg. It takes two arguments, the password and the 
    /// encrypted password.
    /// 
    /// # Examples
    /// 
    /// ```
    ///     ...
    ///     let user = users.filter(password.eq(crypt(<some inputted password string>, password))).first(&conn);
    ///     ...
    /// ```
    fn crypt(a: Text, b: Text) -> Text;
}

sql_function! {
    /// The encryption plugin function used for password encryption
    /// 
    /// This SQL function is from the pgcrypto extension for postgreSQL. It uses a basic salt
    /// encryption to ensure that user passwords are not stored as plain text. It takes one input,
    /// the user's password in our case and returns the resulting encrypted string to be stored.
    /// 
    /// # Examples
    /// 
    /// ```
    ///     ...
    ///     &diesel::insert_into(users)
    ///     .values((
    ///         email.eq(payload.email),
    ///         password.eq(crypt(payload.password, gen_salt("bf"))),
    ///     ))
    ///     ...
    /// ```
    fn gen_salt(a: Text) -> Text;
}