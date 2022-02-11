use std::cell::RefCell;
use std::rc::Rc;

use druid::kurbo::{BezPath, PathEl};
use druid::{Data, Lens, Point};

#[derive(Debug, Clone, Data, Lens)]
pub struct Stroke {
    pub bez_path: Rc<RefCell<BezPath>>,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            bez_path: Rc::new(RefCell::new(BezPath::new())),
        }
    }
}

impl Stroke {
    pub fn new_w_move(pos: Point) -> Self {
        Self {
            bez_path: Rc::new(RefCell::new(BezPath::from_vec(vec![PathEl::MoveTo(pos)]))),
        }
    }
}

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct CanvasState {
    pub strokes: Rc<RefCell<Vec<Stroke>>>,
}

impl CanvasState {}
