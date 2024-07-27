#![feature(never_type)]
#![allow(non_snake_case, non_camel_case_types)]

mod consts;
pub use consts::{
    followersVisibilities, followingVisibilities, moderationLogTypes, mutedNoteReasons,
    noteVisibilities, notificationTypes, permissions,
};
