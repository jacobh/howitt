use super::{ModelName, ModelUuid};

pub type NoteId = ModelUuid<{ ModelName::Note }>;
