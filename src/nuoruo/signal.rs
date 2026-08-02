
pub struct SignalFn {
    f: Box<dyn Fn()>,
    id: usize,
}
pub struct Signal<T> {
    v: T,
    sub: Vec<SignalFn>,
    id: usize,
}

impl<T> Signal<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: t,
            sub: vec![],
            id: 0,
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
        self.id += 1;
        let signal_fn = SignalFn {
            f,
            id: self.id,
        };
        self.sub.push(signal_fn);
        self.id
    }
    pub fn unsubscribe_by_id_vec(&mut self, ids: Vec<usize>) {
        self.sub.retain(|x| !ids.contains(&x.id));
    }
    pub fn unsubscribe(&mut self, id: usize) {
        self.sub.retain(|x| x.id != id);
    }
    pub fn unsubscribe_all(&mut self) {
        self.sub.clear();
    }
    pub fn update<F>(&mut self, f: F) where F: FnOnce(&mut T), {
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
    // println!("{:?}", s);

    let v = s.get();
    println!("{}", v);
    s.set(20);
    s.set(30);
    println!("{}", s.get());
}
