use float_ord::FloatOrd;
use num_traits::{Float, FromPrimitive};

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct UniFloat<T: Float>(T);

impl<T: Float + FromPrimitive> UniFloat<T> {
    pub(crate) fn new(v: T) -> Option<Self> {
        let v = FloatOrd(v.to_f64().unwrap());
        if FloatOrd(0.) <= v && v <= FloatOrd(1.) {
            let v = T::from_f64(v.0).unwrap();
            Some(UniFloat(v))
        } else {
            None
        }
    }

    pub(crate) fn val(&self) -> T {
        self.0
    }
}
