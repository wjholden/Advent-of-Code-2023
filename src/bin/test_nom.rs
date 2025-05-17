use nom; // 7.1.3
use nom::{bytes::complete::tag, character::complete::alpha1, multi::separated_list1, IResult, Parser};

fn main() {
    //println!("{:?}", parse_lines7("Hello, world").unwrap());
    println!("{:?}", parse_lines8("Hello, world").unwrap());
}

// https://docs.rs/nom/7.1.3/nom/multi/fn.separated_list1.html
// https://github.com/rust-bakery/nom/blob/main/CHANGELOG.md
//fn parse_lines7(input: &str) -> IResult<&str, Vec<&str>> {
//    separated_list1(tag(", "), alpha1)(input)
//}

// https://docs.rs/nom/8.0.0/nom/multi/fn.separated_list1.html
fn parse_lines8(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1).parse(input)
}
