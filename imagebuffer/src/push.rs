pub struct Stack<T> {
    elements: Vec<T>, // 使用 Vec 存储栈中的元素
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
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // 打印栈的大小
    println!("Stack size: {}", stack.size());

    // 检查栈是否为空
    println!("Is stack empty? {}", stack.is_empty());
}
