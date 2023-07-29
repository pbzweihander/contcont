use anyhow::{Context, Result};
use serde::Serialize;

use crate::config::CONFIG;

#[derive(Serialize)]
struct PostCreateNoteReq {
    i: String,
    visibility: String,
    text: String,
}

pub async fn post_note(http_client: &reqwest::Client, text: String) -> Result<()> {
    if let (Some(misskey_base_url), Some(misskey_api_key)) =
        (&CONFIG.misskey_base_url, &CONFIG.misskey_api_key)
    {
        let url = misskey_base_url
            .join("./api/notes/create")
            .context("failed to join URL path")?;
        http_client
            .post(url)
            .json(&PostCreateNoteReq {
                i: misskey_api_key.to_string(),
                visibility: "home".to_string(),
                text,
            })
            .send()
            .await
            .context("failed to request to Misskey instance")?
            .error_for_status()
            .context("Misskey instance returned error")?;
        Ok(())
    } else {
        Ok(())
    }
}
