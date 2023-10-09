pub use howitt_derive::Round2;
use itertools::Itertools;

pub fn round2(x: f64) -> f64 {
    f64::round(x * 100.0) / 100.0
}

pub trait Round2 {
    fn round2(self) -> Self;
}

impl Round2 for f64 {
    fn round2(self) -> Self {
        f64::round(self * 100.0) / 100.0
    }
}

impl<T> Round2 for Option<T>
where
    T: Round2,
{
    fn round2(self) -> Self {
        self.map(Round2::round2)
    }
}

impl<T> Round2 for (T, T)
where
    T: Round2,
{
    fn round2(self) -> Self {
        let (a, b) = self;
        (a.round2(), b.round2())
    }
}

impl Round2 for () {
    fn round2(self) -> Self {
        ()
    }
}

impl<T> Round2 for Vec<T>
where
    T: Round2,
{
    fn round2(self) -> Self {
        self.into_iter().map(Round2::round2).collect_vec()
    }
}
