use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 定义事件类型
pub enum Event {
    ImageLoaded,
    ImageProcessed,
}

// 定义观察者接口
pub trait Observer {
    fn on_notify(&self, event: &Event);
}

pub struct EventManager {
    observers: HashMap<Event, Vec<Arc<Mutex<dyn Observer + Send>>>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            observers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event: Event, observer: Arc<Mutex<dyn Observer + Send>>) {
        self.observers
            .entry(event)
            .or_insert(Vec::new())
            .push(observer);
    }

    pub fn notify(&self, event: &Event) {
        if let Some(observers) = self.observers.get(event) {
            for observer in observers {
                observer.lock().unwrap().on_notify(event);
            }
        }
    }
}
