#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {{
        let __start = std::time::Instant::now();
        {
            $block
        };
        let __duration = __start.elapsed();
        println!("[TIMING] '{}' took: {:?}", $name, __duration);
    }};

    ($name:expr, $fn:ident) => {
        time!($name, { $fn() })
    };
}

#[macro_export]
macro_rules! day {
    ($day:tt, $fn:ident) => {
        time!(format!("Day {}", $day), {
            println!("# Day {}", $day);
            $fn(get_input!($day));
        });
        println!("-----");
    };
}

#[macro_export]
macro_rules! get_input {
    ($day:tt) => {
        std::fs::read_to_string(&format!("inputs/day{}.txt", $day)).expect("Couldn't read input-file!")
    };
}
