use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod signal {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static SIGNAL_COUNTER: AtomicUsize = AtomicUsize::new(1);

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SignalId(usize);

    impl SignalId {
        fn new() -> Self {
            SignalId(SIGNAL_COUNTER.fetch_add(1, Ordering::SeqCst))
        }
    }

    #[derive(Clone)]
    pub struct Signal<T> {
        id: SignalId,
        value: Rc<RefCell<T>>,
        subscribers: Rc<RefCell<Vec<SignalId>>>,
    }

    impl<T: Clone + 'static> Signal<T> {
        pub fn new(value: T) -> Self {
            Signal {
                id: SignalId::new(),
                value: Rc::new(RefCell::new(value)),
                subscribers: Rc::new(RefCell::new(Vec::new())),
            }
        }

        pub fn id(&self) -> SignalId {
            self.id
        }

        // pub fn get(&self) -> T {
        //     self.track();
        //     self.value.borrow().clone()
        // }

        // pub fn set(&self, new_value: T) {
        //     *self.value.borrow_mut() = new_value;
        //     self.notify();
        // }

        // pub fn update<F>(&self, f: F)
        // where
        //     F: FnOnce(&mut T),
        // {
        //     f(&mut self.value.borrow_mut());
        //     self.notify();
        // }

        // pub fn subscribe(&self, id: SignalId) {
        //     let mut subs = self.subscribers.borrow_mut();
        //     if !subs.contains(&id) {
        //         subs.push(id);
        //     }
        // }

        // fn track(&self) {
        //     if let Some(current) = CURRENT_SIGNAL.with(|s| *s.borrow()) {
        //         self.subscribe(current);
        //     }
        // }

        // fn notify(&self) {
        //     let subs: Vec<SignalId> = self.subscribers.borrow().clone();
        //     for sub_id in subs {
        //         if let Some(callback) = SUBSCRIBERS.with(|s| s.borrow().get(&sub_id).cloned()) {
        //             callback();
        //         }
        //     }
        // }
    }

    // thread_local! {
    //     static CURRENT_SIGNAL: RefCell<Option<SignalId>> = RefCell::new(None);
    //     static SUBSCRIBERS: RefCell<HashMap<SignalId, Box<dyn Fn()>>> = RefCell::new(HashMap::new());
    // }

    // pub struct Computed<T> {
    //     id: SignalId,
    //     value: Rc<RefCell<Option<T>>>,
    //     compute_fn: Box<dyn Fn() -> T + 'static>,
    //     dirty: Rc<RefCell<bool>>,
    // }

    // impl<T: Clone + 'static> Computed<T> {
    //     pub fn new<F>(compute_fn: F) -> Self
    //     where
    //         F: Fn() -> T + 'static,
    //     {
    //         Computed {
    //             id: SignalId::new(),
    //             value: Rc::new(RefCell::new(None)),
    //             compute_fn: Box::new(compute_fn),
    //             dirty: Rc::new(RefCell::new(true)),
    //         }
    //     }

    //     pub fn id(&self) -> SignalId {
    //         self.id
    //     }

    //     pub fn get(&self) -> T {
    //         if *self.dirty.borrow() {
    //             self.recompute();
    //         }
    //         self.value.borrow().clone().unwrap()
    //     }

    //     fn recompute(&self) {
    //         let old = CURRENT_SIGNAL.with(|s| *s.borrow());
    //         CURRENT_SIGNAL.with(|s| {
    //             *s.borrow_mut() = Some(self.id);
    //         });

    //         let result = (self.compute_fn)();

    //         CURRENT_SIGNAL.with(|s| {
    //             *s.borrow_mut() = old;
    //         });

    //         *self.value.borrow_mut() = Some(result);
    //         *self.dirty.borrow_mut() = false;
    //     }
    // }

    // pub struct Effect {
    //     id: SignalId,
    //     cleanup_fn: Option<Box<dyn Fn()>>,
    // }

    // impl Effect {
    //     pub fn new<F>(effect_fn: F) -> Self
    //     where
    //         F: Fn() + 'static,
    //     {
    //         let id = SignalId::new();

    //         SUBSCRIBERS.with(|s| {
    //             s.borrow_mut()
    //                 .insert(id, Box::new(effect_fn) as Box<dyn Fn()>);
    //         });

    //         Effect {
    //             id,
    //             cleanup_fn: None,
    //         }
    //     }

    //     pub fn id(&self) -> SignalId {
    //         self.id
    //     }

    //     pub fn cleanup(&self) {
    //         if let Some(cleanup) = &self.cleanup_fn {
    //             cleanup();
    //         }
    //         SUBSCRIBERS.with(|s| {
    //             s.borrow_mut().remove(&self.id);
    //         });
    //     }
    // }

    // pub struct ReactiveStore {
    //     signals: Rc<RefCell<HashMap<SignalId, Box<dyn SignalValue>>>>,
    // }

    // pub trait SignalValue: 'static {
    //     fn mark_dirty(&self);
    // }

    // impl<T: Clone + 'static> SignalValue for Signal<T> {
    //     fn mark_dirty(&self) {
    //         self.notify();
    //     }
    // }

    // impl<T: Clone + 'static> SignalValue for Computed<T> {
    //     fn mark_dirty(&self) {
    //         *self.dirty.borrow_mut() = true;
    //     }
    // }

    // impl ReactiveStore {
    //     pub fn new() -> Self {
    //         ReactiveStore {
    //             signals: Rc::new(RefCell::new(HashMap::new())),
    //         }
    //     }

    //     pub fn create_signal<T: Clone + 'static>(&self, value: T) -> Signal<T> {
    //         let signal = Signal::new(value);
    //         let id = signal.id();
    //         self.signals
    //             .borrow_mut()
    //             .insert(id, Box::new(signal.clone()) as Box<dyn SignalValue>);
    //         signal
    //     }

    //     pub fn create_computed<T: Clone + 'static, F>(&self, compute_fn: F) -> Computed<T>
    //     where
    //         F: Fn() -> T + 'static,
    //     {
    //         let computed = Computed::new(compute_fn);
    //         let id = computed.id();
    //         self.signals
    //             .borrow_mut()
    //             .insert(id, Box::new(computed.clone()) as Box<dyn SignalValue>);
    //         computed
    //     }

    //     pub fn create_effect<F>(&self, effect_fn: F) -> Effect
    //     where
    //         F: Fn() + 'static,
    //     {
    //         Effect::new(effect_fn)
    //     }
    // }

    // impl Default for ReactiveStore {
    //     fn default() -> Self {
    //         Self::new()
    //     }
    // }
}
