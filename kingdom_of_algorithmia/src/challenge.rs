pub mod c1;

pub trait Challenge {
    fn name(&self) -> String;
    fn parse_input(&mut self);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    fn part3(&self) -> String;
}

pub enum RunBehavior {
    Last,
    Specific(usize),
}

pub enum PrintType {
    ParsingInput,
    Part1,
    Part2,
    Part3,
}
