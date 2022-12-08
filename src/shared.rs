use nom::{bytes::complete::take_while, combinator::map_res, IResult};

pub fn read_number(input: &str, base: u32) -> IResult<&str, usize> {
    map_res(take_while(|c: char| c.is_digit(base)), |num_str| {
        usize::from_str_radix(num_str, base)
    })(input)
}

pub fn read_number_parser<const BASE: u32>(input: &str) -> IResult<&str, usize> {
    read_number(input, BASE)
}

pub fn read_number_10(input: &str) -> IResult<&str, usize> {
    read_number(input, 10)
}

pub fn read_number_16(input: &str) -> IResult<&str, usize> {
    read_number(input, 16)
}
