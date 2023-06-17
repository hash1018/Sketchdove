use std::sync::Arc;

use super::Figure;

pub mod line;

pub trait Leaf: Figure {}

pub fn upcast(obj: Arc<dyn Leaf>) -> Arc<dyn Figure> {
    obj.as_dyn_figure()
}
