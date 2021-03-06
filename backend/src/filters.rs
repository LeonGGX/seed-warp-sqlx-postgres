// src/filters

use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Html;

use sqlx::PgPool;

use crate::handlers;
use crate::models;


fn with_db(pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn post_json() -> impl Filter<Extract = (models::InsertablePerson,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body() -> impl Filter<Extract = (models::InsertablePerson,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn page_index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone  {
    warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("D:\\Programmation\\Rust\\mes_programmes\\myperson\\frontend\\index.html"))
}

pub fn person_filters(
    pool: PgPool
)-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    page_index()
        .or(list_persons(pool.clone()))
        .or(add_person(pool.clone()))
        .or(find_one_person_id(pool.clone()))
        .or(delete_person(pool.clone()))
        .or(update_person(pool.clone()))
}


pub fn list_persons(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::list_persons_hdler)
}


pub fn add_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  warp::post()
        .and(post_json())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::add_person_hdler)
}

pub fn update_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::put()
        .and(warp::path::param())
        .and(json_body())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_person_hdler)
}

pub fn delete_person(
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::delete()
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_person_hdler)
}

pub fn find_one_person_id (
    pool: PgPool
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
   warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(with_db(pool.clone()))
        .and_then(handlers::find_person_by_id_hdler)
}



