use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{SessionId, UserId};

use crate::DieselError;

use crate::schema;

// NOTE The `Fingerprint` can be used to identify a logged user's device and location, which can enable more security features
// ! however it won't be used since it'd need MFA to be implemented first
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct Fingerprint(serde_json::Value);

impl From<serde_json::Value> for Fingerprint {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

// ! The `Queryable` and `Insertable` derive macros work as long as it's fields are in the same order as the columns in the database table
// NOTE `Queryable` is used to convert a database row into a struct
// NOTE `Insertable` is used to insert a struct into the database table
#[derive(Clone, Debug, PartialEq, Queryable, Insertable)]
#[diesel(table_name = schema::web)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub fingerprint: Fingerprint,
}

pub fn new(
    conn: &mut PgConnection,
    user_id: UserId,
    duration: chrono::Duration,
    fingerprint: Fingerprint,
) -> Result<Session, DieselError> {
    let uid = user_id;
    let new_session = Session {
        id: SessionId::new(),
        user_id: uid,
        expires_at: Utc::now() + duration,
        created_at: Utc::now(),
        fingerprint,
    };
    {
        use crate::schema::web;
        diesel::insert_into(web::table)
            .values(&new_session)
            .on_conflict((web::user_id, web::fingerprint))
            .do_update()
            .set(web::expires_at.eq(new_session.expires_at))
            .get_result(conn)
    }
}
