use std::collections::HashMap;

use anyhow::{Ok, Result};
use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use tokio::fs;
use url::Url;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profile: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profile.get(name)
    }
}

pub struct DiffArgs {}

impl DiffProfile {
    pub async fn diff(&self, _args: DiffArgs) -> Result<String> {
        // let req1 = self.req1.send(&args).await?;
        // let req2 = self.req2.send(&args).await?;

        // let text1 = res1.filter_text(&self.res).await?;
        // let text2 = res2.filter_text(&self.res).await?;

        // text_diff(&text1,&test2)
        todo!()
    }
}
