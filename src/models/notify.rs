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


/////// verb annotation //////
// ITE разместил, SIT предложил, ITS разместили
// COM оставил, WCOM оставила, GCOM оставили
// REP ответил на, WREP ответила на, GREP ответили на

// PUM упомянул Вас в записи, WPUM упомянула Вас в записи, GPUM упомянули Вас в записи
// PCUM упомянул Вас в комментарии к записи, WPCUM упомянула Вас в комментарии к записи, GPCUM упомянули Вас  в комментарии к записи

// RPO отреагировал на запись, WRPO отреагировала на запись, GRPO отреагировали на запись
// RCO отреагировал на комментарий, WRCO отреагировала на комментарий, GRCO отреагировали на комментарий
// RRE отреагировал на ответ, WRRE отреагировала на ответ, GRE отреагировали на ответ
// RME отреагировал на сообщение, GRME отреагировала на сообщение, WRME отреагировали на сообщение,

// RE поделился, WRE поделилась, GRE поделились,
// CR поделилось, GCR поделились,
// LRE поделился списком, WLRE поделилась, GLRE поделились,
// CLR поделилось списком, GCLR поделились,

// CRE подал заявку в, WCRE подала заявку в, GCRE подали заявку в,
// CCO приняты в друзья, WCCO принята, GCCO приняты,
// INV рекомендует, WINV рекомендует, GINV рекомендуют,
// CJO принят в сообщество, WCJO принята, WCJO приняты'),
// REG зарегистрировался, WREG зарегистрировалась, GREG зарегистрировались'),

/////// Notification //////
////////// статус уведомления
    // 'a' Не прочитано
    // 'b' Прочитано
    // 'c' Удалено
    // 'd' Закрыто

