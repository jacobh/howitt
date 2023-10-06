use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub trait ExternallySourced {
    fn external_ref(&self) -> Option<&ExternalRef>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum ExternalId {
    Rwgps(RwgpsId),
}
impl ExternalId {
    pub fn canonical_url(&self) -> url::Url {
        match self {
            ExternalId::Rwgps(rwgps_id) => rwgps_id.canonical_url(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum RwgpsId {
    Route(usize),
    Trip(usize),
}

impl RwgpsId {
    pub fn canonical_url(&self) -> url::Url {
        match self {
            RwgpsId::Route(route_id) => {
                url::Url::parse(&format!("https://ridewithgps.com/routes/{route_id}")).unwrap()
            }
            RwgpsId::Trip(trip_id) => {
                url::Url::parse(&format!("https://ridewithgps.com/trips/{trip_id}")).unwrap()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRef {
    pub id: ExternalId,
    pub updated_at: DateTime<Utc>,
    pub sync_version: Option<usize>,
}

pub struct ExternalRefItemMap<T>(HashMap<ExternalId, (ExternalRef, T)>);
impl<T> ExternalRefItemMap<T> {
    pub fn new(items: impl IntoIterator<Item = (ExternalRef, T)>) -> ExternalRefItemMap<T> {
        ExternalRefItemMap(
            items
                .into_iter()
                .map(|(external_ref, item)| (external_ref.id.clone(), (external_ref, item)))
                .collect(),
        )
    }
    pub fn from_externally_reffed(items: impl IntoIterator<Item = T>) -> ExternalRefItemMap<T>
    where
        T: ExternallySourced,
    {
        ExternalRefItemMap::new(items.into_iter().filter_map(|item| {
            item.external_ref()
                .cloned()
                .map(|external_ref| (external_ref, item))
        }))
    }
    pub fn match_ref(
        &self,
        ExternalRef {
            id,
            updated_at,
            sync_version,
        }: ExternalRef,
    ) -> ExternalRefMatch<'_, T> {
        match self.0.get(&id) {
            Some((external_ref, item)) => {
                if external_ref.sync_version == sync_version
                    && external_ref.updated_at == updated_at
                {
                    ExternalRefMatch::Fresh(item)
                } else {
                    ExternalRefMatch::Stale(item)
                }
            }
            None => ExternalRefMatch::NotFound,
        }
    }
}

pub enum ExternalRefMatch<'a, T> {
    NotFound,
    Fresh(&'a T),
    Stale(&'a T),
}
