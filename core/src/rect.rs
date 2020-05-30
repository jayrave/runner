#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    pub fn offset(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(rect1: &Rect, rect2: &Rect) -> bool {
        rect1.left() < rect2.right()
            && rect2.left() < rect1.right()
            && rect1.top() < rect2.bottom()
            && rect2.top() < rect1.bottom()
    }
}
