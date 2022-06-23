impl warp::reject::Reject for Error {

}

#[derive(Debug)]
pub struct Error {
    msg: String
}

impl<T: ToString> From<T> for Error {
    fn from(e: T) -> Self {
        Error {
            msg: e.to_string()
        }
    }
}