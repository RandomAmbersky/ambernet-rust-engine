use cgmath::{Matrix4, Rad, Vector3};

pub struct TransformSet {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl TransformSet {
    fn matrix_calculated(&self) -> Matrix4<f32> {
        // let mat: Matrix4<f32> = Matrix4::one();
        let rotate_mat_x = Matrix4::from_angle_x(Rad(self.rot.x));
        let rotate_mat_y = Matrix4::from_angle_y(Rad(self.rot.y));
        let rotate_mat_z = Matrix4::from_angle_z(Rad(self.rot.z));
        let trans_mat = Matrix4::from_translation(self.pos);
        let scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x * scale_mat
    }
}
