use core::panic;

use ticket_fields::test_helpers::{ticket_description, ticket_title};
use versioned_patch::data::{Status, TicketDraft, TicketPatch};
use versioned_patch::{launch, PatchRejectedError};

#[test]
fn works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);

    let patch = TicketPatch {
        version: 0,
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::ToDo),
    };

    let _ = client.update(patch).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::ToDo);

    let patch2 = TicketPatch {
        version: 1,
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::InProgress),
    };

    let _ = client.update(patch2).unwrap();

    let patch3 = TicketPatch {
        version: 1,
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::Done),
    };

    let result = client.update(patch3);
    assert!(matches!(result, Err(PatchRejectedError)));

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::InProgress);
}
