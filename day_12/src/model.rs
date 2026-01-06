use common::Grid2D;


#[derive(Clone, Copy, Debug)]
pub enum PresentCell {
    Present,
    Blank
}

impl TryFrom<char> for PresentCell {
    type Error = ();
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(PresentCell::Present),
            '.' => Ok(PresentCell::Blank),
            _ => Err(())
        }
    }
}


pub struct TreeRegion {
    pub width: isize,
    pub height: isize,
    pub present_counts: Vec<usize>
}

pub struct PuzzleSummary {
    pub present_shapes: Vec<Grid2D<PresentCell>>,
    pub regions: Vec<TreeRegion>
}