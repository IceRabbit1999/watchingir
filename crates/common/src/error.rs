use ir_aquila::stack_error;
use snafu::{Location, Snafu};

#[derive(Snafu)]
#[stack_error]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("NoneValue: expected {} but found None", expected))]
    NoneValue {
        expected: String,
        #[snafu(implicit)]
        location: Location,
    },
}
