/* Copyright 2014 Ben Boeckel
 *
 * Licensed under the MIT license. This file may not be copied, modified, or
 * distributed except according to those terms.
 */

//! The core Observable trait.

/// An observer for an Observable.
pub trait Observer<T> {
    /// Notify the observer that the value has changed.
    fn notify(&self, &T);
}

/// The main trait which exposes the interface for Observable types.
pub trait Observable<T> {
    /// Update the current value.
    fn update(&mut self, T);

    /// Register a function which reacts to a value update.
    fn watch(&mut self, ~Observer<T>) -> u64;

    /// Unregister a watcher.
    fn unwatch(&mut self, u64);
}
