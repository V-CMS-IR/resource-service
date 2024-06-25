use std::ops::Deref;
use dotenv::var;
use minio_rsc::Minio;
use minio_rsc::provider::StaticProvider;

#[derive(Clone)]
pub struct FileStorage<T>{
    instance: T
}

impl FileStorage<Minio> {
    pub fn new(secure: bool ) -> FileStorage<Minio>{
        //TODO change this later
        let access_key = var("FILE_STORAGE_ACCESS_KEY")
            .expect("FILE_STORAGE_ACCESS_KEY must be defined");

        let secret_key = var("FILE_STORAGE_ACCESS_SECRET_KEY")
            .expect("FILE_STORAGE_ACCESS_SECRET_KEY must be defined");

        let file_storage_host = var("FILE_STORAGE_HOST")
            .expect("FILE_STORAGE_HOST must be defined");

        let provider = StaticProvider::new(
            access_key,
            secret_key,
            None,
        );

        let minio = Minio::builder()
            .endpoint(file_storage_host)
            .provider(provider)
            .secure(secure)
            .build()
            .expect("Can't reach to file storage");

        FileStorage::<Minio> {
            instance: minio
        }

    }
}


impl <T>Deref for FileStorage<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
