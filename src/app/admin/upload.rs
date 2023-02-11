use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::constant::FILE_UPLOAD_DIC;
use actix_multipart::{Field, Multipart};
use actix_web::{post, web, HttpRequest};
use futures_util::TryStreamExt;
use std::io::Write;
use std::{fs, path};
use uuid::Uuid;

// 图片上传
#[post("/file")]
pub async fn file(mut payload: Multipart, req: HttpRequest) -> result::Response {
    while let Some(mut field) = payload.try_next().await? {
        if field.name().ne("file") {
            continue;
        }

        let filename = save_file(&mut field).await?;

        return Response::success(origin_file_url(&req, filename));
    }

    Err("上传失败".into())
}

// 图片上传
#[post("/files")]
pub async fn files(mut payload: Multipart, req: HttpRequest) -> result::Response {
    let mut filenames = vec![];

    while let Some(mut field) = payload.try_next().await? {
        let filename = save_file(&mut field).await?;

        filenames.push(origin_file_url(&req, filename))
    }

    Response::success(filenames)
}

fn origin_file_url(req: &HttpRequest, filename: String) -> String {
    format!(
        "{}://{}/upload/{}",
        req.connection_info().scheme(),
        req.connection_info().host(),
        filename
    )
}

async fn save_file(field: &mut Field) -> result::Result<String> {
    if !path::Path::new(FILE_UPLOAD_DIC).exists() {
        fs::create_dir_all(FILE_UPLOAD_DIC)?;
    }

    // todo：需要优化一下  可能会被覆盖
    let filename = field
        .content_disposition()
        .get_filename()
        .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

    let filepath = format!("{}{}", FILE_UPLOAD_DIC, filename);

    let mut f = web::block(|| fs::File::create(filepath)).await??;

    while let Some(chunk) = field.try_next().await? {
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
    }

    Ok(filename)
}
