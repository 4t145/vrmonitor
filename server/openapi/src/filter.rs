#[derive(serde::Deserialize, Debug)]
pub struct FindOptions {
    limit: i64,
    skip: u64
}

impl Into<Option<mongodb::options::FindOptions>> for FindOptions {
    fn into(self) -> Option<mongodb::options::FindOptions> {
        Some(mongodb::options::FindOptions::builder().limit(Some(self.limit)).skip(Some(self.skip)).build())
    }
}