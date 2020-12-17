use std::fmt;

/// This struct is intended to prevent deadlocks that can occur when
/// acquiring a pooled db connection. A deadlock can occur whenever
/// a task attempts to acquire a pooled connection while already holding one.
/// Consider the case for a pooled connection and a pool size of 2:
///
/// fn() deadlock() {
///    let conn1 = acquire();
///    sleep(1000);
///    let conn2 = acquire();
/// }
///
/// If the above is called by two threads at the same time, both will
/// acquire conn1, but will not be able to acquire conn2 because
/// the pool is empty and neither thread can progress far enough to drop
/// conn1 leaving them both stuck.
///
/// The case above is relatively easy to monitor, but when the code
/// gets complex this is harder to detect. Consider for instance
/// if conn2 was acquired deep in the call stack through several
/// indirect function calls or even in other tasks.
///
/// TODO: Explain how this fixes the problem
pub struct DbAccess {
    _private: (),
}

impl fmt::Debug for DbAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbAccess {{}}")
    }
}

impl DbAccess {
    // TODO: Document invariants for calling.
    // (Eg: Top level task)
    pub unsafe fn new() -> Self {
        Self { _private: () }
    }

    pub fn wrap<T>(self, value: T) -> Accessed<T> {
        Accessed {
            access: self,
            value,
        }
    }

    pub fn unwrap<T>(access: Accessed<T>) -> Self {
        access.access
    }
}

pub struct Accessed<T> {
    access: DbAccess,
    value: T,
}

impl<T> fmt::Debug for Accessed<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> fmt::Display for Accessed<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::error::Error for Accessed<T> where T: std::error::Error {}

impl<T> std::ops::Deref for Accessed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
