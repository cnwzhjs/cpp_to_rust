#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f { x: x, y: y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Layout {
    pub pos: Vector2f,
    pub size: Vector2f,
}

impl Layout {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Layout {
        Layout {
            pos: Vector2f::new(x, y),
            size: Vector2f::new(width, height),
        }
    }
}

pub trait Layoutable {
    fn position(&self) -> Vector2f;
    fn size(&self) -> Vector2f;
    fn set_position(&mut self, x: f32, y: f32);
    fn set_size(&mut self, width: f32, height: f32);
}

pub trait Dockable: Layoutable {
    fn dock_left(&mut self, parent: &dyn Layoutable) {
        let width = self.size().x;
        let height = parent.size().y;
        self.set_position(0f32, 0f32);
        self.set_size(width, height);
    }
}
