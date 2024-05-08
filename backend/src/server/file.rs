#[cfg(feature = "embed")]
#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

pub(crate) async fn get(path: impl Into<String>) -> Option<Vec<u8>> {
    #[cfg(feature = "embed")]
    return Assets::get(&path.into()).map(|x| x.data.to_vec());

    #[cfg(not(feature = "embed"))]
    return tokio::fs::read(format!("assets/{}", path.into()))
        .await
        .ok();
}
