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
