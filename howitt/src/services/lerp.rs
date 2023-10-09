use std::rc::Rc;

use itertools::Itertools;

pub trait Lerp: std::fmt::Debug + Clone {
    fn lerp(&self, other: &Self, frac: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(&self, other: &Self, frac: f64) -> Self {
        let delta = other - self;

        self + (delta * frac)
    }
}

#[derive(derive_more::Constructor, Clone, Debug)]
pub struct Lerped<T: Lerp> {
    a: T,
    b: T,
    frac: f64,
}

impl<T> Lerped<T>
where
    T: Lerp,
{
    pub fn value(&self) -> T {
        self.a.lerp(&self.b, self.frac)
    }
}

#[derive(Debug, Clone)]
pub struct LerpData<T: Lerp, U: std::fmt::Debug + Clone> {
    lerp: T,
    data: Rc<U>,
}

impl<T, U> LerpData<T, U>
where
    T: Lerp,
    U: std::fmt::Debug + Clone,
{
    pub fn new(lerp: T, data: U) -> LerpData<T, U> {
        LerpData {
            lerp,
            data: Rc::new(data),
        }
    }

    pub fn into_parts(self) -> (T, U) {
        (self.lerp, U::clone(&self.data))
    }

    pub fn value(&self) -> &T {
        &self.lerp
    }

    pub fn data(&self) -> &U {
        self.data.as_ref()
    }
}

// impl<T: Lerp, U> Clone for LerpData<T, U> {
//     fn clone(&self) -> Self {
//         Self { lerp: self.lerp.clone(), data: self.data.clone() }
//     }
// }

impl<T, U> Lerp for LerpData<T, U>
where
    T: Lerp,
    U: std::fmt::Debug + Clone,
{
    fn lerp(&self, other: &Self, frac: f64) -> LerpData<T, U> {
        LerpData {
            lerp: self.value().lerp(other.value(), frac),
            data: if frac <= 0.5 {
                self.data.clone()
            } else {
                other.data.clone()
            },
        }
    }
}

#[derive(Debug, derive_more::Constructor)]
struct LerpNode<T: Lerp> {
    lerp: T,
    size: f64,
}

#[derive(Debug, Clone, derive_more::Constructor)]
struct LerpNodeRef<'a, T: Lerp> {
    idx: usize,
    node: &'a LerpNode<T>,
    nodes: &'a LerpNodes<T>,
}

impl<'a, T> LerpNodeRef<'a, T>
where
    T: Lerp,
{
    fn as_lerp(&self) -> &T {
        &self.node.lerp
    }

    fn frac_delta(&self) -> f64 {
        self.node.size / self.nodes.size()
    }

    fn frac_start(&self) -> f64 {
        self.nodes
            .node_refs()
            .take_while(|node_ref| node_ref.idx != self.idx)
            .map(|node_ref| node_ref.frac_delta())
            .sum()
    }

    fn frac_bounds(&self) -> (f64, f64) {
        let start = self.frac_start();

        (start, start + self.frac_delta())
    }
}

#[derive(Debug, derive_more::Constructor)]
struct LerpNodeRefWindow<'a, T: Lerp> {
    node1: LerpNodeRef<'a, T>,
    node2: LerpNodeRef<'a, T>,
}

impl<'a, T> LerpNodeRefWindow<'a, T>
where
    T: Lerp,
{
    fn as_lerps(&self) -> (&T, &T) {
        let LerpNodeRefWindow { node1, node2 } = self;

        (node1.as_lerp(), node2.as_lerp())
    }

    fn bounds(&self) -> (f64, f64) {
        let (lower1, upper1) = self.node1.frac_bounds();
        let (lower2, upper2) = self.node2.frac_bounds();

        let lower = f64::min(lower1, lower2);
        let upper = f64::max(upper1, upper2);

        (lower, upper)
    }

    fn contains_frac(&self, frac: f64) -> bool {
        let (lower, upper) = self.bounds();

        frac >= lower && frac <= upper
    }

    fn internodal_frac(&self, frac: f64) -> Result<f64, ()> {
        let (lower, upper) = self.bounds();

        if self.contains_frac(frac) {
            Ok(1.0 - ((upper - frac) / (upper - lower)))
        } else {
            Err(())
        }
    }

    fn value(&self, frac: f64) -> Result<T, ()> {
        self.internodal_frac(frac).map(|internodal_frac| {
            let (lerp1, lerp2) = self.as_lerps();
            lerp1.lerp(lerp2, internodal_frac)
        })
    }
}

#[derive(Debug, derive_more::Constructor)]
struct LerpNodes<T: Lerp + Clone> {
    nodes: Vec<LerpNode<T>>,
}

impl<T: Lerp + Clone> LerpNodes<T> {
    fn size(&self) -> f64 {
        self.nodes.iter().map(|node| node.size).sum()
    }

