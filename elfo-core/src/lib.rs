#![allow(missing_docs)] // TODO
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
extern crate static_assertions;
#[macro_use]
extern crate elfo_utils;

// To make `#[message]` and `msg!` work inside `elfo-core`.
extern crate self as elfo_core;

// TODO: revise this list
pub use crate::{
    actor::{ActorMeta, ActorStartCause, ActorStartInfo, ActorStatus, ActorStatusKind},
    addr::Addr,
    config::Config,
    context::{send::Send, Context, RequestBuilder},
    envelope::Envelope,
    group::{ActorGroup, Blueprint, TerminationPolicy},
    local::{Local, MoveOwnership},
    message::{AnyMessage, AnyMessageRef, Message, Request},
    request_table::ResponseToken,
    restarting::{RestartParams, RestartPolicy},
    source::{SourceHandle, UnattachedSource},
    topology::Topology,
};
pub use elfo_macros::{message_core as message, msg_core as msg};

#[macro_use]
mod macros;

pub mod config;
pub mod coop;
pub mod dumping;
pub mod errors;
pub mod init;
pub mod logging;
pub mod messages;
pub mod node;
pub mod routers;
pub mod scope;
pub mod signal;
pub mod stream;
#[cfg(feature = "unstable-stuck-detection")]
pub mod stuck_detection;
pub mod time;
pub mod topology;
pub mod tracing;

mod actor;
mod addr;
mod address_book;
mod context;
mod demux;
mod envelope;
mod exec;
mod group;
mod local;
mod mailbox;
#[cfg(target_os = "linux")]
mod memory_tracker;
mod message;
mod object;
mod panic;
mod permissions;
#[cfg(all(feature = "network", feature = "unstable"))]
pub mod remote;
#[cfg(all(feature = "network", not(feature = "unstable")))]
mod remote;
mod request_table;
mod restarting;
mod runtime;
mod source;
mod subscription;
mod supervisor;
mod telemetry;
mod thread;

#[doc(hidden)]
pub mod _priv {
    pub mod node {
        pub fn set_node_no(node_no: u16) {
            crate::node::set_node_no(node_no)
        }
    }

    #[cfg(feature = "unstable")]
    pub use crate::addr::{GroupNo, NodeLaunchId, NodeNo};
    pub use crate::{
        address_book::AddressBook,
        envelope::{EnvelopeBorrowed, EnvelopeOwned, MessageKind},
        init::do_start,
        message::*,
        object::{GroupVisitor, Object, OwnedObject},
        permissions::{AtomicPermissions, Permissions},
        request_table::RequestId,
    };
    pub use erased_serde;
    pub use idr_ebr::EbrGuard;
    pub use linkme;
    pub use metrics;
    #[cfg(feature = "network")]
    pub use rmp_serde as rmps;
    pub use serde;
    pub use smallbox;
}
