enum WaylandEvent {
    FrameReady,
    KeyboardInput,
}

trait Dispatch {
    fn handle(&self, event: &WaylandEvent);
}

struct EventQueue;

impl EventQueue {
    fn blocking_dispatch<T: Dispatch>(&self, user_state: &T) {
        let event = WaylandEvent::FrameReady; // for example we received this

        println!("[Library]: Received event, calling your code...");

        user_state.handle(&event);
    }
}

// ---

struct MyAppState;

impl Dispatch for MyAppState {
    fn handle(&self, event: &WaylandEvent) {
        match event {
            WaylandEvent::FrameReady => println!("[MyCode]: Cool, got you!"),
            _ => println!("[MyCode]: Ignor"),
        }
    }
}

fn main() {
    let queue = EventQueue;
    let my_app = MyAppState;

    queue.blocking_dispatch(&my_app);
}
