// 实现view
// 主要是实现view的相关方法

pub struct View {
    width: u32,
    height: u32,
    data: Vec<u8>, // 假设视图的数据是一个字节数组
}

impl View {
    // 创建一个新的 View
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![0; (width * height) as usize];
        View {
            width,
            height,
            data,
        }
    }

    // 获取视图的宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    // 获取视图的高度
    pub fn height(&self) -> u32 {
        self.height
    }

    // 获取视图的数据
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // 更新视图中的某个像素
    pub fn update_pixel(&mut self, x: u32, y: u32, value: u8) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.data[index] = value;
        }
    }

    // 清空视图，将所有像素设置为指定值
    pub fn clear(&mut self, value: u8) {
        self.data.fill(value);
    }
}
