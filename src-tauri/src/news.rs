use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsInfo {
    id: u32,
    newsTitle: String,
    // newsCreatetime:String,
    newsCreatetime: DateTime<Utc>,
    newsDetails: String,
}
