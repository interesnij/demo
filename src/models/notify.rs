use crate::schema;
use crate::schema::{
    notifications,
    wall_objects,
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    PgTextExpressionMethods,
    QueryDsl,
    TextExpressionMethods,
};
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
    pub recipient_id:        i32,
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
    pub recipient_id:        i32,
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
    pub fn create_notify(creator_id: i32, recipient_id: i32, verb: String, types: i16,
        object_id: i32, community_id: Option<i32>, action_community_id: Option<i32>,
        user_set_id: Option<i32>, object_set_id: Option<i32>) -> () {

        let _connection = establish_connection();
        let new_notify = NewNotification {
            recipient_id: recipient_id,
            user_id: creator_id,
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
        diesel::insert_into(schema::notifications::table)
            .values(&new_notify)
            .get_result::<Notification>(&_connection)
            .expect("Error.");
    }

    // is_group: нужна ли спайка сигналов в группу
    pub fn create_user_notify(creator: &User, verb: String, types: i16,
        object_id: i32, action_community_id: Option<i32>,
        is_group: bool) -> () {
        use crate::models::notify::notifications::dsl::notifications;
        use chrono::{Duration, Datelike};

        let creator_id = creator.id;
        let _connection = establish_connection();
        let current_verb = &creator.get_verb_gender(&verb);
        let users_ids = creator.get_users_ids_for_main_news();
        let date = chrono::Local::now().naive_utc();

        if is_group {
            // если вложенность уведомлений включена

            if types < 3 {
                // если объект - пользователь или сообщество
                let notifications_exists = notifications
                    .filter(schema::notifications::user_id.eq(creator.id))
                    .filter(schema::notifications::recipient_id.eq(object_id))
                    .filter(schema::notifications::action_community_id.eq(action_community_id))
                    .filter(schema::notifications::object_id.eq(object_id))
                    .filter(schema::notifications::types.eq(types))
                    .filter(schema::notifications::verb.eq(verb))
                    .load::<Notification>(&_connection)
                    .expect("E");
                if notifications_exists.len() > 0 {
                    // если подобное уведомление уже создавалось
                    return
                }
                else {
                    Notification::create_notify (
                        creator_id,
                        object_id,
                        current_verb.to_string(),
                        types,
                        object_id,
                        None,
                        action_community_id,
                        None,
                        None,
                    )
                }
            }
            else {
                // если объект общего порядка
                for user_id in users_ids.iter() {
                    let notifications_exists = notifications
                        .filter(schema::notifications::user_id.eq(creator.id))
                        .filter(schema::notifications::recipient_id.eq(user_id))
                        .filter(schema::notifications::action_community_id.eq(action_community_id))
                        .filter(schema::notifications::object_id.eq(object_id))
                        .filter(schema::notifications::types.eq(types))
                        .filter(schema::notifications::verb.like("%".to_owned() + &verb + &"%".to_string()))
                        .load::<Notification>(&_connection)
                        .expect("E");
                    if notifications_exists.len() > 0 {
                        // если подобное уведомление уже создавалось
                        return
                    }

                    // если пользователь уже совершал сегодня такие действия
                    // на аналогичные объекты по типу
                    else if notifications
                        .filter(schema::notifications::user_id.eq(creator.id))
                        .filter(schema::notifications::recipient_id.eq(user_id))
                        .filter(schema::notifications::types.eq(types))user_set_id
                        .filter(schema::notifications::created.gt(date - Duration::hours(24)))
                        .filter(schema::notifications::action_community_id.eq(action_community_id))
                        .filter(schema::notifications::user_set_id.is_null())
                        .filter(schema::notifications::types.eq(types))
                        .load::<Notification>(&_connection)
                        .expect("E")
                        .len() > 0 {

                    let notify = notifications
                        .filter(schema::notifications::user_id.eq(creator.id))
                        .filter(schema::notifications::recipient_id.eq(user_id))
                        .filter(schema::notifications::types.eq(types))
                        .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                        .filter(schema::notifications::action_community_id.eq(action_community_id))
                        .filter(schema::notifications::verb.eq(current_verb))
                        .load::<Notification>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0)
                        .unwrap();

                        Notification::create_notify (
                            creator_id,
                            *user_id,
                            current_verb.to_string(),
                            types,
                            object_id,
                            None,
                            action_community_id,
                            Some(notify.id),
                            None,
                        )
                    }
                    // если пользователи уже совершали сегодня такие действия
                    // на объект по типу
                    else if notifications
                        .filter(schema::notifications::object_id.eq(object_id))
                        .filter(schema::notifications::recipient_id.eq(user_id))
                        .filter(schema::notifications::types.eq(types))
                        .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                        .filter(schema::notifications::action_community_id.eq(action_community_id))
                        .filter(schema::notifications::verb.ilike("%".to_owned() + &verb + &"%".to_string()))
                        .filter(schema::notifications::object_set_id.is_null())
                        .load::<Notification>(&_connection)
                        .expect("E")
                        .len() > 0 {

                    let notify = notifications
                        .filter(schema::notifications::object_id.eq(object_id))
                        .filter(schema::notifications::recipient_id.eq(user_id))
                        .filter(schema::notifications::types.eq(types))
                        .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                        .filter(schema::notifications::action_community_id.eq(action_community_id))
                        .filter(schema::notifications::verb.ilike("%".to_owned() + &verb + &"%".to_string()))
                        .load::<Notification>(&_connection)
                        .expect("E")
                        .into_iter()
                        .nth(0)
                        .unwrap();

                        Notification::create_notify (
                            creator_id,
                            *user_id,
                            "G".to_string() + &verb,
                            types,
                            object_id,
                            None,
                            action_community_id,
                            None,
                            Some(notify.id),
                        )
                    }
                    // если пользоваели еще не создавали уведомлений на объект
                    else {
                        Notification::create_notify (
                            creator_id,
                            *user_id,
                            current_verb.clone(),
                            types,
                            object_id,
                            None,
                            action_community_id,
                            None,
                            None,
                        )
                    }
                }
            }
        }
        else {
            for user_id in users_ids.iter() {
                Notification::create_notify (
                    creator_id,
                    *user_id,
                    current_verb.clone(),
                    types,
                    object_id,
                    None,
                    action_community_id,
                    None,
                    None,
                )
            }
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
