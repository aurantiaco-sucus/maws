use std::any::Any;

pub trait Element {
    fn as_any(&self) -> &dyn Any;
}