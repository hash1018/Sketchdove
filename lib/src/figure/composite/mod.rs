use std::sync::Arc;

use super::Figure;

pub trait Composite: Figure {}

pub fn upcast(obj: Arc<dyn Composite>) -> Arc<dyn Figure> {
    obj.as_dyn_figure()
}
