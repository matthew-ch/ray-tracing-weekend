use crate::{Float, Vec3};

#[derive(Clone, Copy)]
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub const fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub const fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub const fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: Float, b: Float, c: Float) -> Vec3 {
        a * self.axis[0] + b * self.axis[1] + c * self.axis[2]
    }

    pub fn local_v(&self, v: &Vec3) -> Vec3 {
        self.local(v.x(), v.y(), v.z())
    }
}

impl From<&Vec3> for Onb {
    fn from(n: &Vec3) -> Self {
        let ax2 = n.unit_vector();
        let a = if ax2.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let ax1 = ax2.cross(&a).unit_vector();
        let ax0 = ax2.cross(&ax1);
        Self {
            axis: [ax0, ax1, ax2],
        }
    }
}
