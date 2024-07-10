#![feature(never_type)]
#![allow(non_snake_case, non_camel_case_types)]

mod api_types;
pub use api_types::Endpoints;

mod streaming;

pub use streaming::{Connection as ChannelConnection, Stream};

mod streaming_types;
pub use streaming_types::Channels;

pub mod acct;
pub use acct::Acct;

mod consts;
pub use consts::{
    followersVisibilities, followingVisibilities, moderationLogTypes, mutedNoteReasons,
    noteVisibilities, notificationTypes, permissions,
};

pub mod api;
pub mod entities;

// temp
mod autogen;
