use crate::location::Location;

#[derive(Debug)]
pub struct Link {
    pub source: Location,
    pub target: Location,
}
