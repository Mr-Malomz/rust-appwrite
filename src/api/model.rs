use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    #[serde(rename = "$id")]
    pub id: Option<String>,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectRequest {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectResponse {
    #[serde(rename = "$id")]
    pub id: String,
    #[serde(rename = "$collectionId")]
    pub collection_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JsonAPIBody {
    pub documentId: String,
    pub data: ProjectRequest,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIErrorResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<String>,
}
