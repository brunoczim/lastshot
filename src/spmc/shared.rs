use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicUsize, Ordering::*},
    task,
};

const MESSAGE_BIT: u8 = 1 << 7;
const WAKER_BIT: u8 = 1 << 6;

pub enum NodeDataPtr<T> {
    Orphan(*mut OrphanData<T>),
    Subs(*mut SubsData<T>),
}

impl<T> NodeDataPtr<T> {
    pub fn null() -> Self {
        let null = null_mut();
        assert_eq!(null as usize & 1, 0);
        NodeDataPtr::Orphan(null)
    }

    pub fn is_null(self) -> bool {
        match self {
            NodeDataPtr::Orphan(ptr) => ptr.is_null(),
            NodeDataPtr::Subs(ptr) => ptr.is_null(),
        }
    }

    pub fn encode(self) -> usize {
        match self {
            NodeDataPtr::Orphan(ptr) => ptr as usize,
            NodeDataPtr::Subs(ptr) => ptr as usize | 1,
        }
    }

    pub fn decode(bits: usize) -> Self {
        if bits & 1 == 0 {
            NodeDataPtr::Orphan(bits as *mut _)
        } else {
            NodeDataPtr::Subs((bits & !1) as *mut _)
        }
    }
}

pub struct ReceiverSubs<T> {
    data: NonNull<SubsData<T>>,
}

impl<T> ReceiverSubs<T> {
    pub fn new() -> Self {
        let alloc = Box::new(SubsData {
            waker: UnsafeCell::new(MaybeUninit::uninit()),
            message: UnsafeCell::new(MaybeUninit::uninit()),
            count: AtomicU8::new(1),
        });
        Self { data: NonNull::from(Box::leak(alloc)) }
    }

    pub fn data(&self) -> NonNull<SubsData<T>> {
        self.data
    }

    pub fn data_ref(&self) -> &SubsData<T> {
        unsafe { self.data.as_ref() }
    }

    pub unsafe fn sender_subs(&self) -> SenderSubs<T> {
        self.data_ref().count.fetch_add(1, Acquire);
        SenderSubs { data: self.data }
    }
}

impl<T> Drop for ReceiverSubs<T> {
    fn drop(&mut self) {
        unsafe {
            if self.data_ref().drop_handle() {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}

pub struct SenderSubs<T> {
    data: NonNull<SubsData<T>>,
}

impl<T> SenderSubs<T> {
    pub fn data(&self) -> NonNull<SubsData<T>> {
        self.data
    }

    pub fn data_ref(&self) -> &SubsData<T> {
        unsafe { self.data.as_ref() }
    }
}

impl<T> Drop for SenderSubs<T> {
    fn drop(&mut self) {
        unsafe {
            if self.data_ref().drop_handle() {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}

#[repr(align(/* at least */ 2))]
pub struct OrphanData<T> {
    pub message: T,
}

#[repr(align(/* at least */ 2))]
pub struct SubsData<T> {
    pub waker: UnsafeCell<MaybeUninit<task::Waker>>,
    pub message: UnsafeCell<MaybeUninit<T>>,
    pub count: AtomicU8,
}

impl<T> SubsData<T> {
    unsafe fn drop_handle(&self) -> bool {
        let count = self.count.fetch_sub(1, Release);
        if count & MESSAGE_BIT != 0 {
            let ptr = self.message.get();
            (&mut *ptr).as_mut_ptr().drop_in_place();
        }
        if count & WAKER_BIT != 0 {
            let ptr = self.waker.get();
            (&mut *ptr).as_mut_ptr().drop_in_place();
        }
        count & 1 == 1
    }
}

pub struct Node<T> {
    pub data: AtomicUsize,
    pub next: AtomicPtr<Node<T>>,
    pub _marker: PhantomData<*mut T>,
}

pub struct Shared<T> {
    pub back: AtomicPtr<Node<T>>,
    pub sender: AtomicBool,
    pub receivers: AtomicUsize,
}

impl<T> Shared<T> {
    pub fn one_receiver() -> (Self, NonNull<Node<T>>) {
        let single_node = Box::new(Node {
            data: AtomicUsize::new(NodeDataPtr::<T>::null().encode()),
            next: AtomicPtr::default(),
            _marker: PhantomData,
        });
        let single_node_ptr = NonNull::from(Box::leak(single_node));
        let this = Self {
            back: AtomicPtr::new(single_node_ptr.as_ptr()),
            sender: AtomicBool::new(true),
            receivers: AtomicUsize::new(1),
        };

        (this, single_node_ptr)
    }
}
