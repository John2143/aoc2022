mod code;
pub mod shared;

fn main() {
    let lookup_table = vec![
        (code::p1::a as fn(), code::p1::b as fn()),
        (code::p2::a, code::p2::b),
        (code::p3::a, code::p3::b),
        (code::p4::a, code::p4::b),
        (code::p5::a, code::p5::b),
        (code::p6::a, code::p6::b),
        (code::p7::a, code::p7::b),
        (code::p8::a, code::p8::b),
        (code::p9::a, code::p9::b),
        (code::p10::a, code::p10::b),
        (code::p11::a, code::p11::b),
        (code::p12::a, code::p12::b),
        (code::p13::a, code::p13::b),
        (code::p14::a, code::p14::b),
        (code::p15::a, code::p15::b),
        (code::p16::a, code::p16::b),
        (code::p17::a, code::p17::b),
        (code::p18::a, code::p18::b),
        (code::p19::a, code::p19::b),
        (code::p20::a, code::p20::b),
        (code::p21::a, code::p21::b),
        (code::p22::a, code::p22::b),
        (code::p23::a, code::p23::b),
        (code::p24::a, code::p24::b),
        (code::p25::a, code::p25::b),
    ];

    let day: usize = match std::env::args().nth(1) {
        Some(d) => match d.parse() {
            Ok(d) => d,
            Err(_) => {
                println!(
                    "First argument must be a digit between 1 and 25, got '{}'",
                    d
                );
                return;
            }
        },
        None => {
            println!("First argument must be a digit between 1 and 25");
            return;
        }
    };

    let (a, b) = match lookup_table.get(day.wrapping_sub(1)) {
        Some(x) => x,
        None => {
            println!("Day #{} is out of bounds", day);
            return;
        }
    };

    (a)();
    (b)();
}
