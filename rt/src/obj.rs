pub(crate) trait Object {}

pub(crate) type ObjectBox = Box<dyn Object>;

pub(crate) struct Plane;

impl Object for Plane {}

pub(crate) struct Sphere;

impl Object for Sphere {}
