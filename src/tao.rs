use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use tao::platform::run_return::EventLoopExtRunReturn;

pub trait EventLoopExtRunResult {
    type UserEvent;

    fn run_result<F, E>(&mut self, event_handler: F) -> Result<i32, E>
        where
            F: FnMut(Event<'_, Self::UserEvent>, &EventLoopWindowTarget<Self::UserEvent>, &mut ControlFlow) -> Result<(), E>;
}

impl<T> EventLoopExtRunResult for EventLoop<T> {
    type UserEvent = T;

    fn run_result<F, E>(&mut self, mut event_handler: F) -> Result<i32, E>
        where F: FnMut(Event<'_, Self::UserEvent>, &EventLoopWindowTarget<Self::UserEvent>, &mut ControlFlow) -> Result<(), E>
    {
        let mut result = None;
        let code = self.run_return(|event, target, flow| {
            if let Err(error) = event_handler(event, target, flow) {
                result.get_or_insert(error);
                *flow = ControlFlow::Exit;
            }
        });
        result.map_or(Ok(code), |error| Err(error))
    }
}
