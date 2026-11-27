#![no_std]

use core::ptr::null_mut;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    count: usize,
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {
            count: 0,
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
        }
    }
}

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = self.start;
        self.p_pos = self.end;
        self.count =0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        Err(allocator::AllocError::InvalidParam)
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        let align_start = (self.b_pos + layout.align() - 1) & !(layout.align() - 1);
        if self.p_pos - align_start < layout.size() {
            return Err(allocator::AllocError::NoMemory);
        }

        self.b_pos = align_start + layout.size();
        self.count += 1;

        return Ok(core::ptr::NonNull::new(align_start as *mut u8).unwrap());
    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        self.count -= 1;
        if self.count == 0 {
            self.b_pos = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        todo!()
    }

    fn used_bytes(&self) -> usize {
        todo!()
    }

    fn available_bytes(&self) -> usize {
        todo!()
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        let page_size = num_pages * Self::PAGE_SIZE;
        let align_end = self.p_pos & !(align_pow2 - 1);
        let align_start = align_end - page_size;
        if align_start < self.b_pos {
            return Err(allocator::AllocError::NoMemory);
        } else {
            self.p_pos = align_start;
            return Ok(align_start);
        }
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}
