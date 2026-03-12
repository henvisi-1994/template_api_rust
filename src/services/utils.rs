use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

/// Acepta solo URLs reales o paths ya normalizados (ej: profiles/xxx.png)
fn is_existing_reference(value: &str) -> bool {
    let v = value.trim();
    v.starts_with("http://")
        || v.starts_with("https://")
        || v.starts_with("profiles/")
        || v.starts_with("portfolio/")
        || v.starts_with("requests/")
}

fn ext_from_mime(mime: &str) -> &'static str {
    match mime {
        "image/jpeg" | "image/jpg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "png",
    }
}

fn mime_from_ext(ext: &str) -> &'static str {
    match ext {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/png",
    }
}

fn decode_base64_image(input: &str) -> Result<(Vec<u8>, String)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("base64 vacío"));
    }

    if let Some(rest) = trimmed.strip_prefix("data:") {
        let mut parts = rest.splitn(2, ',');
        let meta = parts.next().unwrap_or("");
        let data = parts.next().ok_or_else(|| anyhow!("data url inválido"))?;

        if !meta.contains("base64") {
            return Err(anyhow!("data url no es base64"));
        }

        let mime = meta.split(';').next().unwrap_or("");
        let ext = ext_from_mime(mime).to_string();

        let bytes = general_purpose::STANDARD
            .decode(data)
            .map_err(|e| anyhow!("base64 inválido: {}", e))?;

        return Ok((bytes, ext));
    }

    let bytes = general_purpose::STANDARD
        .decode(trimmed)
        .map_err(|e| anyhow!("base64 inválido: {}", e))?;

    Ok((bytes, "png".to_string()))
}

fn normalize_to_data_url(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("base64 vacío"));
    }

    if trimmed.starts_with("data:") {
        // Valida formato/decodificación y conserva el valor original
        let _ = decode_base64_image(trimmed)?;
        return Ok(trimmed.to_string());
    }

    let (bytes, ext) = decode_base64_image(input)?;
    let mime = mime_from_ext(&ext);
    let encoded = general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:{};base64,{}", mime, encoded))
}

#[derive(Deserialize)]
struct CloudinaryUploadResponse {
    secure_url: String,
}

#[derive(Deserialize)]
struct CloudinaryErrorResponse {
    error: CloudinaryErrorData,
}

#[derive(Deserialize)]
struct CloudinaryErrorData {
    message: String,
}

async fn upload_to_cloudinary(data_url: &str, folder: &str) -> Result<String> {
    let cloud_name = std::env::var("CLOUDINARY_CLOUD_NAME")
        .map_err(|_| anyhow!("CLOUDINARY_CLOUD_NAME no está configurada"))?;
    let upload_preset = std::env::var("CLOUDINARY_UPLOAD_PRESET")
        .map_err(|_| anyhow!("CLOUDINARY_UPLOAD_PRESET no está configurada"))?;
    let base_folder =
        std::env::var("CLOUDINARY_BASE_FOLDER").unwrap_or_else(|_| "honey-work".to_string());
    let folder = folder.trim_matches('/');
    let base_folder = base_folder.trim_matches('/');
    let target_folder = if base_folder.is_empty() {
        folder.to_string()
    } else if folder.is_empty() {
        base_folder.to_string()
    } else {
        format!("{}/{}", base_folder, folder)
    };

    let endpoint = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        cloud_name
    );

    let form = reqwest::multipart::Form::new()
        .text("file", data_url.to_string())
        .text("upload_preset", upload_preset)
        .text("folder", target_folder);

    let client = reqwest::Client::new();
    let response = client.post(endpoint).multipart(form).send().await?;
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        let cloudinary_message = serde_json::from_str::<CloudinaryErrorResponse>(&body)
            .map(|parsed| parsed.error.message)
            .unwrap_or(body);

        return Err(anyhow!(
            "error subiendo imagen a Cloudinary ({}): {}",
            status,
            cloudinary_message
        ));
    }

    let parsed: CloudinaryUploadResponse = serde_json::from_str(&body)
        .map_err(|e| anyhow!("respuesta inválida de Cloudinary: {}", e))?;
    Ok(parsed.secure_url)
}

/// Sube imagen a Cloudinary y devuelve URL pública (https://...).
pub async fn store_base64_image(input: &str, folder: &str) -> Result<String> {
    if is_existing_reference(input) {
        return Ok(input.trim().to_string());
    }

    let data_url = normalize_to_data_url(input)?;
    upload_to_cloudinary(&data_url, folder).await
}
