use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

/// Extract query parameters from the `/questions` route
///
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
///
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?),
                offset: params
                    .get("offset")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?
        });
    }

    Err(Error::MissingParameters)
}
