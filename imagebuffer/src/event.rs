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
