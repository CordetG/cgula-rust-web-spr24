use crate::error::StoreErr;
use std::collections::HashMap;
use std::num::ParseIntError;

/*
/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<u32>,
    /// The index of the first item which has to be returned
    pub offset: u32,
}*/

/// Pagination struct which is getting extract
/// from query params
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item which has to be returned
    pub start: usize,
    /// The index of the last item which has to be returned
    pub end: usize,
}

/// Extract query parameters from the `/questions` route
///
/// # Example query
///
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
///
/// # Example usage
///
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, StoreErr> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it to a number
            limit: Some(params.get("limit").unwrap().parse::<u32>().map_err(
                |error: std::num::ParseIntError| -> ParseIntError {
                    std::num::ParseIntError::into(error)
                },
            )?),
            // Takes the "offset" parameter in the query
            // and tries to convert it to a number
            offset: params.get("offset").unwrap().parse::<u32>().map_err(
                |error: std::num::ParseIntError| -> ParseIntError {
                    std::num::ParseIntError::into(error)
                },
            )?,
        });
    }

    Err(StoreErr::MissingParameters(
        "Missing Parameters".to_string(),
    ))
}
