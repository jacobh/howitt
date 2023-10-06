use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybePair<T> {
    Singleton(T),
    Pair(T, T),
}

impl<T> MaybePair<T> {
    pub fn map<U, F>(self, f: F) -> MaybePair<U>
    where
        F: Fn(T) -> U,
    {
        match self {
            MaybePair::Singleton(a) => MaybePair::Singleton(f(a)),
            MaybePair::Pair(a, b) => MaybePair::Pair(f(a), f(b)),
        }
    }
}

impl<T> MaybePair<T>
where
    T: Clone,
{
    pub fn as_tuple(&self) -> (&T, Option<&T>) {
        match self {
            MaybePair::Singleton(a) => (a, None),
            MaybePair::Pair(a, b) => (a, Some(b)),
        }
    }

    pub fn into_tuple(self) -> (T, Option<T>) {
        match self {
            MaybePair::Singleton(a) => (a, None),
            MaybePair::Pair(a, b) => (a, Some(b)),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        match self.into_tuple() {
            (a, Some(b)) => vec![a, b],
            (a, None) => vec![a],
        }
    }
}

impl<T> Default for MaybePair<T>
where
    T: Default,
{
    fn default() -> Self {
        MaybePair::Singleton(T::default())
    }
}

impl<T> From<T> for MaybePair<T> {
    fn from(value: T) -> Self {
        MaybePair::Singleton(value)
    }
}

impl<T> From<(T, T)> for MaybePair<T> {
    fn from((a, b): (T, T)) -> Self {
        MaybePair::Pair(a, b)
    }
}
