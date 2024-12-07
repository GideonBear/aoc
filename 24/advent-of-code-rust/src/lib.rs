use grid::Grid;
use std::ops::{Add, AddAssign, Not};

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    pub fn in_bound<T>(&self, grid: &Grid<T>) -> bool {
        self.0 >= 0
            && self.1 >= 0
            && self.0 < grid.rows().try_into().unwrap()
            && self.1 < grid.cols().try_into().unwrap()
    }

    pub fn get<'a, T>(&self, grid: &'a Grid<T>) -> Option<&'a T> {
        grid.get(self.0, self.1)
    }

    pub fn val(&self) -> (usize, usize) {
        (
            usize::try_from(self.0).unwrap(),
            usize::try_from(self.1).unwrap(),
        )
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub i32, pub i32);

impl Not for Vector {
    type Output = Vector;

    fn not(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}

impl Add<Vector> for Coord {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Vector> for Coord {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Vector {
    pub fn get_ortho() -> impl Iterator<Item = Vector> {
        [Self::up(), Self::right(), Self::down(), Self::left()].into_iter()
    }

    pub fn get_diagonal() -> impl Iterator<Item = Vector> {
        [Vector(1, 1), Vector(1, -1), Vector(-1, -1), Vector(-1, 1)].into_iter()
    }

    pub fn get_ortho_diagonal() -> impl IntoIterator<Item = Vector> {
        Self::get_ortho().chain(Self::get_diagonal())
    }

    pub fn up() -> Self {
        Self(-1, 0)
    }

    pub fn down() -> Self {
        Self(1, 0)
    }

    pub fn right() -> Self {
        Self(0, 1)
    }

    pub fn left() -> Self {
        Self(0, -1)
    }

    pub fn turn_90(self) -> Self {
        match self {
            Vector(-1, 0) => Self::right(),
            Vector(0, 1) => Self::down(),
            Vector(1, 0) => Self::left(),
            Vector(0, -1) => Self::up(),
            _ => panic!(),
        }
    }
}
