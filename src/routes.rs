// src/routes.rs
use std::time::Instant;
use std::ops::Deref;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use uuid::Uuid;
use crate::db;
use crate::models;
use crate::server;
use crate::session;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index() -> impl Responder {
	NamedFile::open_async("./static/index.html").await.unwrap()
}

#[get("/ws")]
pub async fn chat_server(
	req: HttpRequest,
	stream: web::Payload,
	pool: web::Data<DbPool>,
	srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    println!("ws starting...");
	ws::start(
		session::WsChatSession {
			id: 0,
			hb: Instant::now(),
			room: "main".to_string(),
			addr: srv.get_ref().clone(),
			db_pool: pool,
            name: None,
		},
		&req, stream)
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    println!("user create!");
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone)
    }).await?.map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/conversations/create")]
pub async fn create_conversation(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewConversation>,
) -> Result<HttpResponse, Error> {
    println!("conversation create!");
    let conversation = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_conversation(&mut conn, form.deref().clone())
    }).await?.map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(conversation))
}

#[post("/rooms/create")]
pub async fn create_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewRoom>,
) -> Result<HttpResponse, Error> {
    println!("room create!");
    let room = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_room(&mut conn, form.deref().clone())
    }).await?.map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(room))
}

#[post("/rooms/join")]
pub async fn join_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::JoinRoom>,
) -> Result<HttpResponse, Error> {
    println!("room join!");
    let room = web::block(move || {
        let mut conn = pool.get()?;
        db::join_room(&mut conn, form.deref().clone())
    }).await?.map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(room))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(json!({
            "error": 404,
            "messsage": format!("No user found with phone: {id}")
        }).to_string());
        Ok(res)
    }
}

#[get("/conversations/{uid}")]
pub async fn get_conversation_by_id(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let conversations = web::block(move || {
        let mut conn = pool.get()?;
        db::get_conversation_by_room_uid(&mut conn, room_id)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(data) = conversations {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(json!({
            "error": 404,
            "messsage": format!("No conversation found with room: {room_id}")
        }).to_string());
        Ok(res)
    }
}

#[get("/users/phone/{user_phone}")]
pub async fn get_user_by_phone(
    pool: web::Data<DbPool>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_phone(&mut conn, user_phone)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(json!({
            "error": 404,
            "messsage": format!("No user found with phone: {}", phone.to_string())
        }).to_string());
        Ok(res)
    }
}

#[get("/rooms")]
pub async fn get_rooms(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_rooms(&mut conn)
    }).await?.map_err(actix_web::error::ErrorInternalServerError)?;
    //if !rooms.is_empty() {
        Ok(HttpResponse::Ok().json(rooms))
    //} else {
    //    let res = HttpResponse::NotFound().body(json!({
    //        "error": 404,
    //        "messsage": format!("No rooms available at the moment")
    //    }).to_string());
    //    Ok(res)
    //}
}

