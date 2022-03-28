use std::collections::HashMap;
use std::ops;
use std::time::SystemTime;

use ::time::{format_description::well_known::Rfc3339, OffsetDateTime};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub user_id: String,
    pub nick_name: String,
    pub default_drive_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListFileRequest<'a> {
    pub drive_id: &'a str,
    pub parent_file_id: &'a str,
    pub limit: u64,
    pub all: bool,
    pub image_thumbnail_process: &'a str,
    pub image_url_process: &'a str,
    pub video_thumbnail_process: &'a str,
    pub fields: &'a str,
    pub order_by: &'a str,
    pub order_direction: &'a str,
    pub marker: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFileResponse {
    pub items: Vec<AliyunFile>,
    pub next_marker: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFileByPathRequest<'a> {
    pub drive_id: &'a str,
    pub file_path: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamInfo {
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFileResponse {
    pub name: String,
    pub file_extension: String,
    #[serde(rename = "file_id")]
    pub id: String,
    pub r#type: FileType,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(default)]
    pub size: u64,
    pub streams_info: HashMap<String, StreamInfo>,
}

impl From<GetFileResponse> for AliyunFile {
    fn from(res: GetFileResponse) -> AliyunFile {
        let size = if res.file_extension != "livp" || res.streams_info.is_empty() {
            res.size
        } else {
            let name = res.name.replace(".livp", "");
            let mut zip_size = 0;
            for (typ, info) in &res.streams_info {
                let name_len = format!("{}.{}", name, typ).len() as u64;
                // local file header size
                zip_size += 30;
                zip_size += name_len;
                // file size
                zip_size += info.size;
                // central directory entry size
                zip_size += 46;
                zip_size += name_len;
            }
            // End of central directory size
            zip_size += 22;
            zip_size
        };
        AliyunFile {
            name: res.name,
            id: res.id,
            r#type: res.r#type,
            created_at: res.created_at,
            updated_at: res.updated_at,
            size,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFileDownloadUrlRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetFileDownloadUrlResponse {
    pub url: String,
    #[serde(default)]
    pub streams_url: HashMap<String, String>,
    pub size: u64,
    pub expiration: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrashRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateFolderRequest<'a> {
    pub check_name_mode: &'a str,
    pub drive_id: &'a str,
    pub name: &'a str,
    pub parent_file_id: &'a str,
    pub r#type: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameFileRequest<'a> {
    pub check_name_mode: &'a str,
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct MoveFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub to_drive_id: &'a str,
    pub to_parent_file_id: &'a str,
    pub new_name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CopyFileRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub to_parent_file_id: &'a str,
    pub new_name: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartInfo {
    pub part_number: u64,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub upload_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateFileWithProofRequest<'a> {
    pub check_name_mode: &'a str,
    pub content_hash: &'a str,
    pub content_hash_name: &'a str,
    pub drive_id: &'a str,
    pub name: &'a str,
    pub parent_file_id: &'a str,
    pub proof_code: &'a str,
    pub proof_version: &'a str,
    pub size: u64,
    pub part_info_list: Vec<UploadPartInfo>,
    pub r#type: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFileWithProofResponse {
    #[serde(default)]
    pub part_info_list: Vec<UploadPartInfo>,
    pub file_id: String,
    pub upload_id: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompleteUploadRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub upload_id: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUploadUrlRequest<'a> {
    pub drive_id: &'a str,
    pub file_id: &'a str,
    pub upload_id: &'a str,
    pub part_info_list: Vec<UploadPartInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDriveResponse {
    pub total_size: u64,
    pub used_size: u64,
}

#[derive(Debug, Clone)]
pub struct DateTime(SystemTime);

impl DateTime {
    pub fn new(st: SystemTime) -> Self {
        Self(st)
    }
}

impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let dt = OffsetDateTime::parse(<&str>::deserialize(deserializer)?, &Rfc3339)
            .map_err(serde::de::Error::custom)?;
        Ok(Self(dt.into()))
    }
}

impl ops::Deref for DateTime {
    type Target = SystemTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Folder,
    File,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AliyunFile {
    pub name: String,
    #[serde(rename = "file_id")]
    pub id: String,
    pub r#type: FileType,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(default)]
    pub size: u64,
}

impl AliyunFile {
    pub fn new_root() -> Self {
        let now = SystemTime::now();
        Self {
            name: "/".to_string(),
            id: "root".to_string(),
            r#type: FileType::Folder,
            created_at: DateTime(now),
            updated_at: DateTime(now),
            size: 0,
        }
    }
}
