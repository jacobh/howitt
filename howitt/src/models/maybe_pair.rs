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
    pub fn into_tuple(self) -> (T, T) {
        match self {
            MaybePair::Singleton(a) => (a.clone(), a),
            MaybePair::Pair(a, b) => (a, b),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        let (a, b) = self.into_tuple();
        vec![a, b]
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
