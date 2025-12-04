use std::marker::PhantomData;
use std::fmt::{Debug, Display};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::InputParser;


pub struct GridUnit;
pub type Point2D = euclid::Point2D<isize, GridUnit>;
pub type Vector2D = euclid::Vector2D<isize, GridUnit>;
pub type Size2D = euclid::Size2D<isize, GridUnit>;
pub type Rect = euclid::Rect<isize, GridUnit>;


pub trait Direction: IntoEnumIterator + Debug {
    fn delta(&self) -> Vector2D;
}

#[derive(Debug, EnumIter)]
pub enum CardinalDirection {
    N,
    E,
    S,
    W,
}

impl Direction for CardinalDirection {
    fn delta(&self) -> Vector2D {
        match self {
            Self::N    => Vector2D::new(0, -1),
            Self::E    => Vector2D::new(1 ,0),
            Self::S    => Vector2D::new(0 ,1),
            Self::W    => Vector2D::new(-1,0),
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum OrdinalDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Direction for OrdinalDirection {
    fn delta(&self) -> Vector2D {
        match self {
            Self::N    => Vector2D::new(0, -1),
            Self::NE   => Vector2D::new(1 ,-1),
            Self::E    => Vector2D::new(1 ,0),
            Self::SE   => Vector2D::new(1 ,1),
            Self::S    => Vector2D::new(0 ,1),
            Self::SW   => Vector2D::new(-1,1),
            Self::W    => Vector2D::new(-1,0),
            Self::NW   => Vector2D::new(-1,-1),
        }
    }
}


#[derive(Clone)]
pub struct Grid2D<T> {
    // Invariant: elements.len() == width*height
    pub elements: Vec<T>,
    pub width: isize,
    pub height: isize
}

impl<T> Grid2D<T> {
    pub fn rect(&self) -> Rect {
        Rect::new(Point2D::zero(), self.size())
    }

    pub fn size(&self) -> Size2D {
        Size2D::new(self.width, self.height)
    }

    fn index(&self, point: Point2D) -> Option<isize> {
        if self.rect().contains(point) {
            Some(point.x + self.width * point.y)
        } else {
            None
        }
    }

    pub fn get(&self, point: Point2D) -> Option<&T> {
        self.index(point)
            .and_then(|index| self.elements.get(index as usize))
    }

    pub fn get_mut(&mut self, point: Point2D) -> Option<&mut T> {
        self.index(point)
            .and_then(|index| self.elements.get_mut(index as usize))
    }

    pub fn point_iter(&self) -> impl Iterator<Item = Point2D> {
        (0..self.height)
        .flat_map(|y| (0..self.width).map(move |x| Point2D::new(x, y)))
    }

    pub fn element_iter(&self) -> impl Iterator<Item = (&T, Point2D)> {
        self.point_iter()
            .map(|point| (self.get(point).unwrap(), point))
    }

    pub fn point_neighbours<D: Direction>(
        &self, 
        point: Point2D, 
    ) -> impl Iterator<Item = Point2D> {
        D::iter()
            .map(move |dir| point + dir.delta())
            .filter(|neighbour| self.rect().contains(*neighbour))
    }

    pub fn element_neighbours<D: Direction>(
        &self, 
        point: Point2D, 
    ) -> impl Iterator<Item = (&T, Point2D)> {
        self.point_neighbours::<D>(point)
            .map(|neighbour| (self.get(neighbour).unwrap(), neighbour))
    }

}

impl<T: Display> Display for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let element = self.get(Point2D::new(x, y)).unwrap();
                write!(f, "{element}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: Display> Grid2D<T> {
    pub fn highlight(
        &self,
        points: Vec<(Point2D, char)>,
    ) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point2D::new(x, y);
                let element = self.get(point).unwrap();

                let highlight = points.iter()
                    .filter(|(p, _)| point == *p)
                    .next();

                if let Some((_, highlight_chr)) = highlight {
                    print!("{highlight_chr}");
                } else {
                    print!("{element}");
                }
            }

            println!();
        }
    }
}

impl<T: PartialEq> Grid2D<T> {
    pub fn element_iter_filtered(
        &self,
        search_type: &T
    ) -> impl Iterator<Item = (&T, Point2D)> {
        self.element_iter()
            .filter(|(element, _)| **element == *search_type)
    }

    pub fn element_neighbours_filtered<D: Direction>(
        &self, 
        point: Point2D, 
        search_type: &T
    ) -> impl Iterator<Item = (&T, Point2D)> {
        self.element_neighbours::<D>(point)
            .filter(|(neighbour, _)| **neighbour == *search_type)
    }
}

