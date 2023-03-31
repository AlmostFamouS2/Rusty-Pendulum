use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
    p3: Pendulum
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        self.p.update();
        self.p.draw(graphics, false);

        self.p2.update();
        self.p2.draw(graphics, false);

        self.p3.update();
        self.p3.draw(graphics, true);

        helper.request_redraw();
    }
}

pub struct Pendulum {
    origin: vector::Vector,
    position: vector::Vector,
    angle: f32,
    angular_vel: f32,
    angular_acc: f32,

    len: f32,
    gravity: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, l: f32, g: f32) -> Pendulum {
        Pendulum {
            origin: vector::Vector::new(x,y),
            position: vector::Vector::new(0.0, 0.0),

            angle: 1.0,
            angular_vel: 0.0,
            angular_acc: 0.0,
        
            len: l,
            gravity: g,
        }
    }

    fn update(&mut self){
        self.angular_acc = -1.0 * self.gravity * self.angle.sin() / self.len;

        self.angular_vel += self.angular_acc;

        self.angle += self.angular_vel;

        self.position.set(self.len * self.angle.sin(), self.len * self.angle.cos());

        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D, isthird: bool){
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            if !isthird { Color::RED } else { Color::GREEN },
        );
        
        graphics.draw_circle( ( self.position.x, self.position.y), 30.0, 
            if !isthird { Color::RED } else {Color::GREEN});
    }
}

pub mod vector {
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector {
            Vector {x,y} // Returning it
        }

        pub fn add(&mut self, other: &Vector) -> &Vector{
            self.x += other.x;
            self.y += other.y;
            self // Returning self in the end
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector{
            self.x = x;
            self.y = y;
            self
        }
    }
}

fn main(){

    let window: Window = Window::new_centered("Title", (940, 680)).unwrap();

    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new( 470.0, 40.0, 400.0, 0.3),
        p2: Pendulum::new(470.0, 40.0, 250.0, 0.45),
        p3: Pendulum::new(470.0, 40.0, 300.0, 0.675)
    };

    window.run_loop(win);
}
