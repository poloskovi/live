extern crate neuronet;
use crate::neuronet::{NeuroMatrix, FORMFACTOR, Tdata};

extern crate rand;
use rand::Rng;

pub trait MatrixAdditions2{
    fn modify(&self, procent: f32) -> NeuroMatrix;
    fn distance(&self, other:&NeuroMatrix) -> i32;
}

impl MatrixAdditions2 for NeuroMatrix{
    /// слегка измененная матрица
    fn modify(&self, procent: f32) -> NeuroMatrix{

        let mut result = NeuroMatrix::new(self.nrow, self.ncol);
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, result.m.len());//берем случайный элемент

        let x = self.m[i];
        let dx_max = (FORMFACTOR as f32 * procent / 100.0) as Tdata;
        let dx = rng.gen_range(-dx_max, dx_max);
        let mut x_new = x+dx;
        if x_new < 0{
            x_new = 0;
        }else if x_new > FORMFACTOR{
            x_new = FORMFACTOR;
        }
        result.m[i] = x_new;
        result

    }

    fn distance(&self, other:&NeuroMatrix) -> i32{

        assert_eq!(self.nrow, other.nrow);
        assert_eq!(self.ncol, other.ncol);

        let mut result = 0;
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                let d = self.get(row,col) - other.get(row,col);
                result += d*d;
            }
        }
        result
    }

}
