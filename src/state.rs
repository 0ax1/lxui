use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

struct StateManager {
    states: HashMap<(TypeId, u64), Box<dyn Any>>,
}

impl StateManager {
    fn new() -> Self {
        StateManager {
            states: HashMap::new(),
        }
    }

    fn set_state<T: 'static>(&mut self, id: u64, value: T) {
        self.states.insert((TypeId::of::<T>(), id), Box::new(value));
    }

    fn get_state<T: 'static + Default + Clone>(&mut self, id: u64) -> T {
        if let Some(state) = self.states.get(&(TypeId::of::<T>(), id)) {
            return state
                .downcast_ref::<T>()
                .expect("error: failed downcast")
                .clone();
        }

        self.set_state(id, T::default());
        T::default()
    }
}

thread_local! {
    static STATE_MANAGER: RefCell<StateManager> = RefCell::new(StateManager::new());
}

pub fn showcase() {
    {
        STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            manager.set_state(0, 10);
            manager.set_state(1, 11);
            manager.set_state(0, "abc".to_owned());
        });
    }
    {
        STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            println!("{}", manager.get_state::<i32>(0));
            println!("{}", manager.get_state::<i32>(1));
            println!("{}", manager.get_state::<String>(0));
        });
    }
}
