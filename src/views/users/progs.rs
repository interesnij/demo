//use crate::schema;
use actix_web::{
    HttpResponse,
    web,
    web::Json,
};
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    //establish_connection,
    get_user,
};
use actix_session::Session;
//use crate::diesel::{ExpressionMethods,RunQueryDsl};
use serde::{Deserialize, Serialize};


pub fn progs_urls(config: &mut web::ServiceConfig) {
    config.route("/users/progs/block/{id}/", web::post().to(user_block));
    config.route("/users/progs/unblock/{id}/", web::post().to(user_unblock));
    config.route("/users/progs/friend/{id}/", web::post().to(user_friend));
    config.route("/users/progs/unfriend/{id}/", web::post().to(user_unfriend));
    config.route("/users/progs/follow/{id}/", web::post().to(user_follow));
    config.route("/users/progs/follow_view/{id}/", web::post().to(user_follow_view));
    config.route("/users/progs/unfollow/{id}/", web::post().to(user_unfollow));
    config.route("/users/progs/save_playlist/{id}/", web::post().to(save_playlist));
}

pub async fn user_friend(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.frend_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_unfriend(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.unfrend_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_follow(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.follow_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
pub async fn user_follow_view(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.follow_view_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_unfollow(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.unfollow_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_block(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.block_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

pub async fn user_unblock(session: Session, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let _user = get_user(*user_id);
        _request_user.unblock_user(_user);
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
    } else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct TrackData {
    pub url:    String,
    pub title:  String,
    pub image:  String,
}
#[derive(Deserialize, Serialize, Debug)]
struct TracksData {
    pub tracks: Vec<TrackData>,
}
pub async fn save_playlist(session: Session, list_id: web::Path<i32>) -> web::Json<TracksData> {
    let mut tracks_list: TracksData = TracksData {
        tracks: Vec::new(),
    };

    if is_signed_in(&session) {
        use crate::utils::get_music_list;

        let _request_user = get_request_user_data(&session);
        let _list = get_music_list(*list_id);
        _list.save_playlist(_request_user);
        for track in _list.get_items().iter() {
            tracks_list.tracks.push(TrackData {
                url:   track.file.clone(),
                title: track.title.clone(),
                image: track.get_image(),
            });
        }
        return Json(tracks_list)
    } else {
        return Json(tracks_list)
    }
}
