use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Interval;
use yew::html::Scope;

use super::{DrawArea, DrawAreaMessage};

#[derive(Default)]
pub struct MouseTracker {
    prex_x: Rc<RefCell<Option<f64>>>,
    prev_y: Rc<RefCell<Option<f64>>>,
    current_x: Rc<RefCell<Option<f64>>>,
    current_y: Rc<RefCell<Option<f64>>>,
    interval: Option<Interval>,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker {
            prex_x: Rc::new(RefCell::new(None)),
            prev_y: Rc::new(RefCell::new(None)),
            current_x: Rc::new(RefCell::new(None)),
            current_y: Rc::new(RefCell::new(None)),
            interval: None,
        }
    }

    pub fn set_current_pos(&mut self, x: f64, y: f64) {
        *self.current_x.borrow_mut() = Some(x);
        *self.current_y.borrow_mut() = Some(y);
    }

    pub fn run(&mut self, link: Scope<DrawArea>) {
        let prev_x = self.prex_x.clone();
        let prev_y = self.prev_y.clone();
        let current_x = self.current_x.clone();
        let current_y = self.current_y.clone();

        let interval = Interval::new(200, move || {
            let mut prev_x = prev_x.borrow_mut();
            let mut prev_y = prev_y.borrow_mut();
            let current_x = current_x.borrow();
            let current_y = current_y.borrow();

            match (*prev_x, *prev_x, *current_x, *current_y) {
                (Some(_), Some(_), Some(x), Some(y)) => {
                    if *prev_x != *current_x && *prev_y != *current_y {
                        *prev_x = *current_x;
                        *prev_y = *current_y;
                        link.send_message(DrawAreaMessage::MousePositionChanged(x, y));
                    }
                }
                (None, None, Some(x), Some(y)) => {
                    *prev_x = *current_x;
                    *prev_y = *current_y;
                    link.send_message(DrawAreaMessage::MousePositionChanged(x, y));
                }
                _ => {}
            }
        });

        self.interval = Some(interval);
    }

    pub fn stop(&mut self) {
        self.interval.take();
    }
}
