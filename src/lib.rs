use std::fs;

pub fn parse_args(day: u8) -> anyhow::Result<String> {
    let args = std::env::args().collect::<Vec<String>>();
    // if --test in args, return the test input
    let dir = match args.contains(&"--test".to_string()) {
        true => format!("inputs/{}-t.txt", day),
        false => format!("inputs/{}.txt", day),
    };
    Ok(fs::read_to_string(dir)?)
}
