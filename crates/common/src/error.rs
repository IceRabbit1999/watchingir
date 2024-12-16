use ir_aquila::stack_error;
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[stack_error]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Error when requesting to: {}", entrypoint))]
    SteamApi {
        entrypoint: String,
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: reqwest::Error,
    },
    #[snafu(display("DataFormat: Invalid data format when trying to parse response to {}", data))]
    DataFormat {
        data: String,
        #[snafu(implicit)]
        location: Location,
        #[snafu(source)]
        error: reqwest::Error,
    },
    #[snafu(display("NoneValue: expected {} but found None", expected))]
    NoneValue {
        expected: String,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("{message}"))]
    #[snafu(whatever)]
    Anyhow {
        message: String,
        #[snafu(implicit)]
        location: Location,
        #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
        error: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
