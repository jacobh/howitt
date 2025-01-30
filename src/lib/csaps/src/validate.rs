use itertools::Itertools;
use ndarray::ArrayView1;

use crate::{CsapsError::InvalidInputData, Real, Result};

pub(crate) fn validate_data_sites<T>(x: ArrayView1<T>) -> Result<()>
where
    T: Real,
{
    for (e1, e2) in x.iter().tuple_windows() {
        if e2 < e1 || e2.almost_equals(*e1) {
            return Err(InvalidInputData(
                "Data site values must satisfy the condition: x1 < x2 < ... < xN".to_string(),
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_smooth_value<T>(smooth: T) -> Result<()>
where
    T: Real,
{
    if smooth < T::zero() || smooth > T::one() {
        return Err(InvalidInputData(format!(
            "`smooth` value must be in range 0..1, given {:?}",
            smooth
        )));
    }

    Ok(())
}
