use crate::vector_math::*;
use crate::components::*;

#[derive(Debug)]
pub struct Camera {
    fov: f32, // Vertical field of view, in degrees
    aspect: f32, // width / height
    near: f32,
    far: f32,

    has_changed: bool,

    projection: Matrix4x4,
    inv_projection: Matrix4x4,
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Camera {
        
        Camera {
            fov: fov,
            aspect: aspect,
            near: near,
            far: far,

            has_changed: true,

            projection: Matrix4x4::identity(),
            inv_projection: Matrix4x4::identity(),
        }
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.has_changed = true;
    }

    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.has_changed = true;
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.has_changed = true;
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.has_changed = true;
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn get_projection_matrix(&mut self) -> Matrix4x4 {
        self.matrix_check();
        
        self.projection
    }

    pub fn get_inv_projection_matrix(&mut self) -> Matrix4x4 {
        self.matrix_check();
        
        self.inv_projection
    }

    // Local(camera)-space to ndc-space
    pub fn transform_point(&mut self, point: Vector3) -> Vector3 {
        self.matrix_check();
        
        let mut t_point = self.projection * Vector4::new(point.x, point.y, point.z, 1.0);
        t_point /= t_point.w;
        Vector3::new(t_point.x, t_point.y, t_point.z)
    }

    pub fn inv_transform_point(&mut self, point: Vector3) -> Vector3 {
        self.matrix_check();
        
        let mut t_point = self.inv_projection * Vector4::new(point.x, point.y, point.z, 1.0);
        t_point /= t_point.w;
        Vector3::new(t_point.x, t_point.y, t_point.z)
    }

    fn matrix_check(&mut self) {
        if self.has_changed {
            self.compute_matrices();
            self.has_changed = false;
        }
    }

    fn compute_matrices(&mut self) {
        let height = 2.0 * self.near * (self.fov * 0.5).to_radians().tan();
        let width = height * self.aspect;
        
        self.projection = Matrix4x4::perspective_projection_sym(width, height, self.near, self.far);
        self.inv_projection = self.projection.inverse();

        self.has_changed = false;
    }
}