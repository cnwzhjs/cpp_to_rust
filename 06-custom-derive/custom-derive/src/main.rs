use ui::*;
use ui_macro::*;

#[derive(Copy, Clone, Debug, Layoutable, Dockable)]
struct Widget {
    layout: Layout
}

impl Widget {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Widget {
        Widget {
            layout: Layout::new(x, y, width, height)
        }
    }
}

#[derive(Copy, Clone, Debug, Layoutable)]
struct MarginWidget {
    layout: Layout,
    margin: f32,
}

impl MarginWidget {
    pub fn new(x: f32, y: f32, width: f32, height: f32, margin: f32) -> MarginWidget {
        MarginWidget {
            layout: Layout::new(x, y, width, height),
            margin: margin,
        }
    }
}

impl Dockable for MarginWidget {
    fn dock_left(&mut self, parent: &dyn Layoutable) {
        let width = self.size().x;
        let height = parent.size().y;
        self.set_position(self.margin, self.margin);
        self.set_size(width, height - 2f32 * self.margin);
    }
}

fn main() {
    let screen = Widget::new(0.0, 0.0, 1920.0, 1080.0);
    let mut window = MarginWidget::new(100.0, 200.0, 50.0, 90.0, 8.0);

    println!("Screen: {:?}", screen);
    println!("Window: {:?}", window);
    window.dock_left(&screen);
    println!("Docked Window: {:?}", window);
}
