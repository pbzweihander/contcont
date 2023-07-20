use anyhow::Result;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
struct NodeInfoVersion {
    rel: String,
    href: Url,
}

#[derive(Debug, Clone, Deserialize)]
struct NodeInfoMeta {
    links: Vec<NodeInfoVersion>,
}

#[derive(Debug, Clone, Deserialize)]
struct NodeInfoSoftware {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct NodeInfo {
    software: NodeInfoSoftware,
}

pub async fn detect_instance(url: Url) -> Result<Option<String>> {
    let nodeinfometa_url = url.join("/.well-known/nodeinfo")?;

    let nodeinfometa: NodeInfoMeta = reqwest::get(nodeinfometa_url).await?.json().await?;
    let nodeinfolink = nodeinfometa
        .links
        .into_iter()
        .find(|link| link.rel == "http://nodeinfo.diaspora.software/ns/schema/2.0");

    if let Some(link) = nodeinfolink {
        let nodeinfo: NodeInfo = reqwest::get(link.href).await?.json().await?;
        Ok(Some(nodeinfo.software.name))
    } else {
        Ok(None)
    }
}
