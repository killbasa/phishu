use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{pages, sqlite, youtube};

pub async fn init_scheduler() -> Result<()> {
    let scheduler = JobScheduler::new().await?;

    // run every 5 minutes
    scheduler
        .add(Job::new_async("0 0/5 * * * *", |_, _| {
            Box::pin(async {
                check_videos().await.expect("failed to check videos");
            })
        })?)
        .await?;

    // run every 6 hours
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

async fn check_videos() -> Result<()> {
    tracing::info!("checking for new videos");

    match youtube::videos::get_videos_api().await {
        Err(e) => {
            tracing::error!("failed to fetch videos: {}", e);
            Ok(())
        }
        Ok(videos) => {
            if videos.is_empty() {
                tracing::info!("no videos found");
                return Ok(());
            }

            tracing::info!("found {} videos", videos.len());
            for video in &videos {
                tracing::debug!("{} (https://youtube.com/watch?v={})", video.title, video.id);
            }

            sqlite::upsert_db_videos(videos.clone())?;
            sqlite::update_db_videos(videos)?;

            pages::refresh_page(pages::Pages::Upcoming).await?;
            pages::refresh_page(pages::Pages::LastSeen).await?;

            Ok(())
        }
    }
}
