use diesel::prelude::*;
use crate::schema;
use crate::schema::{
    notifications,
    wall_objects,
};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::models::{User, Community};


/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct Notification {
    pub id:                  i32,
    pub recipient_id:        Option<i32>,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,     // тип уведомления
    pub status:              String,
    pub types:               i16,        // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,  // Например, человек лайкает несколько постов. Нужно для группировки
    pub object_set_id:       Option<i32>,  // Например, несколько человек лайкает пост. Нужно для группировки
}
#[derive(Deserialize, Insertable)]
#[table_name="notifications"]
pub struct NewNotification {
    pub recipient_id:        Option<i32>,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}
impl Notification {
    // is_group:     нужна ли спайка сигналов в группу
    // is_community: создаются сигналы сообщества
    pub fn create_notify(creator: &User, verb: String, types: i16,
        object_id: i32, community: Option<Community>, action_community_id: Option<i32>,
        is_group: bool, is_community: bool) -> () {

        let user_set_id: Option<i32>;
        let object_set_id: Option<i32>;
        let users_ids: Vec<i32>;
        if is_group {
            user_set_id = None;
            object_set_id = None;
        }
        else {
            user_set_id = None;
            object_set_id = None;
        }
        let community_id: Option<i32>;
        let _connection = establish_connection();
        if is_community && community.is_some(){
            users_ids = community.as_ref().unwrap().get_members_for_notify_ids();
            community_id = Some(community.unwrap().id);
        }
        else {
            users_ids = creator.get_users_ids_for_main_news();
            community_id = None;
        }
        for user_id in users_ids.iter() {
            let new_notify = NewNotification {
                recipient_id: Some(*user_id),
                user_id: creator.id,
                created: chrono::Local::now().naive_utc(),
                verb: verb.clone(),
                status: "a".to_string(),
                types: types,
                object_id: object_id,
                community_id: community_id,
                action_community_id: action_community_id,
                user_set_id: user_set_id,
                object_set_id: object_set_id,
            };
            diesel::insert_into(schema::notifications::table)
                .values(&new_notify)
                .get_result::<Notification>(&_connection)
                .expect("Error.");
        }
    }
}

/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
pub struct WallObject {
    pub id:                  i32,
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,         // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>, // Например, человек лайкает несколько постов. Нужно для группировки
    pub object_set_id:       Option<i32>, //Например, несколько человек лайкает пост. Нужно для группировки
}
#[derive(Deserialize, Insertable)]
#[table_name="wall_objects"]
pub struct NewWallObject {
    pub user_id:             i32,
    pub created:             chrono::NaiveDateTime,
    pub verb:                String,
    pub status:              String,
    pub types:               i16,  // описан в модерации тип объекта
    pub object_id:           i32,
    pub community_id:        Option<i32>,
    pub action_community_id: Option<i32>,
    pub user_set_id:         Option<i32>,
    pub object_set_id:       Option<i32>,
}

impl WallObject {
    // is_group:     нужна ли спайка сигналов в группу
    pub fn create_wall(creator: &User, verb: String, types: i16,
        object_id: i32, community_id: Option<i32>, action_community_id: Option<i32>,
        is_group: bool) -> () {

        let user_set_id: Option<i32>;
        let object_set_id: Option<i32>;
        let users_ids: Vec<i32>;
        if is_group {
            user_set_id = None;
            object_set_id = None;
        }
        else {
            user_set_id = None;
            object_set_id = None;
        }

        let _connection = establish_connection();
        let new_wall = NewWallObject {
            user_id: creator.id,
            created: chrono::Local::now().naive_utc(),
            verb: verb,
            status: "a".to_string(),
            types: types,
            object_id: object_id,
            community_id: community_id,
            action_community_id: action_community_id,
            user_set_id: user_set_id,
            object_set_id: object_set_id,
        };
        diesel::insert_into(schema::wall_objects::table)
            .values(&new_wall)
            .get_result::<WallObject>(&_connection)
            .expect("Error.");
    }
}
