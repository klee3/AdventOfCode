pub trait AdventOfCodeSolution {
    fn name(&self) -> &'static str;
    fn year(&self) -> u16;
    fn day(&self) -> u8;

    fn part1(&self, input: &str) -> Result<String, String>;
    fn part2(&self, input: &str) -> Result<String, String>;
}