pub fn get_verb(verb: &str, is_women: bool) -> (String, String, String) {
    let words: Vec<&str> = verb.split(" ").collect();
    let mut word_ilike = "".to_string();
    let mut gender_word = "".to_string();
    let mut group_word = "".to_string();
    let mut new_verb = "".to_string();

    for (count, word) in words.iter().enumerate() {
        if count == 0 {
            word_ilike = word.to_string();
            if is_women {
                gender_word = word.to_string() + &"а".to_string();
            }
            else {
                gender_word = word.to_string()
            }
            let pop_word = word.to_string().pop();
            group_word = pop_word.unwrap().to_string() + &"и".to_string();
        }
        else {
            new_verb.push_str(&word.to_string());
            new_verb.push_str(&" ".to_string());
        }
    }
    let curr_group_verb = group_word + &" ".to_string() + &new_verb;
    new_verb = gender_word + &" ".to_string() + &new_verb;
    return (word_ilike, curr_group_verb, new_verb);
}

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
    pub fn get_creator(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.user_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_recipient(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.recipient_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_action_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.action_community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn is_have_object_set(&self) -> bool {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::object_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<Notification>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_object_set(&self, limit: i64, offset: i64) -> Vec<Notification> {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::object_set_id.eq(self.id))
            .or_filter(schema::notifications::id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<Notification>(&_connection)
            .expect("E");
    }
    pub fn count_object_set(&self) -> String {
        use crate::utils::get_count_usize_for_ru;
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        let count = notifications
            .filter(schema::notifications::object_set_id.eq(self.id))
            .or_filter(schema::notifications::id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .load::<Notification>(&_connection)
            .expect("E")
            .len() + 1;

        return get_count_usize_for_ru(
            count,
            " человек".to_string(),
            " человека".to_string(),
            " людей".to_string(),
        );
    }
    pub fn get_6_object_set(&self) -> Vec<Notification> {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::object_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(6)
            .load::<Notification>(&_connection)
            .expect("E");
    }
    pub fn get_first_object_set(&self) -> Notification {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::object_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn is_have_user_set(&self) -> bool {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::user_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<Notification>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_user_set(&self, limit: i64, offset: i64) -> Vec<Notification> {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::user_set_id.eq(self.id))
            .or_filter(schema::notifications::id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<Notification>(&_connection)
            .expect("E");
    }
    pub fn count_user_set(&self) -> String {
        use crate::utils::get_count_usize_for_ru;
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        let count = notifications
            .filter(schema::notifications::user_set_id.eq(self.id))
            .or_filter(schema::notifications::id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .load::<Notification>(&_connection)
            .expect("E")
            .len() + 1;

        return get_count_usize_for_ru(
            count,
            " раз".to_string(),
            " раза".to_string(),
            " раз".to_string(),
        );
    }
    pub fn get_6_user_set(&self) -> Vec<Notification> {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::user_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(6)
            .load::<Notification>(&_connection)
            .expect("E");
    }
    pub fn get_first_user_set(&self) -> Notification {
        use crate::schema::notifications::dsl::notifications;

        let _connection = establish_connection();
        return notifications
            .filter(schema::notifications::user_set_id.eq(self.id))
            .filter(schema::notifications::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
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
    pub fn get_creator(&self) -> User {
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return users
            .filter(schema::users::id.eq(self.user_id))
            .filter(schema::users::types.lt(10))
            .load::<User>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }
    pub fn get_action_community(&self) -> Community {
        use crate::schema::communitys::dsl::communitys;

        let _connection = establish_connection();
        return communitys
            .filter(schema::communitys::id.eq(self.action_community_id.unwrap()))
            .filter(schema::communitys::types.lt(10))
            .load::<Community>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn is_have_object_set(&self) -> bool {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::object_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<WallObject>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_object_set(&self, limit: i64, offset: i64) -> Vec<WallObject> {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::object_set_id.eq(self.id))
            .or_filter(schema::wall_objects::id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<WallObject>(&_connection)
            .expect("E");
    }
    pub fn count_object_set(&self) -> String {
        use crate::utils::get_count_usize_for_ru;
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        let count = wall_objects
            .filter(schema::wall_objects::object_set_id.eq(self.id))
            .or_filter(schema::wall_objects::id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .load::<WallObject>(&_connection)
            .expect("E")
            .len();

        return get_count_usize_for_ru(
            count,
            " человек".to_string(),
            " человека".to_string(),
            " людей".to_string(),
        );
    }
    pub fn get_6_object_set(&self) -> Vec<WallObject> {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::object_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(6)
            .load::<WallObject>(&_connection)
            .expect("E");
    }

    pub fn get_first_object_set(&self) -> WallObject {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::object_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn is_have_user_set(&self) -> bool {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::user_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<WallObject>(&_connection)
            .expect("E")
            .len() > 0;
    }
    pub fn get_user_set(&self, limit: i64, offset: i64) -> Vec<WallObject> {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::user_set_id.eq(self.id))
            .or_filter(schema::wall_objects::id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(limit)
            .offset(offset)
            .load::<WallObject>(&_connection)
            .expect("E");
    }
    pub fn count_user_set(&self) -> String {
        use crate::utils::get_count_usize_for_ru;
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        let count = wall_objects
            .filter(schema::wall_objects::user_set_id.eq(self.id))
            .or_filter(schema::wall_objects::id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .load::<WallObject>(&_connection)
            .expect("E")
            .len() + 1;

        return get_count_usize_for_ru(
            count,
            " раз".to_string(),
            " раза".to_string(),
            " раз".to_string(),
        );
    }
    pub fn get_6_user_set(&self) -> Vec<WallObject> {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::user_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(6)
            .load::<WallObject>(&_connection)
            .expect("E");
    }

    pub fn get_first_user_set(&self) -> WallObject {
        use crate::schema::wall_objects::dsl::wall_objects;

        let _connection = establish_connection();
        return wall_objects
            .filter(schema::wall_objects::user_set_id.eq(self.id))
            .filter(schema::wall_objects::status.eq_any(vec!["a","b"]))
            .limit(1)
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();
    }

    pub fn get_comment_verb(&self) -> String {
        return match self.types {
            81 => {
                use crate::utils::get_post_comment;
                let comment = get_post_comment(self.object_id);
                comment.get_small_content()
            },
            82 => {
                use crate::utils::get_photo_comment;
                let comment = get_photo_comment(self.object_id);
                comment.get_small_content()
            },
            _ => "".to_string(),
        };
    }
    pub fn get_verb(&self) -> String {
        let verb = &self.verb;

        // можно добавить текст комментов и так далее...
        //if self.types > 80 && self.types < 100 {
        //    verb = self.verb.clone() + &self.get_comment_verb();
        //}
        //else {
        //    verb = self.verb.clone();
        //}

        if verb.contains("опуб") {
            return "".to_string();
        }
        else {
            if self.is_have_object_set() {
                let first_notify = self.get_first_object_set();
                let creator = first_notify.get_creator();
                return concat_string!(
                    "<p style='padding-left: 7px;'><a href='",
                    creator.link,
                    "' class='ajax' style='font-weight: bold;'>",
                    creator.get_full_name(),
                    "</a> и ещё ",
                    self.count_object_set(),
                    verb,
                    " </p>"
                );
            }
            else if self.is_have_user_set() {
                let creator = self.get_creator();
                return concat_string!(
                    "<p style='padding-left: 7px'><a href='",
                    creator.link,
                    "' class='ajax' style='font-weight: bold;'>",
                    creator.get_full_name(),
                    "</a> ",
                    verb,
                    self.count_user_set(),
                    "</p>"
                );
            }
            else {
                let creator = self.get_creator();
                return concat_string!(
                    "<p style='padding-left: 7px'><a href='",
                    creator.link,
                    "' class='ajax' style='font-weight: bold;'>",
                    creator.get_full_name(),
                    "</a> ",
                    verb,
                    "</p>"
                );
            }
        }
    }
    pub fn get_wall_item(&self, user_id: i32, is_staff: bool) -> String {
        let verb = self.get_verb();
        let item = self.get_item(user_id, is_staff);
        return verb + &item;
    }
    pub fn get_item(&self, user_id: i32, is_staff: bool) -> String {
        return match self.types {
            1 => {
                use crate::utils::add_user;
                add_user(self.object_id)
            },
            2 => {
                use crate::utils::add_community;
                add_community(self.object_id)
            },
            3 => "".to_string(),
            4 => "".to_string(),
            20 => {
                use crate::utils::add_post_list;
                add_post_list(self.object_id)
            },
            21 => {
                use crate::utils::add_music_list;
                add_music_list(self.object_id)
            },
            22 => {
                use crate::utils::add_doc_list;
                add_doc_list(self.object_id)
            },
            23 => {
                //use crate::utils::add_survey_list;
                //add_survey_list(self.object_id)
                "".to_string()
            },
            24 => {
                use crate::utils::add_photo_list;
                add_photo_list(self.object_id)
            },
            25 => {
                use crate::utils::add_video_list;
                add_video_list(self.object_id)
            },
            26 => {
                use crate::utils::add_good_list;
                add_good_list(self.object_id)
            },

            51 => {
                use crate::utils::add_post;
                add_post(self.object_id, user_id, is_staff)
            },
            81 => {
                use crate::utils::add_post;
                add_post(self.object_id, user_id, is_staff)
            },
            87 => {
                use crate::utils::add_post;
                add_post(self.object_id, user_id, is_staff)
            },

            52 => {
                use crate::utils::add_music;
                add_music(self.object_id, is_staff, user_id, "music_list_item".to_string())
            },
            53 => {
                use crate::utils::add_doc;
                add_doc(self.object_id, is_staff, user_id)
            },
            54 => {
                use crate::utils::add_survey;
                add_survey(self.object_id, is_staff, user_id)
            },

            55 => {
                use crate::utils::add_photo;
                add_photo(self.object_id, "detail_photo".to_string())
            },
            82 => {
                use crate::utils::add_photo;
                add_photo(self.object_id, "detail_photo".to_string())
            },
            88 => {
                use crate::utils::add_photo;
                add_photo(self.object_id, "detail_photo".to_string())
            },

            56 => {
                use crate::utils::add_video;
                add_video(self.object_id, "video_list_detail".to_string())
            },
            83 => {
                use crate::utils::add_video;
                add_video(self.object_id, "video_list_detail".to_string())
            },
            89 => {
                use crate::utils::add_video;
                add_video(self.object_id, "video_list_detail".to_string())
            },

            57 => {
                use crate::utils::add_good;
                add_good(self.object_id)
            },
            84 => {
                use crate::utils::add_good;
                add_good(self.object_id)
            },
            90 => {
                use crate::utils::add_good;
                add_good(self.object_id)
            },

            _ => "".to_string(),
        };

    }
}

// is_group:     нужна ли спайка сигналов в группу
pub fn create_wall(creator_id: i32, verb: String, types: i16,
    object_id: i32, community_id: Option<i32>, action_community_id: Option<i32>,
    user_set_id: Option<i32>, object_set_id: Option<i32>) -> () {

    let _connection = establish_connection();
    let new_wall = NewWallObject {
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
    diesel::insert_into(schema::wall_objects::table)
        .values(&new_wall)
        .get_result::<WallObject>(&_connection)
        .expect("Error.");
}

// is_group: нужна ли спайка сигналов в группу
// нужна переменная parent_id - ответ на какой коммент
// нужна переменная owner_id - на какой объект написан коммент или ответ
pub fn create_user_wall(creator: &User, verb: String, types: i16,
    object_id: i32, action_community_id: Option<i32>,
    is_group: bool) -> () {
    use crate::schema::wall_objects::dsl::wall_objects;
    use chrono::Duration;

    let creator_id = creator.id;
    let _connection = establish_connection();
    let (word_ilike, group_word, current_verb) = get_verb(&verb, creator.is_women());
    let date = chrono::Local::now().naive_utc();

    if is_group {
        // если вложенность уведомлений включена
        if types < 3 {
            println!("Пользователь или сообщество");
            // если объект - пользователь или сообщество
            let wall_exists = wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::verb.like(&word_ilike))
                .load::<WallObject>(&_connection)
                .expect("E");
            if wall_exists.len() > 0 {
                println!("подобное уведомление уже создавалось");
                // если подобное уведомление уже создавалось
                return
            }
            else {
                create_wall (
                    creator_id,
                    current_verb.to_string(),
                    types,
                    object_id,
                    None,
                    action_community_id,
                    None,
                    None,
                );
            }
        }
        else {
            // если объект общего порядка
            println!("объект общего порядка");
            println!("word_ilike {:?}", &word_ilike);
            if wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::verb.like(&word_ilike))
                .load::<WallObject>(&_connection)
                .expect("E")
                .len() > 0 {

                println!("уведомление уже создавалось");
                // если подобное уведомление уже создавалось
                return
            }
            // если пользователь уже совершал сегодня такие действия
            // на аналогичные объекты по типу
            else if wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::types.eq(types))
                //.filter(schema::wall_objects::created.gt(date - Duration::hours(24)))
                //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::verb.ilike(&word_ilike))
                .load::<WallObject>(&_connection)
                .expect("E")
                .len() > 0 {

                println!("пользователь уже совершал сегодня такие действия на аналогичные объекты по типу");
                let wall = wall_objects
                    .filter(schema::wall_objects::user_id.eq(creator.id))
                    .filter(schema::wall_objects::types.eq(types))
                    //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                    //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
                    .filter(schema::wall_objects::verb.ilike(&word_ilike))
                    .filter(schema::wall_objects::user_set_id.is_null())
                    .load::<WallObject>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                create_wall (
                    creator_id,
                    current_verb.clone(),
                    types,
                    object_id,
                    None,
                    action_community_id,
                    Some(wall.id),
                    None,
                );
            }
            // если пользователи уже совершали сегодня такие действия
            // на объект по типу
            else if wall_objects
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::types.eq(types))
                //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::verb.ilike(&word_ilike))
                .load::<WallObject>(&_connection)
                .expect("E")
                .len() > 0 {

                println!("пользователи уже совершали сегодня такие действия на объект по типу");
                let wall = wall_objects
                    .filter(schema::wall_objects::object_id.eq(object_id))
                    .filter(schema::wall_objects::types.eq(types))
                    //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                    //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
                    .filter(schema::wall_objects::verb.ilike(&word_ilike))
                    .filter(schema::wall_objects::object_set_id.is_null())
                    .load::<WallObject>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                create_wall (
                    creator_id,
                    group_word,
                    types,
                    object_id,
                    None,
                    action_community_id,
                    None,
                    Some(wall.id),
                );
            }
            // если пользоваели еще не создавали уведомлений на объект
            else {
                println!("пользователи еще не создавали уведомлений на объект");
                create_wall (
                    creator_id,
                    current_verb.clone(),
                    types,
                    object_id,
                    None,
                    action_community_id,
                    None,
                    None,
                );
            }
        }
    }
    else {
        println!("пользователи еще не создавали уведомлений");
        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            None,
            action_community_id,
            None,
            None,
        );
    }
}

// is_group: нужна ли спайка сигналов в группу
pub fn create_community_wall(creator: &User, community: &Community,
    verb: String, types: i16, object_id: i32,
    action_community_id: Option<i32>, is_group: bool) -> () {
    use crate::schema::wall_objects::dsl::wall_objects;
    use chrono::Duration;

    let creator_id = creator.id;
    let community_id = Some(community.id);
    let _connection = establish_connection();
    let (first_word, group_word, current_verb) = get_verb(&verb, creator.is_women());
    let date = chrono::Local::now().naive_utc();

    if is_group {
        // если вложенность уведомлений включена
        if types < 3 {
            // если объект - пользователь или сообщество
            let wall_exists = wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::community_id.eq(community_id))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::verb.eq(verb))
                .load::<WallObject>(&_connection)
                .expect("E");
            if wall_exists.len() > 0 {
                // если подобное уведомление уже создавалось
                return
            }
            else {
                create_wall (
                    creator_id,
                    current_verb.to_string(),
                    types,
                    object_id,
                    community_id,
                    action_community_id,
                    None,
                    None,
                );
            }
        }
        else {
            // если объект общего порядка
            let wall_exists = wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::community_id.eq(community_id))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::verb.like("%".to_owned() + &first_word + &"%".to_string()))
                .load::<WallObject>(&_connection)
                .expect("E");
            if wall_exists.len() > 0 {
                // если подобное уведомление уже создавалось
                return
            }

            // если пользователь уже совершал сегодня такие действия
            // на аналогичные объекты по типу
            else if wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::community_id.eq(community_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::created.gt(date - Duration::hours(24)))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::user_set_id.is_null())
                .filter(schema::wall_objects::types.eq(types))
                .load::<WallObject>(&_connection)
                .expect("E")
                .len() > 0 {

            let notify = wall_objects
                .filter(schema::wall_objects::user_id.eq(creator.id))
                .filter(schema::wall_objects::community_id.eq(community_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::verb.eq(current_verb.clone()))
                .load::<WallObject>(&_connection)
                .expect("E")
                .into_iter()
                .nth(0)
                .unwrap();

                create_wall (
                    creator_id,
                    current_verb.clone(),
                    types,
                    object_id,
                    community_id,
                    action_community_id,
                    Some(notify.id),
                    None,
                );
            }
            // если пользователи уже совершали сегодня такие действия
            // на объект по типу
            else if wall_objects
                .filter(schema::wall_objects::object_id.eq(object_id))
                .filter(schema::wall_objects::community_id.eq(community_id))
                .filter(schema::wall_objects::types.eq(types))
                .filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                .filter(schema::wall_objects::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
                .filter(schema::wall_objects::object_set_id.is_null())
                .load::<WallObject>(&_connection)
                .expect("E")
                .len() > 0 {

                let notify = wall_objects
                    .filter(schema::wall_objects::object_id.eq(object_id))
                    .filter(schema::wall_objects::community_id.eq(community_id))
                    .filter(schema::wall_objects::types.eq(types))
                    .filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
                    .filter(schema::wall_objects::action_community_id.eq(action_community_id))
                    .filter(schema::wall_objects::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
                    .load::<WallObject>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                create_wall (
                    creator_id,
                    group_word,
                    types,
                    object_id,
                    community_id,
                    action_community_id,
                    None,
                    Some(notify.id),
                );
            }
            // если пользоваели еще не создавали уведомлений на объект
            else {
                create_wall (
                    creator_id,
                    current_verb.clone(),
                    types,
                    object_id,
                    community_id,
                    action_community_id,
                    None,
                    None,
                );
            }
        }
    }
    else {
        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            None,
            None,
        );
    }
}

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
    use crate::schema::notifications::dsl::notifications;
    use chrono::Duration;

    let creator_id = creator.id;
    let _connection = establish_connection();
    let (first_word, group_word, current_verb) = get_verb(&verb, creator.is_women());
    let (users_ids, _communities_ids) = creator.get_ids_for_notifications();
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
                create_notify (
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
                    .filter(schema::notifications::verb.like("%".to_owned() + &first_word + &"%".to_string()))
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
                    .filter(schema::notifications::types.eq(types))
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
                    .filter(schema::notifications::verb.eq(current_verb.clone()))
                    .load::<Notification>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                    create_notify (
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
                    .filter(schema::notifications::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
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
                    .filter(schema::notifications::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
                    .load::<Notification>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                    create_notify (
                        creator_id,
                        *user_id,
                        group_word.clone(),
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
                    create_notify (
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
            create_notify (
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

// is_group: нужна ли спайка сигналов в группу
pub fn create_community_notify(creator: &User, community: &Community,
    verb: String, types: i16, object_id: i32,
    action_community_id: Option<i32>, is_group: bool) -> () {
    use crate::schema::notifications::dsl::notifications;
    use chrono::Duration;

    let creator_id = creator.id;
    let community_id = Some(community.id);
    let _connection = establish_connection();
    let (first_word, group_word, current_verb) = get_verb(&verb, creator.is_women());
    let (_users_ids, communities_ids) = creator.get_ids_for_notifications();
    let date = chrono::Local::now().naive_utc();

    if is_group {
        // если вложенность уведомлений включена
        if types < 3 {
            // если объект - пользователь или сообщество
            let notifications_exists = notifications
                .filter(schema::notifications::user_id.eq(creator.id))
                .filter(schema::notifications::community_id.eq(community_id))
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
                create_notify (
                    creator_id,
                    object_id,
                    current_verb.to_string(),
                    types,
                    object_id,
                    community_id,
                    action_community_id,
                    None,
                    None,
                )
            }
        }
        else {
            // если объект общего порядка
            for user_id in communities_ids.iter() {
                let notifications_exists = notifications
                    .filter(schema::notifications::user_id.eq(creator.id))
                    .filter(schema::notifications::recipient_id.eq(user_id))
                    .filter(schema::notifications::community_id.eq(community_id))
                    .filter(schema::notifications::action_community_id.eq(action_community_id))
                    .filter(schema::notifications::object_id.eq(object_id))
                    .filter(schema::notifications::types.eq(types))
                    .filter(schema::notifications::verb.like("%".to_owned() + &first_word + &"%".to_string()))
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
                    .filter(schema::notifications::community_id.eq(community_id))
                    .filter(schema::notifications::types.eq(types))
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
                    .filter(schema::notifications::community_id.eq(community_id))
                    .filter(schema::notifications::types.eq(types))
                    .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                    .filter(schema::notifications::action_community_id.eq(action_community_id))
                    .filter(schema::notifications::verb.eq(current_verb.clone()))
                    .load::<Notification>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                    create_notify (
                        creator_id,
                        *user_id,
                        current_verb.to_string(),
                        types,
                        object_id,
                        community_id,
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
                    .filter(schema::notifications::community_id.eq(community_id))
                    .filter(schema::notifications::types.eq(types))
                    .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                    .filter(schema::notifications::action_community_id.eq(action_community_id))
                    .filter(schema::notifications::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
                    .filter(schema::notifications::object_set_id.is_null())
                    .load::<Notification>(&_connection)
                    .expect("E")
                    .len() > 0 {

                let notify = notifications
                    .filter(schema::notifications::object_id.eq(object_id))
                    .filter(schema::notifications::recipient_id.eq(user_id))
                    .filter(schema::notifications::community_id.eq(community_id))
                    .filter(schema::notifications::types.eq(types))
                    .filter(schema::notifications::created.eq(date - Duration::hours(24)))
                    .filter(schema::notifications::action_community_id.eq(action_community_id))
                    .filter(schema::notifications::verb.ilike("%".to_owned() + &first_word + &"%".to_string()))
                    .load::<Notification>(&_connection)
                    .expect("E")
                    .into_iter()
                    .nth(0)
                    .unwrap();

                    create_notify (
                        creator_id,
                        *user_id,
                        group_word.clone(),
                        types,
                        object_id,
                        community_id,
                        action_community_id,
                        None,
                        Some(notify.id),
                    )
                }
                // если пользоваели еще не создавали уведомлений на объект
                else {
                    create_notify (
                        creator_id,
                        *user_id,
                        current_verb.clone(),
                        types,
                        object_id,
                        community_id,
                        action_community_id,
                        None,
                        None,
                    )
                }
            }
        }
    }
    else {
        for user_id in communities_ids.iter() {
            create_notify (
                creator_id,
                *user_id,
                current_verb.clone(),
                types,
                object_id,
                community_id,
                action_community_id,
                None,
                None,
            )
        }
    }
}


// нужна переменная parent_id - ответ на какой коммент
// нужна переменная owner_id - на какой объект написан коммент или ответ
pub fn create_comment_user_wall(creator: &User, verb: String, types: i16,
    object_id: i32, action_community_id: Option<i32>,
    comment_id: i32, parent_id: Option<i32>) -> () {
    use crate::schema::wall_objects::dsl::wall_objects;
    use chrono::Duration;

    let creator_id = creator.id;
    let dop_verb: String;
    let _connection = establish_connection();

    let date = chrono::Local::now().naive_utc();
    if parent_id.is_some() {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_reply_comment' reply-comment-pk='",
            comment_id.to_string(),
            "'>ответ</a> на <a class='underline pointer show_parent_comment' parent-comment-pk='",
            parent_id.unwrap().to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    else {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_parent_comment' parent-comment-pk='",
            comment_id.to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    let (word_ilike, group_word, current_verb) = get_verb(&dop_verb, creator.is_women());

    // если пользователь уже совершал сегодня такие действия
    // на аналогичные объекты по типу
    if wall_objects
        .filter(schema::wall_objects::user_id.eq(creator_id))
        .filter(schema::wall_objects::types.eq(types))
        .filter(schema::wall_objects::object_id.eq(object_id))
        //.filter(schema::wall_objects::created.gt(date - Duration::hours(24)))
        .filter(schema::wall_objects::verb.like("написал"))
        .load::<WallObject>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователь уже совершал сегодня такие действия на аналогичные объекты по типу");
        let wall = wall_objects
            .filter(schema::wall_objects::user_id.eq(creator_id))
            .filter(schema::wall_objects::types.eq(types))
            .filter(schema::wall_objects::object_id.eq(object_id))
            //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
            .filter(schema::wall_objects::verb.like("написал"))
            .filter(schema::wall_objects::user_set_id.is_null())
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            None,
            action_community_id,
            Some(wall.id),
            None,
        );
    }
    // если пользователи уже совершали сегодня такие действия
    // на объект по типу
    else if wall_objects
        .filter(schema::wall_objects::object_id.eq(object_id))
        .filter(schema::wall_objects::types.eq(types))
        //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
        .filter(schema::wall_objects::verb.like("написал"))
        .load::<WallObject>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователи уже совершали сегодня такие действия на объект по типу");
        let wall = wall_objects
            .filter(schema::wall_objects::object_id.eq(object_id))
            .filter(schema::wall_objects::types.eq(types))
            //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
            .filter(schema::wall_objects::verb.like("написал"))
            .filter(schema::wall_objects::object_set_id.is_null())
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_wall (
            creator_id,
            group_word,
            types,
            object_id,
            None,
            action_community_id,
            None,
            Some(wall.id),
        );
    }
    // если пользоваели еще не создавали уведомлений на объект
    else {
        println!("пользователи еще не создавали уведомлений на объект");
        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            None,
            action_community_id,
            None,
            None,
        );
    }
}

pub fn create_comment_community_wall(creator: &User, community: &Community,
    verb: String, types: i16, object_id: i32, action_community_id: Option<i32>,
    comment_id: i32, parent_id: Option<i32>) -> () {
    use crate::schema::wall_objects::dsl::wall_objects;
    use chrono::Duration;

    let creator_id = creator.id;
    let community_id = Some(community.id);
    let dop_verb: String;
    let _connection = establish_connection();

    let date = chrono::Local::now().naive_utc();
    if parent_id.is_some() {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_reply_comment' reply-comment-pk='",
            comment_id.to_string(),
            "'>ответ</a> на <a class='underline pointer show_parent_comment' parent-comment-pk='",
            parent_id.unwrap().to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    else {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_parent_comment' parent-comment-pk='",
            comment_id.to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    let (word_ilike, group_word, current_verb) = get_verb(&dop_verb, creator.is_women());

    // если пользователь уже совершал сегодня такие действия
    // на аналогичные объекты по типу
    if wall_objects
        .filter(schema::wall_objects::user_id.eq(creator_id))
        .filter(schema::wall_objects::types.eq(types))
        .filter(schema::wall_objects::object_id.eq(object_id))
        //.filter(schema::wall_objects::created.gt(date - Duration::hours(24)))
        //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
        .filter(schema::wall_objects::verb.ilike(&word_ilike))
        .load::<WallObject>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователь уже совершал сегодня такие действия на аналогичные объекты по типу");
        let wall = wall_objects
            .filter(schema::wall_objects::user_id.eq(creator_id))
            .filter(schema::wall_objects::types.eq(types))
            .filter(schema::wall_objects::object_id.eq(object_id))
            //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
            //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
            .filter(schema::wall_objects::verb.ilike(&word_ilike))
            .filter(schema::wall_objects::user_set_id.is_null())
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            Some(wall.id),
            None,
        );
    }
    // если пользователи уже совершали сегодня такие действия
    // на объект по типу
    else if wall_objects
        .filter(schema::wall_objects::object_id.eq(object_id))
        .filter(schema::wall_objects::types.eq(types))
        //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
        //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
        .filter(schema::wall_objects::verb.ilike(&word_ilike))
        .load::<WallObject>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователи уже совершали сегодня такие действия на объект по типу");
        let wall = wall_objects
            .filter(schema::wall_objects::object_id.eq(object_id))
            .filter(schema::wall_objects::types.eq(types))
            //.filter(schema::wall_objects::created.eq(date - Duration::hours(24)))
            //.filter(schema::wall_objects::action_community_id.eq(action_community_id))
            .filter(schema::wall_objects::verb.ilike(&word_ilike))
            .filter(schema::wall_objects::object_set_id.is_null())
            .load::<WallObject>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_wall (
            creator_id,
            group_word,
            types,
            object_id,
            community_id,
            action_community_id,
            None,
            Some(wall.id),
        );
    }
    // если пользоваели еще не создавали уведомлений на объект
    else {
        println!("пользователи еще не создавали уведомлений на объект");
        create_wall (
            creator_id,
            current_verb.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            None,
            None,
        );
    }
}


pub fn create_comment_user_notify(creator: &User, verb: String, types: i16,
    object_id: i32, action_community_id: Option<i32>,
    comment_id: i32, parent_id: Option<i32>) -> () {
    use crate::schema::notifications::dsl::notifications;
    use chrono::Duration;

    let creator_id = creator.id;
    let dop_verb: String;
    let _connection = establish_connection();
    let (users_ids, _communities_ids) = creator.get_ids_for_notifications();

    let date = chrono::Local::now().naive_utc();
    if parent_id.is_some() {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_reply_comment' reply-comment-pk='",
            comment_id.to_string(),
            "'>ответ</a> на <a class='underline pointer show_parent_comment' parent-comment-pk='",
            parent_id.unwrap().to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    else {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_parent_comment' parent-comment-pk='",
            comment_id.to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    let (word_ilike, group_word, current_verb) = get_verb(&dop_verb, creator.is_women());

    // если пользователь уже совершал сегодня такие действия
    // на аналогичные объекты по типу
    for user_id in users_ids.iter() {
        if notifications
        .filter(schema::notifications::user_id.eq(creator_id))
        .filter(schema::notifications::recipient_id.eq(user_id))
        .filter(schema::notifications::types.eq(types))
        .filter(schema::notifications::object_id.eq(object_id))
        //.filter(schema::notifications::created.gt(date - Duration::hours(24)))
        //.filter(schema::notifications::action_community_id.eq(action_community_id))
        .filter(schema::notifications::verb.ilike(&word_ilike))
        .load::<Notification>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователь уже совершал сегодня такие действия на аналогичные объекты по типу");
        let notify = notifications
            .filter(schema::notifications::user_id.eq(creator_id))
            .filter(schema::notifications::recipient_id.eq(user_id))
            .filter(schema::notifications::types.eq(types))
            .filter(schema::notifications::object_id.eq(object_id))
            //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
            //.filter(schema::notifications::action_community_id.eq(action_community_id))
            .filter(schema::notifications::verb.ilike(&word_ilike))
            .filter(schema::notifications::user_set_id.is_null())
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_notify (
            creator_id,
            *user_id,
            current_verb.clone(),
            types,
            object_id,
            None,
            action_community_id,
            Some(notify.id),
            None,
        );
        }
        // если пользователи уже совершали сегодня такие действия
        // на объект по типу
        else if notifications
        .filter(schema::notifications::object_id.eq(object_id))
        .filter(schema::notifications::recipient_id.eq(user_id))
        .filter(schema::notifications::types.eq(types))
        //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
        //.filter(schema::notifications::action_community_id.eq(action_community_id))
        .filter(schema::notifications::verb.ilike(&word_ilike))
        .load::<Notification>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователи уже совершали сегодня такие действия на объект по типу");
        let notify = notifications
            .filter(schema::notifications::object_id.eq(object_id))
            .filter(schema::notifications::recipient_id.eq(user_id))
            .filter(schema::notifications::types.eq(types))
            //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
            //.filter(schema::notifications::action_community_id.eq(action_community_id))
            .filter(schema::notifications::verb.ilike(&word_ilike))
            .filter(schema::notifications::object_set_id.is_null())
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_notify (
            creator_id,
            *user_id,
            group_word.clone(),
            types,
            object_id,
            None,
            action_community_id,
            None,
            Some(notify.id),
        );
        }
        // если пользоваели еще не создавали уведомлений на объект
        else {
        println!("пользователи еще не создавали уведомлений на объект");
        create_notify (
            creator_id,
            *user_id,
            current_verb.clone(),
            types,
            object_id,
            None,
            action_community_id,
            None,
            None,
        );
        }
    }
}

pub fn create_comment_community_notify(creator: &User, community: &Community,
    verb: String, types: i16, object_id: i32, action_community_id: Option<i32>,
    comment_id: i32, parent_id: Option<i32>) -> () {
    use crate::schema::notifications::dsl::notifications;
    use chrono::Duration;

    let creator_id = creator.id;
    let community_id = Some(community.id);
    let dop_verb: String;
    let _connection = establish_connection();
    let (_users_ids, communities_ids) = creator.get_ids_for_notifications();

    let date = chrono::Local::now().naive_utc();
    if parent_id.is_some() {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_reply_comment' reply-comment-pk='",
            comment_id.to_string(),
            "'>ответ</a> на <a class='underline pointer show_parent_comment' parent-comment-pk='",
            parent_id.unwrap().to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    else {
        dop_verb = concat_string!(
            "написал <a class='underline pointer show_parent_comment' parent-comment-pk='",
            comment_id.to_string(),
            "'>комментарий</a> к <a class='underline pointer show_owner_comment_item' owner-comment-pk='",
            object_id.to_string(), "'>", verb, "</a>"
        )
    }
    let (word_ilike, group_word, current_verb) = get_verb(&dop_verb, creator.is_women());
    for user_id in communities_ids.iter() {
        // если пользователь уже совершал сегодня такие действия
        // на аналогичные объекты по типу
        if notifications
        .filter(schema::notifications::user_id.eq(creator_id))
        .filter(schema::notifications::recipient_id.eq(user_id))
        .filter(schema::notifications::types.eq(types))
        .filter(schema::notifications::object_id.eq(object_id))
        //.filter(schema::notifications::created.gt(date - Duration::hours(24)))
        //.filter(schema::notifications::action_community_id.eq(action_community_id))
        .filter(schema::notifications::verb.ilike(&word_ilike))
        .load::<Notification>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователь уже совершал сегодня такие действия на аналогичные объекты по типу");
        let notify = notifications
            .filter(schema::notifications::user_id.eq(creator_id))
            .filter(schema::notifications::recipient_id.eq(user_id))
            .filter(schema::notifications::types.eq(types))
            .filter(schema::notifications::object_id.eq(object_id))
            //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
            //.filter(schema::notifications::action_community_id.eq(action_community_id))
            .filter(schema::notifications::verb.ilike(&word_ilike))
            .filter(schema::notifications::user_set_id.is_null())
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_notify (
            creator_id,
            *user_id,
            current_verb.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            Some(notify.id),
            None,
        );
        }
        // если пользователи уже совершали сегодня такие действия
        // на объект по типу
        else if notifications
        .filter(schema::notifications::object_id.eq(object_id))
        .filter(schema::notifications::recipient_id.eq(user_id))
        .filter(schema::notifications::types.eq(types))
        //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
        //.filter(schema::notifications::action_community_id.eq(action_community_id))
        .filter(schema::notifications::verb.ilike(&word_ilike))
        .load::<Notification>(&_connection)
        .expect("E")
        .len() > 0 {

        println!("пользователи уже совершали сегодня такие действия на объект по типу");
        let notify = notifications
            .filter(schema::notifications::object_id.eq(object_id))
            .filter(schema::notifications::recipient_id.eq(user_id))
            .filter(schema::notifications::types.eq(types))
            //.filter(schema::notifications::created.eq(date - Duration::hours(24)))
            //.filter(schema::notifications::action_community_id.eq(action_community_id))
            .filter(schema::notifications::verb.ilike(&word_ilike))
            .filter(schema::notifications::object_set_id.is_null())
            .load::<Notification>(&_connection)
            .expect("E")
            .into_iter()
            .nth(0)
            .unwrap();

        create_notify (
            creator_id,
            *user_id,
            group_word.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            None,
            Some(notify.id),
        );
        }
        // если пользоваели еще не создавали уведомлений на объект
        else {
        println!("пользователи еще не создавали уведомлений на объект");
        create_notify (
            creator_id,
            *user_id,
            current_verb.clone(),
            types,
            object_id,
            community_id,
            action_community_id,
            None,
            None,
        );
        }
    }
}
