use grid::Grid;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Not};

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coord(pub i32, pub i32);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

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

    pub fn vector_to(&self, other: Self) -> Vector {
        Vector(other.0 - self.0, other.1 - self.1)
    }
}

impl<T> TryFrom<(T, T)> for Coord
where
    T: TryInto<i32>,
{
    type Error = <T as TryInto<i32>>::Error;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        Ok(Self(value.0.try_into()?, value.1.try_into()?))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vector(pub i32, pub i32);

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            v if *v == Vector::up() => write!(f, "Up"),
            v if *v == Vector::down() => write!(f, "Down"),
            v if *v == Vector::left() => write!(f, "Left"),
            v if *v == Vector::right() => write!(f, "Right"),
            Vector(i, j) => write!(f, "({i},{j})"),
        }
    }
}

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

impl Coord {
    pub fn wrapping_add(self, rhs: Vector, rows: i32, cols: i32) -> Self {
        let mut coord = self + rhs;
        if coord.0 >= rows {
            coord.0 = coord.0 - rows;
        }
        if coord.1 >= cols {
            coord.1 = coord.1 - cols;
        }
        if coord.0 < 0 {
            coord.0 = rows - -coord.0;
        }
        if coord.1 < 0 {
            coord.1 = cols - -coord.1;
        }
        coord
    }
}

impl AddAssign<Vector> for Coord {
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Vector> for Vector {
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

    pub fn from_arrow(c: char) -> Self {
        match c {
            '>' => Self::right(),
            'v' => Self::down(),
            '<' => Self::left(),
            '^' => Self::up(),
            _ => panic!(),
        }
    }
}

pub trait IndexedIterCoord<T> {
    fn indexed_iter_coord<'a>(&'a self) -> impl Iterator<Item = (Coord, &'a T)>
    where
        T: 'a;
}

impl<T> IndexedIterCoord<T> for Grid<T> {
    fn indexed_iter_coord<'a>(&'a self) -> impl Iterator<Item = (Coord, &'a T)>
    where
        T: 'a,
    {
        self.indexed_iter()
            .map(|((i, j), x)| (Coord::try_from((i, j)).unwrap(), x))
    }
}
