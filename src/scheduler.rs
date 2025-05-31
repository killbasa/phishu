use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{
    pages, sqlite,
    youtube::{self, YoutubeVideo, videos},
};

pub async fn init_scheduler() -> Result<()> {
    let scheduler = JobScheduler::new().await?;

    // run every 15 minutes
    // min quota usage: 96
    scheduler
        .add(Job::new_async("0 0/15 * * * *", |_, _| {
            Box::pin(async {
                check_new_videos().await.expect("failed to check new videos");
            })
        })?)
        .await?;

    // run every 5 minutes (w/ 30 second delay for new videos)
    // min quota usage: 288
    scheduler
        .add(Job::new_async("30 0/5 * * * *", |_, _| {
            Box::pin(async {
                check_existing_videos().await.expect("failed to update videos");
            })
        })?)
        .await?;

    // run every 6 hours
    // min quota usage: 4
    scheduler
        .add(Job::new_async("0 0 0/6 * * *", |_, _| {
            Box::pin(async {
                pages::refresh_page(pages::Pages::Info).await.expect("failed to refresh info page");
            })
        })?)
        .await?;

    scheduler.start().await?;

    Ok(())
}

async fn check_new_videos() -> Result<()> {
    tracing::info!("checking for new videos");

    let xml_video_ids = videos::get_video_ids_xml().await?;

    if xml_video_ids.is_empty() {
        tracing::info!("no videos found (xml)");
        return Ok(());
    }

    match youtube::videos::get_videos_api(&xml_video_ids).await {
        Err(e) => {
            tracing::error!("failed to fetch videos: {}", e);
            Ok(())
        }
        Ok(api_videos) => {
            if api_videos.is_empty() {
                tracing::info!("no videos found (api)");
                return Ok(());
            }

            tracing::info!("found {} videos", api_videos.len());
            for api_video in &api_videos {
                tracing::debug!("upserting {}", api_video.id);
            }

            sqlite::upsert_db_videos(api_videos)?;

            pages::refresh_page(pages::Pages::Upcoming).await?;

            Ok(())
        }
    }
}

async fn check_existing_videos() -> Result<()> {
    tracing::info!("checking for new videos");

    let db_videos = sqlite::get_db_upcoming_videos()?;

    if db_videos.is_empty() {
        tracing::info!("no videos found (db)");
        return Ok(());
    }

    let db_video_ids: Vec<String> = db_videos //
        .iter()
        .map(|video| video.id.clone())
        .collect();

    match youtube::videos::get_videos_api(&db_video_ids).await {
        Err(e) => {
            tracing::error!("failed to fetch videos: {}", e);
            Ok(())
        }
        Ok(api_videos) => {
            if api_videos.is_empty() {
                tracing::info!("no videos found (api)");
                sqlite::delete_db_videos(&db_video_ids)?;

                return Ok(());
            }

            tracing::info!("found {} videos", api_videos.len());

            if db_videos.len() == db_video_ids.len() {
                for api_video in &api_videos {
                    tracing::debug!("upserting {}", api_video.id);
                }

                sqlite::upsert_db_videos(api_videos)?;

                return Ok(());
            }

            let mut api_videos_iter = api_videos.iter();
            let videos_to_delete: Vec<String> = db_video_ids
                .into_iter()
                .filter(|video_id| api_videos_iter.all(|v| &v.id != video_id))
                .collect();

            for video_to_delete in &videos_to_delete {
                tracing::debug!("deleting {}", video_to_delete);
            }

            let mut videos_to_delete_iter = videos_to_delete.iter();
            let videos_to_update: Vec<YoutubeVideo> = db_videos //
                .into_iter()
                .filter(|video| videos_to_delete_iter.all(|v_id| v_id != &video.id))
                .collect();

            for video_to_update in &videos_to_update {
                tracing::debug!("upserting {}", video_to_update.id);
            }

            sqlite::upsert_db_videos(videos_to_update)?;
            sqlite::delete_db_videos(&videos_to_delete)?;

            pages::refresh_page(pages::Pages::Upcoming).await?;
            pages::refresh_page(pages::Pages::LastSeen).await?;

            Ok(())
        }
    }
}
