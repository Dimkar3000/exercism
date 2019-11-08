// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x:isize,
    y:isize,
    r:Direction,
}
impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{x:x,y:y,r:d}
    }

    pub fn turn_right(mut self) -> Self {
        match self.r{
            Direction::North => self.r = Direction::East,
            Direction::East  => self.r = Direction::South,
            Direction::South => self.r = Direction::West,
            Direction::West  => self.r = Direction::North,
        }
        self

    }

    pub fn turn_left(mut self) -> Self {
        match self.r{
            Direction::North => self.r = Direction::West,
            Direction::East  => self.r = Direction::North,
            Direction::South => self.r = Direction::East,
            Direction::West  => self.r = Direction::South,
        }
        self
    }

    pub fn advance(mut self) -> Self {
        match self.r{
            Direction::North => self.y += 1,
            Direction::East  => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West  => self.x -= 1,
        }
        self

    }

    #[allow(unused_variables)]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for i in instructions.chars(){
            if i == 'L'{
                self =  self.turn_left();
            }else if i == 'R'{
                self = self.turn_right();
            }else if i == 'A'{
                self = self.advance();
            }
        }
        self
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x,self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.r
    }
}