use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use password_hash::PasswordHashString;
use uchat_domain::ids::UserId;
use uchat_domain::Username;
use uchat_endpoint::Update;

use crate::util::{is_one, DeleteStatus};
use crate::{schema, DieselError, QueryError};

pub fn new<T: AsRef<str>>(
    conn: &mut PgConnection,
    hash: PasswordHashString,
    handle: T,
) -> Result<UserId, QueryError> {
    use crate::schema::users::{self, columns};

    let user_id = UserId::new();

    diesel::insert_into(users::table)
        .values((
            columns::id.eq(user_id),
            columns::password_hash.eq(hash.to_string()),
            columns::handle.eq(handle.as_ref()),
        ))
        .execute(conn)?;

    Ok(user_id)
}

pub fn get_password_hash(
    conn: &mut PgConnection,
    username: &Username,
) -> Result<String, QueryError> {
    use crate::schema::users::dsl::*;

    Ok(users
        .filter(handle.eq(username.as_ref()))
        .select(password_hash)
        .get_result(conn)?)
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: UserId,
    pub email: Option<String>,
    pub email_confirmed: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub handle: String,
    pub created_at: DateTime<Utc>,
    pub profile_image: Option<String>,
}

pub fn get(conn: &mut PgConnection, user_id: UserId) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;

    users.filter(id.eq(user_id)).get_result(conn)
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
pub struct Profile {
    pub id: UserId,
    pub display_name: Option<String>,
    pub handle: String,
    pub email: Option<String>,
    pub profile_image: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ? More optimized query that only gets necessary data
pub fn get_profile(conn: &mut PgConnection, user_id: UserId) -> Result<Profile, DieselError> {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .select(Profile::as_select())
        .get_result(conn)
}

pub fn is_following(
    conn: &mut PgConnection,
    user_id: UserId,
    follows: UserId,
) -> Result<bool, DieselError> {
    use crate::schema::followers;
    use diesel::dsl::count;

    followers::table
        .filter(followers::user_id.eq(user_id))
        .filter(followers::follows.eq(follows))
        .select(count(followers::user_id))
        .get_result(conn)
        .optional()
        .map(is_one)
}

pub fn unfollow(
    conn: &mut PgConnection,
    user_id: UserId,
    follows: UserId,
) -> Result<DeleteStatus, DieselError> {
    use crate::schema::followers;

    diesel::delete(followers::table)
        .filter(followers::user_id.eq(user_id))
        .filter(followers::follows.eq(follows))
        .execute(conn)
        .map(DeleteStatus::new)
}

pub fn follow(
    conn: &mut PgConnection,
    user_id: UserId,
    follows: UserId,
) -> Result<(), DieselError> {
    use crate::schema::followers;

    diesel::insert_into(followers::table)
        .values((
            followers::user_id.eq(user_id),
            followers::follows.eq(follows),
        ))
        .execute(conn)?;
    Ok(())
}

pub fn find(conn: &mut PgConnection, username: &Username) -> Result<User, DieselError> {
    use crate::schema::users::dsl::*;

    users.filter(handle.eq(username.as_ref())).get_result(conn)
}

#[derive(Debug)]
pub struct UpdateProfileParams {
    pub id: UserId,
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub password_hash: Update<PasswordHashString>,
    pub profile_image: Update<String>,
}

// NOTE AsChangeset is a diesel trait that enables updating only the necessary fields on a given struct
#[derive(AsChangeset, Debug)]
#[diesel(table_name = crate::schema::users)]
struct UpdateProfileInternal {
    // NOTE The outermost option being None won't cause updates to the DB
    // ? None => nothing happens
    // ? Some(None) => the field is set to NULL
    // ? Some(Some(_)) => the field is updated
    pub display_name: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub password_hash: Option<String>,
    pub profile_image: Option<Option<String>>,
}

pub fn update_profile(
    conn: &mut PgConnection,
    query_params: UpdateProfileParams,
) -> Result<(), DieselError> {
    use crate::schema::users;

    let update = UpdateProfileInternal {
        display_name: query_params.display_name.into_nullable(),
        email: query_params.email.into_nullable(),
        password_hash: query_params
            .password_hash
            .into_option()
            .map(|s| s.to_string()),
        profile_image: query_params.profile_image.into_nullable(),
    };

    diesel::update(users::table)
        .filter(users::id.eq(query_params.id))
        // NOTE AsChangeset enables much easier update syntax than manually specifying each field
        .set(&update)
        .execute(conn)
        .map(|_| ())
}

#[cfg(test)]
pub mod tests {
    pub mod util {
        use diesel::PgConnection;

        use crate::user::User;

        pub fn new_user(conn: &mut PgConnection, handle: &str) -> User {
            use crate::user as user_query;

            let hash = uchat_crypto::hash_password("password").unwrap();
            let id = user_query::new(conn, hash, handle).unwrap();

            user_query::get(conn, id).unwrap()
        }
    }
}
