use na::{Axpy};
use na;
use math::Scalar;
use std::ops::Mul;

/// Computes the center of a set of point.
// FIXME: Dot<N> is here only to determine `N`.
#[inline]
pub fn center<N, P>(pts: &[P]) -> P
    where N: Scalar,
          P: Axpy<N> + Mul<N, Output=P> + Clone {
    assert!(pts.len() >= 1, "Cannot compute the center of less than 1 point.");

    let denom: N = na::cast(1.0 / (pts.len() as f64));

    let mut piter = pts.iter();
    let mut res = piter.next().unwrap().clone().mul(denom);

    for pt in piter {
        res.axpy(&denom, pt);
    }

    res
}
