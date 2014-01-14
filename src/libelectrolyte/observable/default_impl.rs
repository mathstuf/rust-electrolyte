/* Copyright 2014 Ben Boeckel
 *
 * Licensed under the MIT license. This file may not be copied, modified, or
 * distributed except according to those terms.
 */

//! Default implementation of an Observer.

use observable::trait_::Observer;
use observable::trait_::Observable;

struct DefaultObservable<T> {
    value: ~T,
    listeners: ~[~Observer<T>]
}

impl<T> Observable<T> for DefaultObservable<T> {
    fn update(&mut self, value: ~T) {
        self.value = value;
        self.listeners.map(|f| {
            f.notify(self.value);
        });
    }

    fn onValue(&mut self, f: ~Observer<T>) {
        self.listeners.push(f);
    }
}

/// Create a new, default observable around a value.
pub fn observe<T>(value: ~T) -> DefaultObservable<T> {
    DefaultObservable {
        value: value,
        listeners: ~[]
    }
}
