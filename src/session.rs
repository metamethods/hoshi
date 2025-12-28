use std::{error::Error, path::PathBuf};

use reqwest::Client as ReqwestClient;
use tempfile::TempDir;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
};
use twilight_model::channel::Attachment;

use crate::download_attachment;

type FileSessionError<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct FileSession(pub TempDir);

impl FileSession {
    pub fn new() -> FileSessionError<Self> {
        let dir = tempfile::tempdir()?;
        Ok(Self(dir))
    }

    pub fn path(&self) -> PathBuf {
        self.0.path().into()
    }

    pub async fn add_file<Filename: AsRef<str>>(
        &self,
        filename: Filename,
        bytes: &[u8],
    ) -> FileSessionError<()> {
        File::create(self.path().join(filename.as_ref()))
            .await?
            .write_all(bytes)
            .await?;
        Ok(())
    }

    pub async fn add_file_from_attachment(
        &self,
        attachment: &Attachment,
        reqwest_client: &ReqwestClient,
    ) -> FileSessionError<()> {
        self.add_file(
            &attachment.filename,
            download_attachment(attachment, reqwest_client)
                .await?
                .as_ref(),
        )
        .await?;
        Ok(())
    }

    pub async fn get_file<Filename: AsRef<str>>(
        &self,
        filename: Filename,
    ) -> FileSessionError<File> {
        Ok(File::open(self.path().join(filename.as_ref())).await?)
    }

    pub async fn read_file<Filename: AsRef<str>>(
        &self,
        filename: Filename,
    ) -> FileSessionError<Vec<u8>> {
        let mut reader = BufReader::new(self.get_file(filename).await?);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }

    pub async fn get_file_as_string<Filename: AsRef<str>>(
        &self,
        filename: Filename,
    ) -> FileSessionError<String> {
        Ok(String::from_utf8(self.read_file(filename).await?)?)
    }
}

impl Into<PathBuf> for FileSession {
    fn into(self) -> PathBuf {
        self.path()
    }
}
