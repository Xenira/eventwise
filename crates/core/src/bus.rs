use tokio::sync::mpsc;

use crate::aggregate::Aggregate;

// pub struct CommandBus {
//     router: mpsc::UnboundedSender<Dispatch>,
// }
//
// impl CommandBus {
//     pub fn spawn() -> (Self, CommandBusHandle) { /* router_loop + channels */ }
//
//     pub fn dispatch<D, C>(&self, cmd: C)
//     where
//         D: Aggregate + 'static,
//         C: Command<D> + Send + 'static,
//         D::Event: Send + 'static,
//         /* executor generic bounds */
//     {
//         let key = cmd.id().key().to_string();
//         let kind = D::KIND;                    // or StreamId::kind on D::Id
//         let exec = move |repo: Repository<..., D>| async move {
//             match repo.execute(&cmd).await {
//                 Ok(events) => { /* publish to event bus, notify caller */ }
//                 Err(e)    => { /* log / dead-letter / reply channel */ }
//             }
//         };
//         let _ = self.router.send(Dispatch { kind, stream_key: key, envelope: Box::new(exec) });
//     }
// }
