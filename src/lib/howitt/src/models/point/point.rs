pub trait Point: std::fmt::Debug + Clone {
    fn as_geo_point(&self) -> &geo::Point;

    fn to_geo_point(&self) -> geo::Point {
        *self.as_geo_point()
    }

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

pub trait WithElevation: Point {
    fn elevation(&self) -> f64;
}

pub trait WithDatetime: Point {
    fn datetime(&self) -> &chrono::DateTime<chrono::Utc>;
}

impl Point for geo::Point {
    fn as_geo_point(&self) -> &geo::Point {
        self
    }
}

impl Point for &geo::Point {
    fn as_geo_point(&self) -> &geo::Point {
        self
    }
}
