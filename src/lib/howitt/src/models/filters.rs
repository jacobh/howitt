use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TemporalFilter {
    Before {
        before: DateTime<Utc>,
        last: Option<usize>,
    },
    After {
        after: DateTime<Utc>,
        first: Option<usize>,
    },
}
