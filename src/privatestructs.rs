/// A simple counter.
#[derive(Debug)]
pub struct Counter {
    count: usize,
}

impl Counter {
    /// Creates a new `Counter` that starts at 0.
    #[inline]
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    /// Returns a reference to the current count.
    #[inline]
    pub fn get(&self) -> &usize {
        &self.count
    }

    /// Increments the counter by 1.
    #[inline]
    pub fn inc(&mut self) {
        self.count += 1;
    }

    /// Decrements the counter by 1.
    #[inline]
    pub fn dec(&mut self) {
        self.count -= 1;
    }
}

/// A simple on/off switch.
#[derive(Debug)]
pub struct Switch {
    switch: bool,
}

impl Switch {
    /// Creates a new `Switch` that is initially off.
    #[inline]
    pub fn new() -> Switch {
        Switch { switch: false }
    }

    /// Returns `true` if the switch is on, `false` if it is off.
    #[inline]
    pub fn is_on(&self) -> bool {
        self.switch
    }

    /// Turns the switch on.
    #[inline]
    pub fn on(&mut self) {
        self.switch = true;
    }

    /// Turns the switch off.
    #[inline]
    pub fn off(&mut self) {
        self.switch = false;
    }
}
