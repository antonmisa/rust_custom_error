mod myerror;

use myerror::CliError;
use std::path::Path;
use std::fs::File;
use std::io::Read;

/// Function with custom error
///
/// We do not use map_err, because CliError implements From::from
/// and macro ? (try!) realize it call on errors.
///
fn file_double<P: AsRef<Path>>(file_path: P) -> Result<f32, CliError> {
	let mut file = File::open(file_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let n: f32 = contents.trim().parse()?;
	Ok(2.*n)
}

fn main() {
    match file_double("test.txt") {
		Ok(n) => println!("{}", n),
		Err(err) => println!("Error: {}", err),
	}
}
