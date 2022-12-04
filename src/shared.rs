use nom::{bytes::complete::take_while, combinator::map_res, IResult};

pub fn read_number(input: &str, base: u32) -> IResult<&str, usize> {
    map_res(take_while(|c: char| c.is_digit(base)), |num_str| {
        usize::from_str_radix(num_str, base)
    })(input)
}
