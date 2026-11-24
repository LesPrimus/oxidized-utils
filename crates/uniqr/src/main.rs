use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("crates/uniqr/tests/inputs/three.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    // counters
    let mut results = vec![];
    let mut previous = String::new();
    let mut count: usize = 0;

    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }

        // Compare without the line ending, so "a\n" and a final "a" match.
        if count > 0
            && buf.trim_end_matches(['\r', '\n']) == previous.trim_end_matches(['\r', '\n'])
        {
            count += 1;
        } else {
            if count > 0 {
                results.push((previous.clone(), count));
            }
            std::mem::swap(&mut previous, &mut buf);
            count = 1;
        }
    }
    if count > 0 {
        results.push((previous, count));
    }

    dbg!(results);

    Ok(())
}
