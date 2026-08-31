use crate::data::{Status, Ticket, TicketDraft, TicketPatch};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn update(&mut self, ticket_patch: TicketPatch) -> Result<(), ()> {
        // sleepy, will prob handle the `unwrap` another day
        let ticket_mut = self.tickets.get_mut(&ticket_patch.id).unwrap();

        if let Some(item) = ticket_patch.status {
            ticket_mut.status = item;
        }

        if let Some(item) = ticket_patch.description {
            ticket_mut.description = item.clone();
        }

        if let Some(item) = ticket_patch.title {
            ticket_mut.title = item.clone();
        }

        Ok(())
    }
}
