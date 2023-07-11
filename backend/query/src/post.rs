use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};
use password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PostId, UserId};
use uchat_domain::Username;
use uuid::Uuid;

use crate::{schema, DieselError, QueryError};

#[derive(Clone, Debug, DieselNewType, Serialize, Deserialize)]
pub struct Content(pub serde_json::Value);

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::posts)]
pub struct Post {
    pub id: PostId,
    pub user_id: UserId,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub direct_message_to: Option<UserId>,
    pub reply_to: Option<PostId>,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(
        posted_by: UserId,
        content: uchat_endpoint::post::types::Content,
        options: uchat_endpoint::post::types::NewPostOptions,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            // ! necessary to be `Uuid::new_v4().into()` ?
            id: PostId::new(),
            user_id: posted_by,
            content: Content(serde_json::to_value(content)?),
            time_posted: options.time_posted,
            direct_message_to: options.direct_message_to,
            reply_to: options.reply_to,
            created_at: Utc::now(),
        })
    }
}

pub fn new(conn: &mut PgConnection, post: Post) -> Result<PostId, DieselError> {
    conn.transaction::<PostId, DieselError, _>(|conn| {
        diesel::insert_into(schema::posts::table)
            .values(&post)
            .execute(conn)?;
        Ok(post.id)
    })
}

pub fn get(conn: &mut PgConnection, post_id: PostId) -> Result<Post, DieselError> {
    use crate::schema::posts::dsl::*;

    posts.filter(id.eq(post_id.as_uuid())).get_result(conn)
}

pub fn get_trending(conn: &mut PgConnection) -> Result<Vec<Post>, DieselError> {
    use crate::schema::posts;
    posts::table
        .filter(posts::time_posted.lt(Utc::now()))
        .filter(posts::direct_message_to.is_null())
        .order(posts::time_posted.desc())
        .limit(30)
        .get_results(conn)
}
