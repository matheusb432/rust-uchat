use serde::{Deserialize, Serialize};
use uchat_domain::{ids::*, Password, Username};

use crate::Endpoint;

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,
}

impl Endpoint for CreateUser {
    // NOTE Will be acessible from the struct as `CreateUser::URL` or from the trait as `&self.url()`
    const URL: &'static str = "/account/create";
}
