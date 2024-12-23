use ir_aquila::stack_error;
use snafu::{Location, Snafu};

/// Top level error that wraps all other errors
#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[stack_error]
pub(crate) enum AppError {
    #[snafu(display("CommonError"))]
    Common {
        #[snafu(implicit)]
        location: Location,
        source: common::Error,
    },
    #[snafu(display("ServerError"))]
    Server {
        #[snafu(implicit)]
        location: Location,
        source: server::Error,
    },
    #[snafu(display("RuntimeError"))]
    Application {
        #[snafu(implicit)]
        location: Location,
        source: Error,
    },
}

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[stack_error]
pub(crate) enum Error {
    #[snafu(display("ReadJson: Error when reading json from file: {}", filename))]
    ReadFile {
        filename: String,
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: std::io::Error,
    },
    #[snafu(display("WriteJson: Error when writing json to file: {}", filename))]
    WriteFile {
        filename: String,
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: std::io::Error,
    },
    #[snafu(display("Json: Error when de/serializing json"))]
    Json {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: serde_json::Error,
    },
    #[snafu(display("Toml: Error when de/serializing toml"))]
    Toml {
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: toml::ser::Error,
    },
}
