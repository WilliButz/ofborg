#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate amqp;
extern crate fs2;
extern crate md5;

pub mod checkout;
pub mod locks;
pub mod clone;
pub mod worker;
pub mod config;
pub mod message;

pub mod ofborg {
    pub use config;
    pub use checkout;
    pub use locks;
    pub use clone;
    pub use worker;
    pub use message;

}