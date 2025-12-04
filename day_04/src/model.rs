use common::{
    Grid2D, 
    OrdinalDirection, 
    Point2D
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RoomCell {
    Blank,
    PaperRoll
}

impl TryFrom<char> for RoomCell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(RoomCell::Blank),
            '@' => Ok(RoomCell::PaperRoll),
            _ => Err(())
        }
    }
}

impl std::fmt::Display for RoomCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chr = match self {
            RoomCell::Blank => '.',
            RoomCell::PaperRoll => '@',
        };

        write!(f,"{chr}")
    }
}

const MAX_ROLL_NEIGHBOURS: usize = 4;

pub fn get_available_rolls(input: &Grid2D<RoomCell>) -> impl Iterator<Item = Point2D> {
    let roll_counts= input
        .element_iter_filtered(&RoomCell::PaperRoll)
        .map(|(_cell, point)| {
            let roll_count = input.element_neighbours_filtered::<OrdinalDirection>(
                point,
                &RoomCell::PaperRoll
            ).count();

            (point, roll_count)
        });

    roll_counts
        .filter(|(_, count)| *count < MAX_ROLL_NEIGHBOURS)
        .map(|(point, _)| point)
}

pub fn count_available_rolls(input: &Grid2D<RoomCell>) -> usize {
    get_available_rolls(input).count()
}

pub fn retrieve_available_rolls(input: &mut Grid2D<RoomCell>) -> Vec<Point2D> {
    let rolls: Vec<Point2D> = get_available_rolls(input).collect();

    // Remove from input
    for point in &rolls {
        let roll_element = input.get_mut(*point).unwrap();
        *roll_element = RoomCell::Blank
    }

    rolls
}

pub fn count_available_rolls_iterative(input: &mut Grid2D<RoomCell>) -> usize {
    let mut total_count = 0;
    loop {
        let roll_count = retrieve_available_rolls(input).len();
        if roll_count == 0 {
            break;
        }

        total_count += roll_count;
    } 

    total_count
}