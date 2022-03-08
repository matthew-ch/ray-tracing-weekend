use rand::{prelude::SliceRandom, thread_rng};

use crate::{Float, Point3, Vec3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perlin = Self {
            ranvec: (0..POINT_COUNT)
                .map(|_| Vec3::random_range(-1.0..1.0).unit_vector())
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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (p.x().floor()) as isize;
        let j = (p.y().floor()) as isize;
        let k = (p.z().floor()) as isize;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in [0, 1] {
            for dj in [0, 1] {
                for dk in [0, 1] {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: u32) -> Float {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in [0, 1] {
            for j in [0, 1] {
                for k in [0, 1] {
                    let weight_v = Vec3::new(u - i as Float, v - j as Float, w - k as Float);
                    accum += c[i][j][k].dot(&weight_v)
                        * (if i == 0 { 1.0 - u } else { u })
                        * (if j == 0 { 1.0 - v } else { v })
                        * (if k == 0 { 1.0 - w } else { w });
                }
            }
        }

        accum
    }
}
