use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct StateManager {
    states: HashMap<u64, Box<dyn Any>>,
    id: u64,
}

impl StateManager {
    fn new() -> Self {
        StateManager {
            states: HashMap::new(),
            id: 0,
        }
    }

    pub fn reset_id(&mut self) {
        self.id = 0;
    }

    pub fn set_state<T: 'static>(&mut self, id: u64, value: T) {
        self.states.insert(id, Box::new(value));
    }

    pub fn get_state<T: 'static + Clone>(&mut self, id: u64) -> Option<T> {
        if let Some(state) = self.states.get(&id) {
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

pub fn callback<T>(state: &State<T>, f: impl Fn(&mut T)) -> impl Fn()
where
    T: Clone + 'static,
{
    let state = state.clone();

    move || {
        f(&mut state.borrow_mut());
        state.notify();

        STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            manager.set_state(state.id, state.data.clone().borrow().clone());
        })
    }
}

pub struct State<T: 'static + Clone> {
    data: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<std::vec::Vec<Box<dyn FnMut(&mut T)>>>>,
    id: u64,
}

impl<T: Clone + 'static> State<T> {
    pub fn new(value: T) -> Self {
        STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            let id = manager.id;
            manager.id += 1;
            if let Some(other) = manager.get_state::<T>(id) {
                return State {
                    data: Rc::new(RefCell::new(other)),
                    subscribers: Rc::new(RefCell::default()),
                    id,
                };
            } else {
                return State {
                    data: Rc::new(RefCell::new(value)),
                    subscribers: Rc::new(RefCell::default()),
                    id,
                };
            }
        })
    }

    pub fn value(&self) -> T {
        self.data.borrow().clone()
    }

    pub fn subscribe<F>(&self, closure: F)
    where
        F: FnMut(&mut T) + 'static,
    {
        self.subscribers.borrow_mut().push(Box::new(closure));
    }

    pub fn notify(&self) {
        for subscriber in self.subscribers.borrow_mut().iter_mut() {
            subscriber(&mut self.data.borrow_mut());
        }
    }
}

impl<T: Clone> std::clone::Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            data: self.data.clone(),
            subscribers: self.subscribers.clone(),
            id: self.id,
        }
    }
}

impl<T: Clone> Deref for State<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.data).expect("error: multiple references")
    }
}
