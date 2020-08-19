pub trait Light {}

pub type LightBox = Box<dyn Light>;

pub struct Ambient;

impl Light for Ambient {}
