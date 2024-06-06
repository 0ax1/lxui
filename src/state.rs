use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct StateManager {
    states: HashMap<(TypeId, u64), Box<dyn Any>>,
}

impl StateManager {
    fn new() -> Self {
        StateManager {
            states: HashMap::new(),
        }
    }

    pub fn set_state<T: 'static>(&mut self, id: u64, value: T) {
        self.states.insert((TypeId::of::<T>(), id), Box::new(value));
    }

    pub fn get_state<T: 'static + Clone>(&mut self, id: u64) -> Option<T> {
        if let Some(state) = self.states.get(&(TypeId::of::<T>(), id)) {
            return Some(
                state
                    .downcast_ref::<T>()
                    .expect("error: failed downcast")
                    .clone(),
            );
        }

        None
    }
}

thread_local! {
    pub static STATE_MANAGER: RefCell<StateManager> = RefCell::new(StateManager::new());
}
