-- Your SQL goes here

ALTER TABLE user_profiles DROP COLUMN playlist;
ALTER TABLE user_profiles ADD COLUMN saved_playlist
VARCHAR(100)NOT NULL DEFAULT '';
