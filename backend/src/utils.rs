use anyhow::Result;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct NodeInfoVersion {
    rel: String,
    href: Url,
}

#[derive(Deserialize)]
struct NodeInfoMeta {
    links: Vec<NodeInfoVersion>,
}

#[derive(Deserialize)]
struct NodeInfoSoftware {
    name: String,
}

#[derive(Deserialize)]
struct NodeInfo {
    software: NodeInfoSoftware,
}

pub async fn detect_instance(http_client: &reqwest::Client, url: Url) -> Result<Option<String>> {
    let nodeinfometa_url = url.join("/.well-known/nodeinfo")?;

    let nodeinfometa: NodeInfoMeta = http_client
        .get(nodeinfometa_url)
        .send()
        .await?
        .json()
        .await?;
    let nodeinfolink = nodeinfometa
        .links
        .into_iter()
        .find(|link| link.rel == "http://nodeinfo.diaspora.software/ns/schema/2.0");

    if let Some(link) = nodeinfolink {
        let nodeinfo: NodeInfo = http_client.get(link.href).send().await?.json().await?;
        Ok(Some(nodeinfo.software.name))
    } else {
        Ok(None)
    }
}