    fn node_refs(&self) -> impl Iterator<Item = LerpNodeRef<'_, T>> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| LerpNodeRef {
                idx,
                node,
                nodes: self,
            })
    }

    fn node_windows(&self) -> impl Iterator<Item = LerpNodeRefWindow<'_, T>> {
        self.node_refs()
            .tuple_windows()
            .map(|(ref1, ref2)| LerpNodeRefWindow::new(ref1, ref2))
    }

    fn value(&self, frac: f64) -> T {
        if self.nodes.len() == 0 {
            panic!()
        }

        if self.nodes.len() == 1 {
            return self.nodes.first().unwrap().lerp.clone();
        }

        self.node_windows()
            .find_map(|window| window.value(frac).ok())
            .unwrap()
    }

    fn values(&self, count: usize) -> impl Iterator<Item = T> + '_ {
        (0..count).map(move |i| self.value((i as f64) / (count as f64)))
    }
}

pub fn lerp_data_vec<T: Lerp + Clone, U: std::fmt::Debug + Clone>(
    items: Vec<(T, f64, U)>,
    output_count: usize,
) -> Vec<(T, U)> {
    let nodes = items
        .into_iter()
        .map(|(value, node, data)| LerpNode::new(LerpData::new(value, data), node))
        .collect_vec();

    let nodes = LerpNodes::new(nodes);

    // dbg!(&nodes);

    nodes
        .values(output_count)
        .map(|x| x.into_parts())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::services::lerp::{Lerp, LerpData};

    use super::{lerp_data_vec, LerpNode, LerpNodes, Lerped};

    #[test]
    fn test_lerped() {
        let lerped = Lerped::new(0.0, 10.0, 0.5);

        assert_eq!(lerped.value(), 5.0);
    }

    #[test]
    fn test_lerpeddata() {
        let lerped_data1 = LerpData::new(0.0, "a");
        let lerped_data2 = LerpData::new(10.0, "b");

        let lerped1 = lerped_data1.lerp(&lerped_data2, 0.3);
        let lerped2 = lerped_data1.lerp(&lerped_data2, 0.7);

        assert_eq!(lerped1.value(), &3.0);
        assert_eq!(lerped1.data(), &"a");

        assert_eq!(lerped2.value(), &7.0);
        assert_eq!(lerped2.data(), &"b");
    }

    #[test]
    fn test_node_windows() {
        let nodes = LerpNodes::new(vec![
            LerpNode::new(1.0, 20.0),
            LerpNode::new(2.0, 20.0),
            LerpNode::new(3.0, 20.0),
            LerpNode::new(4.0, 20.0),
            LerpNode::new(5.0, 20.0),
        ]);

        let lerp_bounds = nodes
            .node_windows()
            .map(|window| {
                let (a, b) = window.as_lerps();
                let bounds = window.bounds();
                vec![(*a, *b), bounds]
            })
            .collect_vec();

        let expected = vec![
            vec![(1.0, 2.0), (0.0, 0.25)],
            vec![(2.0, 3.0), (0.25, 0.5)],
            vec![(3.0, 4.0), (0.5, 0.75)],
            vec![(4.0, 5.0), (0.75, 1.0)]
        ];

        assert_eq!(
            lerp_bounds,
            expected
        );
    }

    // #[test]
    fn test_lerp_data_vec() {
        let data_vec = vec![(5.0, 10.0, "a"), (20.0, 30.0, "b"), (10.0, 20.0, "c")];

        let result = lerp_data_vec(data_vec, 50);

        assert_eq!(
            result,
            vec![
                (5.0, "a"),
                (5.45, "a"),
                (5.9, "a"),
                (6.35, "a"),
                (6.8, "a"),
                (7.25, "a"),
                (7.700000000000001, "a"),
                (8.150000000000002, "a"),
                (8.600000000000001, "a"),
                (9.05, "a"),
                (9.5, "a"),
                (9.95, "a"),
                (10.399999999999999, "a"),
                (10.850000000000001, "a"),
                (11.3, "a"),
                (11.75, "a"),
                (12.2, "a"),
                (12.65, "b"),
                (13.100000000000001, "b"),
                (13.55, "b"),
                (14.000000000000002, "b"),
                (14.45, "b"),
                (14.9, "b"),
                (15.350000000000001, "b"),
                (15.799999999999999, "b"),
                (16.25, "b"),
                (16.700000000000003, "b"),
                (17.15, "b"),
                (17.6, "b"),
                (18.05, "b"),
                (18.5, "b"),
                (18.950000000000003, "b"),
                (19.4, "b"),
                // this is weird. shouldn't jump like this
                (19.85, "b"),
                (13.84, "c"),
                //
                (13.600000000000001, "c"),
                (13.360000000000001, "c"),
                (13.120000000000001, "c"),
                (12.88, "c"),
                (12.64, "c"),
                (12.4, "c"),
                (12.16, "c"),
                (11.92, "c"),
                (11.68, "c"),
                (11.44, "c"),
                (11.2, "c"),
                (10.959999999999999, "c"),
                (10.72, "c"),
                (10.48, "c"),
                (10.24, "c")
            ]
        )
    }
}
