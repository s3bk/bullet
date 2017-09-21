use std::iter::IntoIterator;
use std::time::Duration;

pub fn try_fold<T, I, F, E>(iter: I, mut init: T, func: F) -> Result<T, E>
    where I: IntoIterator<Item=Result<T, E>>, F: Fn(T, T) -> Result<T, E>
{
    for i in iter.into_iter() {
        init = func(init, i?)?;
    }
    Ok(init)
}

pub fn duration_as_seconds(dt: Duration) -> f64 {
    dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9
}
