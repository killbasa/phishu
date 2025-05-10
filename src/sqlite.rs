use anyhow::Result;
use once_cell::sync::Lazy;
use rusqlite::{Connection, params};
use std::{fs, sync::Mutex};

use crate::youtube::YoutubeVideo;

static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    fs::create_dir_all("data").expect("failed to create data dir");
    let conn = Connection::open("data/db.sqlite").expect("failed to open db");
    Mutex::new(conn)
});

pub fn init_db() {
    let conn = DB.lock().expect("failed to lock DB");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS videos (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			scheduled_time TEXT NOT NULL,
			start_time TEXT,
			end_time TEXT
		)",
        [],
    )
    .expect("failed to create table");
}

pub fn get_db_videos() -> Result<Vec<YoutubeVideo>> {
    let conn = DB.lock().expect("failed to lock DB");

    let mut stmt = conn.prepare(
        "SELECT id,title,scheduled_time,start_time,end_time
			FROM videos
			WHERE end_time is null
			ORDER BY scheduled_time DESC
			LIMIT 10",
    )?;

    let video_iter = stmt.query_map([], |row| {
        Ok(YoutubeVideo {
            id: row.get(0)?,
            title: row.get(1)?,
            scheduled_time: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
        })
    })?;

    let videos = video_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;

    Ok(videos)
}

pub fn get_db_most_recent_video() -> Result<Option<YoutubeVideo>> {
    let conn = DB.lock().expect("failed to lock DB");

    let mut stmt = conn.prepare(
        "SELECT id,title,scheduled_time,start_time,end_time
			FROM videos
			WHERE start_time is not null
			ORDER BY end_time DESC NULLS FIRST
			LIMIT 1",
    )?;

    let video_iter = stmt.query_map([], |row| {
        Ok(YoutubeVideo {
            id: row.get(0)?,
            title: row.get(1)?,
            scheduled_time: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
        })
    })?;

    let videos = video_iter.collect::<Result<Vec<_>, rusqlite::Error>>()?;

    Ok(videos.into_iter().next())
}

pub fn upsert_db_videos(videos: Vec<YoutubeVideo>) -> Result<()> {
    let mut conn = DB.lock().expect("failed to lock DB");
    let tx = conn.transaction()?;

    for video in videos {
        tx.execute(
            "INSERT OR REPLACE INTO videos (id,title,scheduled_time,start_time,end_time)
				VALUES (?1,?2,?3,?4,?5)",
            params![video.id, video.title, video.scheduled_time, video.start_time, video.end_time],
        )?;
    }

    tx.commit()?;
    Ok(())
}
