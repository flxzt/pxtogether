use tokio::io::AsyncWriteExt;

pub(crate) async fn open_file() -> Option<Vec<u8>> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("~")
        .pick_file()
        .await?;

    Some(file.read().await)
}

pub(crate) async fn save_file(data: Vec<u8>) -> anyhow::Result<()> {
    if let Some(file) = rfd::AsyncFileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("~")
        .set_file_name("grid.json")
        .save_file()
        .await
    {
        let mut file = tokio::fs::File::create(file.inner()).await?;
        file.write_all(&data).await?;
    }

    Ok(())
}
