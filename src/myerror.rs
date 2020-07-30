use std::io;
use std::num;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
	Io(io::Error),
	ParseInt(num::ParseIntError),
	ParseFloat(num::ParseFloatError)
}

impl From<io::Error> for CliError {
	fn from(err: io::Error) -> CliError {
		CliError::Io(err)
	}
}

impl From<num::ParseIntError> for CliError {
	fn from(err: num::ParseIntError) -> CliError {
		CliError::ParseInt(err)
	}
}

impl From<num::ParseFloatError> for CliError {
	fn from(err: num::ParseFloatError) -> CliError {
		CliError::ParseFloat(err)
	}
}

impl fmt::Display for CliError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			CliError::Io(ref err) => write!(f, "IO error: {}", err),
			CliError::ParseInt(ref err) => write!(f, "Parse int error: {}", err),
			CliError::ParseFloat(ref err) => write!(f, "Parse float error: {}", err),
		}
	}
}