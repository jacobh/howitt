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
    nodes: &'a LerpNodes<T>,
    node1: LerpNodeRef<'a, T>,
    node2: LerpNodeRef<'a, T>,
}

impl<'a, T> LerpNodeRefWindow<'a, T>
where
    T: Lerp,
{
    fn as_lerps(&self) -> (&T, &T) {
        let LerpNodeRefWindow { node1, node2, .. } = self;

        (node1.as_lerp(), node2.as_lerp())
    }

    fn bounds(&self) -> (f64, f64) {
        let (lower, upper) = self.node1.frac_bounds();
        let scaling_factor = self.nodes.bounds_scaling_factor();


        (lower * scaling_factor, upper * scaling_factor)
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
            .map(|(ref1, ref2)| LerpNodeRefWindow::new(self, ref1, ref2))
    }

    fn bounds_scaling_factor(&self) -> f64 {
        let last_node = self.node_refs().last().unwrap();

        let (lower, _) = last_node.frac_bounds();

        1.0 / lower
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

    use crate::services::num::Round2;

    use super::{lerp_data_vec, LerpNode, LerpNodes, Lerped, Lerp, LerpData};

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
                vec![(*a, *b), bounds].round2()
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

    #[test]
    fn test_lerp_data_vec() {
        let data_vec = vec![(5.0, 10.0, "a"), (20.0, 20.0, "b"), (40.0, 20.0, "c"), (10.0, 10.0, "d")];

        let result = lerp_data_vec(data_vec, 20).into_iter().map(|(x, y)| (x.round2(), y)).collect_vec();

        insta::assert_toml_snapshot!(result);
    }
}
