use super::point_delta::DeltaData;

pub trait Point: std::fmt::Debug + Clone {
    type DeltaData: DeltaData;

    fn as_geo_point(&self) -> &geo::Point;
    fn delta(&self, other: &Self) -> Self::DeltaData;

    fn x_y(&self) -> (f64, f64) {
        geo::Point::x_y(*self.as_geo_point())
    }

    fn ordered_x_y(
        &self,
    ) -> (
        ordered_float::OrderedFloat<f64>,
        ordered_float::OrderedFloat<f64>,
    ) {
        let (x, y) = self.x_y();

        (
            ordered_float::OrderedFloat(x),
            ordered_float::OrderedFloat(y),
        )
    }

    fn to_x_y_vec(&self) -> Vec<f64> {
        let (x, y) = self.x_y();
        vec![x, y]
    }

    fn into_x_y_vec(self) -> Vec<f64> {
        let (x, y) = self.x_y();
        vec![x, y]
    }
}

impl Point for geo::Point {
    type DeltaData = ();

    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn delta(&self, _: &Self) -> Self::DeltaData {}
}

impl Point for &geo::Point {
    type DeltaData = ();

    fn as_geo_point(&self) -> &geo::Point {
        self
    }

    fn delta(&self, _: &Self) -> Self::DeltaData {}
}
