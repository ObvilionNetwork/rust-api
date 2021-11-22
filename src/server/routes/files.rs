use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::{response::Responder, Request};
use std::{fs, path::Path};

pub enum AbstractContent {
    File(NamedFile),
    Directory(Json<Vec<String>>),
}

impl AbstractContent {
    pub async fn new(provided_path: String) -> Self {
        let mut provided_path = format!("{}{}", "./resources/", provided_path.as_str());
        provided_path = provided_path.replace("..", "");
        let path = Path::new(&provided_path);

        if path.is_file() {
            Self::File(NamedFile::open(path).await.unwrap())
        } else {
            let mut dir_content: Vec<String> = Vec::new();
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry
                    .path()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    .replace("./resources/", "");
                dir_content.push(path);
            }
            Self::Directory(Json::from(dir_content))
        }
    }
}

impl<'r> Responder<'r, 'static> for AbstractContent {
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'static> {
        match self {
            AbstractContent::File(file) => file.respond_to(request),
            AbstractContent::Directory(dir) => dir.respond_to(request),
        }
    }
}

#[get("/files?<path>")]
pub async fn get(path: &str) -> AbstractContent {
    AbstractContent::new(path.into()).await
}
