use crate::systems::input::InputConsumerSystem;
use crate::video::VIDEO_HEIGHT;
use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pos: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    pub visible_z_range: f32,
}

impl Camera {
    pub fn new(pos: Point3<f32>) -> Camera {
        let front = Vector3::<f32>::new(0.0, 0.0, -1.0);
        let up = Vector3::<f32>::y();
        Camera {
            pos,
            front,
            up,
            right: front.cross(&up).normalize(),
            visible_z_range: 0.0,
        }
    }

    pub fn pos(&self) -> Point3<f32> {
        self.pos
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
    }

    pub fn set_z(&mut self, z: f32) {
        self.pos.z = z;
    }

    pub fn update_visible_z_range(&mut self, projection: &Matrix4<f32>) {
        let view = self.create_view_matrix();
        let center = InputConsumerSystem::picking_2d_3d(
            0,
            (VIDEO_HEIGHT / 2) as u16,
            &self.pos(),
            projection,
            &view,
        );
        self.visible_z_range = (self.pos.z - center.y).abs();
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32) {
        self.front = Vector3::<f32>::new(
            pitch.to_radians().cos() * yaw.to_radians().cos(),
            pitch.to_radians().sin(),
            pitch.to_radians().cos() * yaw.to_radians().sin(),
        )
        .normalize();
        self.right = self.front.cross(&Vector3::y()).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }

    pub fn move_forward(&mut self, speed: f32) {
        self.pos += speed * self.front;
    }

    pub fn move_side(&mut self, speed: f32) {
        self.pos += self.front.cross(&self.up).normalize() * speed;
    }

    pub fn move_along_z(&mut self, speed: f32) {
        self.pos.z += speed;
    }

    pub fn move_along_x(&mut self, speed: f32) {
        self.pos.x += speed;
    }

    pub fn create_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn look_at(&mut self, p: Point3<f32>) {
        self.front = (p - self.pos).normalize();
        self.right = self.front.cross(&Vector3::y()).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
