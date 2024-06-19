use serde_json::json;
use serde_json::Value;
use warp::{reply::Json, Filter};
use crate::middleware::{do_auth, UserCtx, db_pool};

pub fn users_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_url = warp::path("users");
    let list = base_url
        .and(warp::get())
        .and(warp::path::end())
        .and(do_auth())
        .and_then(users_list);

    let get = base_url
        .and(warp::get())
        .and(do_auth())
        .and(warp::path::param())
        .and_then(users_get);

        let create = base_url
        .and(warp::post())
        .and(do_auth())
        .and(warp::body::json())
        .and_then(users_create);

    list.or(get).or(create)
}



async fn users_list(_user_ctx: UserCtx, pool:db_pool()) -> Result<Json, warp::Rejection> {
    let users = Users::read(&pool).await?;

    let user = warp::reply::json(&users);
    Ok(user)
}

async fn users_get(_user_ctx: UserCtx, id:i64) -> Result<Json, warp::Rejection> {
    let users = json!([
        {"id":id, "user_id":_user_ctx.user_id, "name": format!("Name: {}", id)},
    ]);

    let user = warp::reply::json(&users);
    Ok(user)
}

async fn users_create(_user_ctx: UserCtx, data: Value) -> Result<Json, warp::Rejection> {
    let users = data;
    let users = warp::reply::json(&users);
    Ok(users)
}