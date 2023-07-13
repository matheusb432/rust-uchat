use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PostId, UserId};

use crate::{schema, DieselError};

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

pub fn bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<(), DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::bookmarks::dsl::*;

        diesel::insert_into(bookmarks)
            .values((user_id.eq(uid), post_id.eq(pid)))
            .on_conflict((user_id, post_id))
            .do_nothing()
            .execute(conn)
            .map(|_| ())
    }
}

pub fn get_bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<bool, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::bookmarks::dsl::*;
        use diesel::dsl::count;

        bookmarks
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .select(count(post_id))
            .get_result(conn)
            .optional()
            .map(is_one)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeleteStatus {
    Deleted,
    NotFound,
}

impl DeleteStatus {
    pub fn new(count: usize) -> Self {
        if count > 0 {
            DeleteStatus::Deleted
        } else {
            DeleteStatus::NotFound
        }
    }
}

pub fn delete_bookmark(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let pid = post_id;
    {
        use crate::schema::bookmarks::dsl::*;

        diesel::delete(bookmarks)
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .execute(conn)
            .map(DeleteStatus::new)
    }
}

#[derive(Clone, Debug, DieselNewType, Serialize, Deserialize)]
pub struct ReactionData(serde_json::Value);

#[derive(Clone, Debug, Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = schema::reactions)]
pub struct Reaction {
    pub user_id: UserId,
    pub post_id: PostId,
    pub created_at: DateTime<Utc>,
    pub like_status: i16,
    // ! Storing the reaction as a JSONB value is not scalable, should be a separate table
    pub reaction: Option<ReactionData>,
}

pub fn react(conn: &mut PgConnection, reaction: Reaction) -> Result<(), DieselError> {
    use crate::schema::reactions;

    diesel::insert_into(reactions::table)
        .values(&reaction)
        .on_conflict((reactions::user_id, reactions::post_id))
        .do_update()
        .set((
            reactions::like_status.eq(&reaction.like_status),
            reactions::reaction.eq(&reaction.reaction),
        ))
        .execute(conn)
        .map(|_| ())
}

pub fn get_reaction(
    conn: &mut PgConnection,
    post_id: PostId,
    user_id: UserId,
) -> Result<Option<Reaction>, DieselError> {
    let pid = post_id;
    let uid = user_id;
    {
        use crate::schema::reactions::dsl::*;

        reactions
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .get_result(conn)
            .optional()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AggregatePostInfo {
    pub post_id: PostId,
    pub likes: i64,
    pub dislikes: i64,
    pub boosts: i64,
}

pub fn aggregate_reactions(
    conn: &mut PgConnection,
    post_id: PostId,
) -> Result<AggregatePostInfo, DieselError> {
    let pid = post_id;

    let (likes, dislikes) = {
        use crate::schema::reactions::dsl::*;
        let likes = reactions
            .filter(post_id.eq(pid))
            .filter(like_status.eq(1))
            .count()
            .get_result(conn)?;
        let dislikes = reactions
            .filter(post_id.eq(pid))
            .filter(like_status.eq(-1))
            .count()
            .get_result(conn)?;

        (likes, dislikes)
    };

    let boosts = {
        use crate::schema::boosts::dsl::*;
        boosts.filter(post_id.eq(pid)).count().get_result(conn)?
    };

    Ok(AggregatePostInfo {
        post_id: pid,
        likes,
        dislikes,
        boosts,
    })
}

pub fn boost(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
    when: DateTime<Utc>,
) -> Result<(), DieselError> {
    let pid = post_id;
    let uid = user_id;

    {
        use crate::schema::boosts::dsl::*;

        diesel::insert_into(boosts)
            .values((user_id.eq(uid), post_id.eq(pid), boosted_at.eq(when)))
            .on_conflict((user_id, post_id))
            .do_update()
            .set(boosted_at.eq(when))
            .execute(conn)
            .map(|_| ())
    }
}

fn is_one(n: Option<i64>) -> bool {
    match n {
        Some(n) => n == 1,
        None => false,
    }
}

// TODO refactor closure duplications in boost and bookmark
pub fn get_boost(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<bool, DieselError> {
    let uid = user_id;
    let pid = post_id;

    {
        use crate::schema::boosts::dsl::*;
        use diesel::dsl::count;

        boosts
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .select(count(post_id))
            .get_result(conn)
            .optional()
            .map(is_one)
    }
}

pub fn delete_boost(
    conn: &mut PgConnection,
    user_id: UserId,
    post_id: PostId,
) -> Result<DeleteStatus, DieselError> {
    let uid = user_id;
    let pid = post_id;
    {
        use crate::schema::boosts::dsl::*;

        diesel::delete(boosts)
            .filter(post_id.eq(pid))
            .filter(user_id.eq(uid))
            .execute(conn)
            .map(DeleteStatus::new)
    }
}
