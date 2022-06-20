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
    config.route("/users/progs/save_playlist/{types}/", web::post().to(save_playlist));
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
pub struct TrackData {
    pub url:    String,
    pub title:  String,
    pub image:  String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaylistData {
    pub tracks:      Vec<TrackData>,
    pub name:        String,
    pub image:       String,
    pub description: String,
    pub types:       String,
}
pub async fn save_playlist(session: Session, types: web::Path<String>) -> web::Json<PlaylistData> {
    let mut data: PlaylistData = PlaylistData {
        tracks:      Vec::new(),
        name:        "".to_string(),
        image:       "".to_string(),
        description: "".to_string(),
        types:       "".to_string(),
    };

    if is_signed_in(&session) {
        use crate::models::Music;

        let _request_user = get_request_user_data(&session);
        let mut _types = types.into_inner();
        _request_user.save_playlist(&_types);

        let mut tracks: Vec<Music> = Vec::new();
        let mut name = "".to_string();
        let mut image = "".to_string();
        let mut description = "".to_string();

        if _types == "".to_string() {
            let playlist = _request_user.get_music_list();
            tracks = playlist.get_paginate_items(30,0);
            image = playlist.get_image();
            description = playlist.get_descriptions();
            name = playlist.name;
            _types = "lis".to_string() + &playlist.id.to_string();
        }
        else {
            let pk: i32 = _types[3..].parse().unwrap();
            let code = &_types[..3];

            if code == "lis".to_string() {
                use crate::utils::get_music_list;
                let playlist = get_music_list(pk);
                tracks = playlist.get_paginate_items(30,0);
                image = playlist.get_image();
                description = playlist.get_descriptions();
                name = playlist.name;
            }
            else if code == "pos".to_string() {
                use crate::utils::get_post;
                let post = get_post(pk);
                if post.community_id.is_some() {
                    let community = post.get_community();
                    name = community.name;
                    if community.b_avatar.is_some() {
                        image = community.b_avatar.as_deref().unwrap().to_string();
                    }
                    else {
                        image = "/static/images/news_small3.jpg".to_string();
                    }
                }
                else {
                    let creator = post.get_creator();
                    name = creator.get_full_name();
                    if creator.b_avatar.is_some() {
                        image = creator.b_avatar.as_deref().unwrap().to_string();
                    }
                    else {
                        image = "/static/images/news_small3.jpg".to_string();
                    }
                }
                tracks = post.get_attach_tracks();
                description = "Аудиозаписи поста".to_string();
            }
            else if code == "mes".to_string() {
                use crate::utils::get_message;
                let message = get_message(pk);

                let creator = message.get_creator();
                if creator.b_avatar.is_some() {
                    image = creator.b_avatar.as_deref().unwrap().to_string();
                }
                else {
                    image = "/static/images/news_small3.jpg".to_string();
                }
                tracks = message.get_attach_tracks();
                description = "Аудиозаписи сообщения".to_string();
                name = creator.get_full_name();
            }
            else if code == "cpo".to_string() {
                use crate::utils::get_post_comment;
                let comment = get_post_comment(pk);

                let creator = comment.get_creator();
                if creator.b_avatar.is_some() {
                    image = creator.b_avatar.as_deref().unwrap().to_string();
                }
                else {
                    image = "/static/images/news_small3.jpg".to_string();
                }
                tracks = comment.get_attach_tracks();
                description = "Аудиозаписи сообщения".to_string();
                name = creator.get_full_name();
            }
            else if code == "cgo".to_string() {
                use crate::utils::get_good_comment;
                let comment = get_good_comment(pk);

                let creator = comment.get_commenter();
                if creator.b_avatar.is_some() {
                    image = creator.b_avatar.as_deref().unwrap().to_string();
                }
                else {
                    image = "/static/images/news_small3.jpg".to_string();
                }
                tracks = comment.get_attach_tracks();
                description = "Аудиозаписи сообщения".to_string();
                name = creator.get_full_name();
            }
            else if code == "cph".to_string() {
                use crate::utils::get_photo_comment;
                let comment = get_photo_comment(pk);

                let creator = comment.get_commenter();
                if creator.b_avatar.is_some() {
                    image = creator.b_avatar.as_deref().unwrap().to_string();
                }
                else {
                    image = "/static/images/news_small3.jpg".to_string();
                }
                tracks = comment.get_attach_tracks();
                description = "Аудиозаписи сообщения".to_string();
                name = creator.get_full_name();
            }
            else if code == "cvi".to_string() {
                use crate::utils::get_video_comment;
                let comment = get_video_comment(pk);

                let creator = comment.get_commenter();
                if creator.b_avatar.is_some() {
                    image = creator.b_avatar.as_deref().unwrap().to_string();
                }
                else {
                    image = "/static/images/news_small3.jpg".to_string();
                }
                tracks = comment.get_attach_tracks();
                description = "Аудиозаписи сообщения".to_string();
                name = creator.get_full_name();
            }
        }

        let mut stack = Vec::new();
        for track in tracks.iter() {
            stack.push (
                TrackData {
                    url:   track.file.clone(),
                    title: track.title.clone(),
                    image: track.get_image(),
                });
        }
        data = PlaylistData {
            tracks:      stack,
            name:        name,
            image:       image,
            description: description,
            types: _types,
        };
        return Json(data)
    } else {
        return Json(data)
    }
}
