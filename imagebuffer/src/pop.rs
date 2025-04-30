pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 创建一个新的空栈
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 向栈中添加元素
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 从栈中移除并返回最后一个元素
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // 获取栈的大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 创建一个新的空栈
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 向栈中添加元素
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 从栈中移除并返回最后一个元素
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // 获取栈的大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // 创建一个新的空栈
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    // 向栈中添加元素
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 从栈中移除并返回最后一个元素
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // 获取栈的大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    // 向栈中添加元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 打印栈的大小
    println!("Stack size: {}", stack.size());

    // 弹出元素
    if let Some(value) = stack.pop() {
        println!("Popped value: {}", value);
    }

    // 检查栈是否为空
    println!("Is stack empty? {}", stack.is_empty());
}
