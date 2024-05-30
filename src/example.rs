use std::convert::Infallible;

// NOTE: this is an example of some type of component
//  we would need to keep in state for our app. This
//   could be a database connection, a file handle, etc.
// For now we will just use an empty struct as a placeholder
//  as a demonstration of how this would work.
#[derive(Debug, Clone)]
pub struct ExampleThing;

impl ExampleThing {
    pub fn new() -> Self {
        Self
    }

    pub fn is_ok(&self) -> Result<(), Infallible> {
        Ok(())
    }
}
