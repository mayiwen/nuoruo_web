pub struct SignalFn {
    f: Box<dyn Fn()>,
    i: usize,
}
pub struct Signal<T> {
    v: T,
    sub: Vec<SignalFn>,
    i: usize,
}

impl<T> Signal<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: t,
            sub: vec![],
            i: 0,
        }
    }
    pub fn get(&self) -> &T {
        &self.v
    }
    pub fn set(&mut self, t: T) {
        self.v = t;
        for sub in &mut self.sub {
            (sub.f)();
        }
    }
    pub fn subscribe(&mut self, f: Box<dyn Fn()>) -> usize {
        self.i += 1;
        let signal_fn = SignalFn { f, i: self.i };
        self.sub.push(signal_fn);
        self.i
    }
    pub fn unsubscribe_by_id_vec(&mut self, ids: Vec<usize>) {
        self.sub.retain(|x| !ids.contains(&x.i));
    }
    pub fn unsubscribe(&mut self, id: usize) {
        self.sub.retain(|x| x.i != id);
    }
    pub fn unsubscribe_all(&mut self) {
        self.sub.clear();
    }
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.v);
        for sub in &mut self.sub {
            (sub.f)();
        }
    }
}

/// 进行基础测试
#[test]
fn test() {
    println!("test");
    let mut s = Signal::new(10);
    let v = s.get();
    println!("{}", v);
    s.set(20);
    s.set(30);
    println!("{}", s.get());
}
