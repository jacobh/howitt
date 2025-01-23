use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Cue {
    origin: String,
    destination: String,
    distance_meters: f64,
    elevation_ascent_meters: f64,
    elevation_descent_meters: f64,
}

impl From<howitt::models::cuesheet::Cue> for Cue {
    fn from(value: howitt::models::cuesheet::Cue) -> Self {
        Cue {
            origin: value.origin.to_string(),
            destination: value.destination.to_string(),
            distance_meters: value.summary.distance_m,
            elevation_ascent_meters: value.summary.elevation_gain_m,
            elevation_descent_meters: value.summary.elevation_loss_m,
        }
    }
}
