use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

#[repr(align(64))]
struct PaddedAtomic(AtomicUsize);

pub struct RingBuffer<T, const N: usize> {
    data: [UnsafeCell<MaybeUninit<T>>; N],
    head: PaddedAtomic,
    tail: PaddedAtomic,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        RingBuffer {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            head: PaddedAtomic(AtomicUsize::new(0)),
            tail: PaddedAtomic(AtomicUsize::new(0)),
        }
    }

    pub fn push(&self, value: T) -> bool {
        let t = self.tail.0.load(Ordering::Relaxed);
        let next_t = (t + 1) & (N - 1);

        if next_t == self.head.0.load(Ordering::Acquire) {
            return false;
        }

        unsafe { (*self.data[t].get()).write(value); }
        self.tail.0.store(next_t, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<T> {
        let h = self.head.0.load(Ordering::Relaxed);

        if h == self.tail.0.load(Ordering::Acquire) {
            return None;
        }

        let value = unsafe { (*self.data[h].get()).assume_init_read() };
        self.head.0.store((h + 1) & (N - 1), Ordering::Release);
        Some(value)
    }
}