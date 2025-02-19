use super::kernel;
use crate::hal::cpu::GlobalPointer;
use crate::hal::Register;

type Handle = u32;

/// A handle to a [BIOS thread](http://problemkaputt.de/psx-spx.htm#biosthreadfunctions).
#[derive(Debug)]
pub struct Thread {
    handle: Handle,
}

impl Thread {
    /// A handle to the initial thread created by the BIOS.
    pub const INIT: Thread = Thread::new(0xFF00_0000);

    const fn new(handle: Handle) -> Self {
        Thread { handle }
    }

    /// Creates a new thread with the program counter initialized to `func`, the
    /// stack and frame pointers initialized to `sp`. If `gp` is `Some` it's
    /// used to initialize the thread's global pointer, otherwise the global
    /// pointer will be taken from the current thread. Returns `None` if the
    /// BIOS cannot find a free thread control block.
    pub fn open<R>(func: fn() -> R, sp: u32, gp: Option<GlobalPointer>) -> Option<Self> {
        let gp = gp.unwrap_or(GlobalPointer::load());
        let handle = unsafe { kernel::open_thread(func as u32, sp, gp.bits()) };
        match handle {
            0xFF00_0000..=0xFFFF_FFFE => Some(Self::new(handle)),
            0xFFFF_FFFF => None,
            _ => illegal!("Received unknown error code from BIOS in `kernel::open_thread`"),
        }
    }

    /// Closes the given thread and destroys its handle. Note that if the
    /// current thread is closed, its execution is not terminated.
    pub fn close(self) {
        unsafe {
            kernel::close_thread(self.handle);
        }
    }

    /// Pauses the current thread and activates a new thread. Threads aren't
    /// automatically scheduled so all other threads will remain paused until
    /// the new thread yields by calling this function again on another thread.
    pub fn change(&self) {
        unsafe {
            kernel::change_thread(self.handle);
        }
    }

    pub fn spawn<R>(func: fn() -> R, sp: u32, gp: Option<GlobalPointer>) -> Option<Self> {
        let t = Thread::open(func, sp, gp);
        t.as_ref().map(|t| t.change());
        t
    }
}

#[cfg(test)]
mod tests {
    use super::Thread;

    #[test_case]
    fn open_and_close() {
        let new_thread = || Thread::open(|| (), 0, None);
        // Default max number of TCBs is 4
        // Since we don't provide a SYSTEM.CNF here, we can only create 3 new threads
        let t0 = new_thread();
        let t1 = new_thread();
        let t2 = new_thread();
        let t3 = new_thread();
        let t4 = new_thread();
        match (t0, t1, t2) {
            (Some(t0), Some(t1), Some(t2)) => {
                assert!(t0.handle == 0xFF00_0001);
                assert!(t1.handle == 0xFF00_0002);
                assert!(t2.handle == 0xFF00_0003);
                t0.close();
                t1.close();
                t2.close();
            },
            _ => panic!("Could not open the expected number of threads"),
        }
        assert!(t3.is_none());
        assert!(t4.is_none());
    }

    #[test_case]
    fn change_thread() {
        static mut X: u32 = 0;
        let t = Thread::open(
            || {
                unsafe {
                    X += 1;
                }
                Thread::INIT.change();
            },
            // Set `sp` to the end of the scratchpad/D-cache to avoid overlap between stacks
            0x9F80_0000 + 1024,
            None,
        )
        .expect("Unable to open thread");
        assert!(unsafe { X == 0 });
        t.change();
        assert!(unsafe { X == 1 });
        t.close();
    }
}
