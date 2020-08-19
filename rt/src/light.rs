pub(crate) trait Light {}

pub(crate) type LightBox = Box<dyn Light>;

pub(crate) struct Ambient;

impl Light for Ambient {}
