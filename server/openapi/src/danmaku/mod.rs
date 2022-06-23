use crate::common::*;

use mongodb::bson;

#[derive(serde::Deserialize, Debug)]
pub struct Query {
    uid: Option<u32>,
    time_from: i64,
    time_to: i64,
    limit: Option<i64>,
    skip: Option<u64>
}

pub async fn query(q: Query, coll: mongodb::Collection<crate::common::ExtendedEvent>) -> Result<Vec<ExtendedEvent>, mongodb::error::Error> {
    use futures::stream::TryStreamExt;

    let mut filter = bson::doc! {
        "tag": "Danmaku",
        "timestamp": {
            "$exists": true,
            "$gte": q.time_from,
            "$lt": q.time_to,
        },
    };
    if let Some(user) = q.uid {
        filter.insert("data.user.uid", bson::Bson::Int64(user as i64));
    }
    let options = mongodb::options::FindOptions::builder().limit(q.limit).skip(q.skip).build();
    let mut cursor = coll.find(filter, options).await?;
    let mut collector = Vec::new();

    while let Some(log) = cursor.try_next().await? {
        collector.push(log);
    }

    Ok(collector)
}