pub mod camera;
pub use camera::Camera;
pub mod transform;
pub use transform::Transform;

// Base trait for every struct that act as a component for a game object
pub trait Component {
    fn init(&mut self) {

    }

    fn update(&mut self) {
        
    }

    fn destruct(&mut self) {

    }
}