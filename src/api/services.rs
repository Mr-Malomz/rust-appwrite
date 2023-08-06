use dotenv::dotenv;
use reqwest::{header, Client, Error};
use std::env;

use super::model::{JsonAPIBody, Project, ProjectRequest, ProjectResponse};

pub struct AppwriteService {}

impl AppwriteService {
    fn env_loader(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }
    }

    fn init() -> Client {
        Client::new()
    }

    pub async fn create_project(new_project: Project) -> Result<ProjectResponse, Error> {
        //get details from environment variable
        let project_id = AppwriteService::env_loader("PROJECT_ID");
        let database_id = AppwriteService::env_loader("DATABASE_ID");
        let collection_id = AppwriteService::env_loader("COLLECTION_ID");
        let api_key = AppwriteService::env_loader("API_KEY");

        let url = format!("https://cloud.appwrite.io/v1/databases/{database_id}/collections/{collection_id}/documents");

        //create header
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Appwrite-Key", api_key.parse().unwrap());
        headers.insert("X-Appwrite-Project", project_id.parse().unwrap());

        let client = AppwriteService::init()
            .post(url)
            .headers(headers)
            .json(&JsonAPIBody {
                documentId: "unique()".to_string(),
                data: ProjectRequest {
                    name: new_project.name,
                    description: new_project.description,
                },
            })
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let created_project: ProjectResponse = serde_json::from_str(json.as_str()).unwrap();

                Ok(created_project)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_project(document_id: String) -> Result<Project, Error> {
        //get details from environment variable
        let project_id = AppwriteService::env_loader("PROJECT_ID");
        let database_id = AppwriteService::env_loader("DATABASE_ID");
        let collection_id = AppwriteService::env_loader("COLLECTION_ID");
        let api_key = AppwriteService::env_loader("API_KEY");

        let url = format!("https://cloud.appwrite.io/v1/databases/{database_id}/collections/{collection_id}/documents/{document_id}");

        //create header
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Appwrite-Key", api_key.parse().unwrap());
        headers.insert("X-Appwrite-Project", project_id.parse().unwrap());

        let client = AppwriteService::init()
            .get(url)
            .headers(headers)
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let created_project: Project = serde_json::from_str(json.as_str()).unwrap();

                Ok(created_project)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn update_project(
        updated_project: Project,
        document_id: String,
    ) -> Result<ProjectResponse, Error> {
        //get details from environment variable
        let project_id = AppwriteService::env_loader("PROJECT_ID");
        let database_id = AppwriteService::env_loader("DATABASE_ID");
        let collection_id = AppwriteService::env_loader("COLLECTION_ID");
        let api_key = AppwriteService::env_loader("API_KEY");

        let url = format!("https://cloud.appwrite.io/v1/databases/{database_id}/collections/{collection_id}/documents/{document_id}");

        //create header
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Appwrite-Key", api_key.parse().unwrap());
        headers.insert("X-Appwrite-Project", project_id.parse().unwrap());

        let client = AppwriteService::init()
            .patch(url)
            .headers(headers)
            .json(&JsonAPIBody {
                documentId: "unique()".to_string(),
                data: ProjectRequest {
                    name: updated_project.name,
                    description: updated_project.description,
                },
            })
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let updates: ProjectResponse = serde_json::from_str(json.as_str()).unwrap();

                Ok(updates)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn delete_project(document_id: String) -> Result<String, Error> {
        //get details from environment variable
        let project_id = AppwriteService::env_loader("PROJECT_ID");
        let database_id = AppwriteService::env_loader("DATABASE_ID");
        let collection_id = AppwriteService::env_loader("COLLECTION_ID");
        let api_key = AppwriteService::env_loader("API_KEY");

        let url = format!("https://cloud.appwrite.io/v1/databases/{database_id}/collections/{collection_id}/documents/{document_id}");

        //create header
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Appwrite-Key", api_key.parse().unwrap());
        headers.insert("X-Appwrite-Project", project_id.parse().unwrap());

        let client = AppwriteService::init()
            .delete(url)
            .headers(headers)
            .send()
            .await;

        match client {
            Ok(_) => {
                let json = format!("Project with ID: ${document_id} deleted successfully!!");
                Ok(json)
            }
            Err(error) => Err(error),
        }
    }
}
