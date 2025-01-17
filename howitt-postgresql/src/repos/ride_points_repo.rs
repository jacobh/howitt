use howitt::{
    models::{ride::RidePoints, Model},
    repos::Repo,
};

use crate::{PostgresClient, PostgresRepoError};

#[derive(Debug, derive_more::Constructor)]
pub struct PostgresRidePointsRepo {
    client: PostgresClient,
}

#[async_trait::async_trait]
impl Repo for PostgresRidePointsRepo {
    type Model = RidePoints;
    type Error = PostgresRepoError;

    async fn filter_models(&self, _: ()) -> Result<Vec<RidePoints>, PostgresRepoError> {
        unimplemented!()
    }

    async fn all_indexes(
        &self,
    ) -> Result<Vec<<RidePoints as Model>::IndexItem>, PostgresRepoError> {
        unimplemented!()
    }
    async fn get(&self, _id: <RidePoints as Model>::Id) -> Result<RidePoints, PostgresRepoError> {
        unimplemented!()
    }
    async fn get_index(
        &self,
        _id: <RidePoints as Model>::Id,
    ) -> Result<<RidePoints as Model>::IndexItem, PostgresRepoError> {
        unimplemented!()
    }
    async fn put(&self, _ride_points: RidePoints) -> Result<(), PostgresRepoError> {
        unimplemented!()
    }
}
