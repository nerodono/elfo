use std::{cell::RefCell, sync::Arc};

use parking_lot::RwLock;

use crate::{
    addr::Addr,
    address_book::{AddressBook, VacantEntry},
    context::Context,
    demux::{Demux, Filter},
    envelope::Envelope,
    group::Schema,
};

#[derive(Clone)]
pub struct Topology {
    book: AddressBook,
    inner: Arc<RwLock<Inner>>,
}

#[derive(Default)]
struct Inner {
    groups: Vec<ActorGroup>,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone)]
pub struct ActorGroup {
    pub addr: Addr,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub from: Addr,
    pub to: Addr,
}

impl Topology {
    pub fn empty() -> Self {
        Self {
            book: AddressBook::new(),
            inner: Arc::new(RwLock::new(Inner::default())),
        }
    }

    pub fn local(&self, name: impl Into<String>) -> Local<'_> {
        let name = name.into();
        let entry = self.book.vacant_entry();

        let mut inner = self.inner.write();
        inner.groups.push(ActorGroup {
            addr: entry.addr(),
            name: name.clone(),
        });

        Local {
            name,
            topology: self,
            entry,
            demux: RefCell::new(Demux::default()),
        }
    }

    pub fn remote(&self, _name: impl Into<String>) -> Remote<'_> {
        todo!()
    }

    pub fn actor_groups(&self) -> impl Iterator<Item = ActorGroup> + '_ {
        let inner = self.inner.read();
        inner.groups.clone().into_iter()
    }

    pub fn connections(&self) -> impl Iterator<Item = Connection> + '_ {
        let inner = self.inner.read();
        inner.connections.clone().into_iter()
    }
}

impl Default for Topology {
    fn default() -> Self {
        Self::empty()
    }
}

#[must_use]
pub struct Local<'t> {
    name: String,
    topology: &'t Topology,
    entry: VacantEntry<'t>,
    demux: RefCell<Demux>,
}

impl<'t> Local<'t> {
    pub fn route_to(
        &self,
        dest: &impl GetAddrs,
        filter: impl Fn(&Envelope) -> bool + Send + Sync + 'static,
    ) {
        let filter = Arc::new(filter);
        for addr in dest.addrs() {
            self.demux
                .borrow_mut()
                .append(addr, Filter::Dynamic(filter.clone()));
        }
    }

    pub fn route_all_to(&self, dest: &impl GetAddrs) {
        // TODO: more efficient impls.
        self.route_to(dest, |_| true)
    }

    pub async fn mount<M: crate::Message>(self, schema: Schema, msg: Option<M>) {
        let addr = self.entry.addr();
        let book = self.topology.book.clone();
        let root = Context::new(book.clone(), Demux::default());
        let ctx = Context::new(book, self.demux.into_inner()).with_addr(addr);
        let object = (schema.run)(ctx, self.name);
        self.entry.insert(object);

        if let Some(msg) = msg {
            match root.send_to(addr, msg).await {
                Ok(_) => tracing::info!("ok"),
                Err(_) => tracing::error!("fail"),
            }
        }
    }
}

pub struct Remote<'t> {
    addr: Addr,
    _topology: &'t Topology,
}

pub trait GetAddrs {
    fn addrs(&self) -> Vec<Addr>;
}

impl<'t> GetAddrs for Local<'t> {
    fn addrs(&self) -> Vec<Addr> {
        vec![self.entry.addr()]
    }
}

impl<'t> GetAddrs for Remote<'t> {
    fn addrs(&self) -> Vec<Addr> {
        vec![self.addr]
    }
}