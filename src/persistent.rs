use iron::{Request, Response, Middleware, Alloy, Status, Continue};
use std::sync::{Arc, RWLock};

/// A `Middleware` that allows for sharing a single piece of data between
/// requests under an `RWLock`.
///
/// `Persistent` has two type parameters: `Data`, which dictates the type of
/// the contained data, and `Phantom`, which is used to differentiate the types
/// of multiple instances of `Persistent` that contain the same data.
///
/// This phantom type parameter is used so that multiple `Persistent` instances
/// containing the same kind of data can be stored in the same `Alloy`.
pub struct Persistent<Data, Phantom> {
    /// The data contained within Persistent
    pub data: Arc<RWLock<Data>>
}

impl<D: Send + Share, P> Clone for Persistent<D, P> {
    fn clone(&self) -> Persistent<D, P> {
        Persistent {
            data: self.data.clone()
        }
    }
}

impl<D: Send + Share, P> Middleware for Persistent<D, P> {
    fn enter(&mut self, _: &mut Request,
             _: &mut Response, alloy: &mut Alloy) -> Status {
        alloy.insert::<Persistent<D, P>>(self.clone());
        Continue
    }
}

impl<D: Send + Share, P> Persistent<D, P> {
    /// Creates a new instance of `Persistent` containing the passed-in data.
    pub fn new(data: D) -> Persistent<D, P> {
        Persistent { data: Arc::new(RWLock::new(data)) }
    }
}