pub struct Grid2DParser<T> {
    pub _element_type: PhantomData<T>
}

impl<T: TryFrom<char>> InputParser for Grid2DParser<T>
    where <T as TryFrom<char>>::Error: std::fmt::Debug
{
    type Output = Grid2D<T>;

    fn parse_input(input: &str) -> Self::Output {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as isize;
        let width = lines.get(0).expect("At least one line of input").len() as isize;

        let elements: Vec<T> = lines.iter()
            .flat_map(|line| line.chars())
            .map(|chr| T::try_from(chr).expect("Input is all valid characters"))
            .collect();

        assert_eq!(width * height, elements.len() as isize);

        Grid2D::<T> {
            elements,
            width,
            height
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{CardinalDirection, Direction, Grid2DParser, InputParser, OrdinalDirection, grid::Point2D};

    const TEST_GRID: &'static str = include_str!("test_grid");

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum TestCell {
        On, Off
    }

    impl TryFrom<char> for TestCell {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
                '.' => Ok(TestCell::Off),
                '@' => Ok(TestCell::On),
                _ => Err(())
            }
        }
    }

    impl std::fmt::Display for TestCell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let chr = match self {
                TestCell::Off => '.',
                TestCell::On => '@',
            };

            write!(f,"{chr}")
        }
    }

    #[test]
    fn test_parser_grid() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.get(Point2D::new(0, 0)).cloned(), Some(TestCell::Off));
        assert_eq!(grid.get(Point2D::new(1, 0)).cloned(), Some(TestCell::Off));
        assert_eq!(grid.get(Point2D::new(2, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(3, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(4, 0)).cloned(), Some(TestCell::Off));
        assert_eq!(grid.get(Point2D::new(5, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(6, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(7, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(8, 0)).cloned(), Some(TestCell::On));
        assert_eq!(grid.get(Point2D::new(9, 0)).cloned(), Some(TestCell::Off));
    }

    #[test]
    fn test_grid_bounds() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        assert_eq!(grid.get(Point2D::new(-1, 0)), None);
        assert_eq!(grid.get(Point2D::new(10, 0)), None);
        assert_eq!(grid.get(Point2D::new(0, -1)), None);
        assert_eq!(grid.get(Point2D::new(0, 10)), None);
        assert_eq!(grid.get(Point2D::new(10, 10)), None);
        assert_eq!(grid.get(Point2D::new(-1, -1)), None);

        assert!(grid.get(Point2D::new(0, 0)).is_some());
        assert!(grid.get(Point2D::new(0, 9)).is_some());
        assert!(grid.get(Point2D::new(9, 0)).is_some());
        assert!(grid.get(Point2D::new(9, 9)).is_some());
    }

    #[test]
    fn test_grid_display() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let grid_display = format!("{grid}");
        
        // -1 for expected trailing new line on display
        assert_eq!(TEST_GRID, &grid_display[..grid_display.len()-1])
    }

    #[test]
    fn test_neighbours_cardinal() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let test_point = Point2D::new(1, 1);
        let mut neighbours = grid.point_neighbours::<CardinalDirection>(test_point);
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::N.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::E.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::S.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::W.delta()));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_neighbours_ordinal() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let test_point = Point2D::new(1, 1);
        let mut neighbours = grid.point_neighbours::<OrdinalDirection>(test_point);
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::N.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::NE.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::E.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::SE.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::S.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::SW.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::W.delta()));
        assert_eq!(neighbours.next(), Some(test_point + OrdinalDirection::NW.delta()));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_neighbours_boundary_top_left() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let test_point = Point2D::new(0, 0);
        let mut neighbours = grid.point_neighbours::<CardinalDirection>(test_point);
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::E.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::S.delta()));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_neighbours_boundary_bottom_right() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let test_point = Point2D::new(9, 9);
        let mut neighbours = grid.point_neighbours::<CardinalDirection>(test_point);
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::N.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::W.delta()));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn test_neighbours_boundary_edge() {
        let grid = Grid2DParser::<TestCell>::parse_input(TEST_GRID);
        let test_point = Point2D::new(5, 9);
        let mut neighbours = grid.point_neighbours::<CardinalDirection>(test_point);
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::N.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::E.delta()));
        assert_eq!(neighbours.next(), Some(test_point + CardinalDirection::W.delta()));
        assert_eq!(neighbours.next(), None);
    }
}