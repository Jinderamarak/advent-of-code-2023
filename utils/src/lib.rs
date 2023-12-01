use std::{
    env,
    fs::{self, ReadDir},
    io,
    path::PathBuf,
};

/// Loads text file from the `inputs` folder.
pub fn get_example_input(name: &str) -> anyhow::Result<String> {
    let inputs = get_path_to_inputs()?;
    let full = inputs.join(name);
    let data = fs::read_to_string(full)?;
    Ok(data)
}

/// Fetches the input from the AoC API.
/// Requires AOC_SESSION env variable set.
pub fn get_full_input(year: u16, day: u8) -> anyhow::Result<String> {
    if has_local(year, day)? {
        load_local(year, day)
    } else {
        let input = load_remote(year, day)?;
        save_local(year, day, &input)?;
        Ok(input)
    }
}

fn load_remote(year: u16, day: u8) -> anyhow::Result<String> {
    dotenv::dotenv()?;
    let session = env::var("AOC_SESSION")?;
    fetch_input(year, day, &session)
}

fn fetch_input(year: u16, day: u8, session: &str) -> anyhow::Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let cookie = format!("session={session}");

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("Cookie", cookie).send()?;

    let data = response.text()?;
    Ok(data)
}

fn save_local(year: u16, day: u8, input: &str) -> anyhow::Result<()> {
    let path = get_path_to_day(year, day)?;
    fs::write(path, input)?;
    Ok(())
}

fn load_local(year: u16, day: u8) -> anyhow::Result<String> {
    let path = get_path_to_day(year, day)?;
    let input = fs::read_to_string(path)?;
    Ok(input)
}

fn has_local(year: u16, day: u8) -> anyhow::Result<bool> {
    let path = get_path_to_day(year, day)?;
    Ok(path.exists())
}

fn get_path_to_day(year: u16, day: u8) -> anyhow::Result<PathBuf> {
    let inputs = get_path_to_inputs()?.join("full");

    fs::create_dir_all(inputs.clone())?;
    let full_path = inputs.join(format!("{year}-{day}")).with_extension("txt");
    Ok(full_path)
}

const POSSIBLE_PATHS: [&'static str; 2] = [".", ".."];
fn get_path_to_inputs() -> anyhow::Result<PathBuf> {
    for prefix in POSSIBLE_PATHS {
        let dir = fs::read_dir(prefix)?;
        if readdir_contains_inputs(dir)? {
            return Ok([prefix, "inputs"].iter().collect());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Inputs folder not found").into())
}

fn readdir_contains_inputs(dir: ReadDir) -> anyhow::Result<bool> {
    for entry in dir {
        let entry = entry?;
        if entry.file_name() == "inputs" {
            let t = entry.file_type()?;
            return Ok(t.is_dir());
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_example_input() {
        let input = get_example_input("test.txt").unwrap();
        assert_eq!("test case input\n123\n", input);
    }
}
