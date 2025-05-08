pub struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    // 创建一个新的空列表
    pub fn new() -> Self {
        List {
            elements: Vec::new(),
        }
    }

    // 在列表开头插入一个元素
    pub fn unshift(&mut self, item: T) {
        self.elements.insert(0, item);
    }

    // 获取列表的元素
    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}
