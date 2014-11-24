
use std::iter;
use std::num::{Float, NumCast};

pub type Linspace<F> = iter::Take<iter::Counter<F>>;

/// Return an iterator with `n` elements, where the first
/// element is `a` and the last element is `b`.
///
/// Iterator element type is `F`.
///
/// **Panics** if `n` is zero.
///
/// ```
/// use itertools as it;
/// let mut xs = it::linspace(0.0_f32, 1., 5);
/// assert_eq!(xs.collect::<Vec<_>>(),
///            vec![0., 0.25, 0.5, 0.75, 1.0]);
/// ```
#[inline]
pub fn linspace<F: Float>(a: F, b: F, n: uint) -> Linspace<F>
{
    let nf: F = NumCast::from(n).expect("linspace requires n > 0");
    let step = (b - a)/(nf - Float::one());
    iter::count(a, step).take(n)
}