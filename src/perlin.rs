use rand::{prelude::SliceRandom, random, thread_rng};

use crate::{Float, Point3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranfloat: [Float; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perlin = Self {
            ranfloat: (0..POINT_COUNT)
                .map(|_| random::<Float>())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            perm_x: (0..POINT_COUNT)
                .into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            perm_y: (0..POINT_COUNT)
                .into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            perm_z: (0..POINT_COUNT)
                .into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };

        let mut rng = thread_rng();
        perlin.perm_x.shuffle(&mut rng);
        perlin.perm_y.shuffle(&mut rng);
        perlin.perm_z.shuffle(&mut rng);

        perlin
    }

    pub fn noise(&self, p: &Point3) -> Float {
        let i = (((4.0 * p.x()) as isize) & 255) as usize;
        let j = (((4.0 * p.y()) as isize) & 255) as usize;
        let k = (((4.0 * p.z()) as isize) & 255) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
