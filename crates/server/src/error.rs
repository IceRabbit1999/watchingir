use ir_aquila::stack_error;
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[snafu(visibility(pub(crate)))]
#[stack_error]
pub enum Error {
    #[snafu(display("SteamApi: Error when requesting to: {}", entrypoint))]
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
}
