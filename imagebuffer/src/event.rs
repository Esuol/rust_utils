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

pub struct ImageLoader;

impl Observer for ImageLoader {
    fn on_notify(&self, event: &Event) {
        match event {
            Event::ImageLoaded => println!("ImageLoader: Image has been loaded."),
            Event::ImageProcessed => println!("ImageLoader: Image has been processed."),
        }
    }
}

pub struct ImageProcessor;

impl Observer for ImageProcessor {
    fn on_notify(&self, event: &Event) {
        match event {
            Event::ImageLoaded => println!("ImageProcessor: Image has been loaded."),
            Event::ImageProcessed => println!("ImageProcessor: Image has been processed."),
        }
    }
}

use imagebuffer::event::{Event, EventManager, ImageLoader, ImageProcessor};
use std::sync::{Arc, Mutex};

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

use imagebuffer::event::{Event, EventManager, ImageLoader, ImageProcessor};
use std::sync::{Arc, Mutex};

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}

fn main() {
    let mut event_manager = EventManager::new();

    let image_loader = Arc::new(Mutex::new(ImageLoader));
    let image_processor = Arc::new(Mutex::new(ImageProcessor));

    event_manager.subscribe(Event::ImageLoaded, image_loader.clone());
    event_manager.subscribe(Event::ImageProcessed, image_processor.clone());

    event_manager.notify(&Event::ImageLoaded);
    event_manager.notify(&Event::ImageProcessed);
}
