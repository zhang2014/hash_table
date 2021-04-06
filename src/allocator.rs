use anyhow::{bail, Result};
use std::alloc::realloc;
use std::cmp::min;

pub trait Allocator
{
    unsafe fn free(&self, ptr: *mut u8, size: usize);
    unsafe fn alloc(&self, size: usize, alignment: usize) -> Result<*mut u8>;
    unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, alignment: usize) -> Result<*mut u8>;
}

static MALLOC_MIN_ALIGNMENT: usize = 8;

// #[cfg(target_os != "linux")]
static MREMAP_MAYMOVE: usize = 1;

pub struct DefaultAllocator {
    mmap_threshold: usize
}

impl DefaultAllocator {
    unsafe fn get_page_size() -> usize {
        libc::sysconf(libc::_SC_PAGESIZE) as usize
    }

    // #[cfg(target_os != "linux")]
    unsafe fn mremap(alloc_old_mmap: *mut u8, new_size: usize, old_size: usize, flags: u32, mmap_prot: i32, mmap_flags: i32) -> Result<*mut u8> {
        if new_size < old_size {
            Result::Ok(alloc_old_mmap)
        } else {
            // if !(flags & MREMAP_MAYMOVE) {
            //     bail!("MAP_FAILED");
            // }
            let alloc_new_mmap = libc::mmap(std::ptr::null_mut(), new_size, mmap_prot, mmap_flags, -1, 0);

            if alloc_new_mmap == libc::MAP_FAILED {
                bail!("MAP_FAILED")
            }

            libc::memcpy(alloc_new_mmap, alloc_old_mmap as *mut _ as *mut libc::c_void, old_size);
            if libc::munmap(alloc_old_mmap as *mut _ as *mut libc::c_void, old_size) != 0 {
                libc::abort();
            }

            Result::Ok(alloc_new_mmap as *mut u8)
        }
    }
}

impl Default for DefaultAllocator {
    fn default() -> Self {
        DefaultAllocator {
            mmap_threshold: (64 * (1_usize << 20)) as usize
        }
    }
}

impl Allocator for DefaultAllocator {
    unsafe fn free(&self, ptr: *mut u8, size: usize) {

    }

    unsafe fn alloc(&self, size: usize, alignment: usize) -> Result<*mut u8> {
        if size >= self.mmap_threshold {
            let mmap_min_alignment = DefaultAllocator::get_page_size();

            if alignment > mmap_min_alignment {
                bail!("Exception");
            }

            /// MAP_PRIVATE | MAP_ANONYMOUS #if defined(OS_LINUX) | (mmap_populate ? MAP_POPULATE : 0)
            let alloc_mmap = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE,
                                        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0);
            if alloc_mmap == libc::MAP_FAILED {
                bail!("Error");
            }

            Result::Ok(alloc_mmap as *mut u8)
        } else {
            if alignment <= MALLOC_MIN_ALIGNMENT {
                let alloc_memory = libc::calloc(size, 1);

                if alloc_memory == std::ptr::null_mut() {
                    bail!("Allocator: Cannot malloc.")
                }

                Result::Ok(alloc_memory as *mut u8)
            } else {
                let layout = std::alloc::Layout::from_size_align_unchecked(size, alignment);
                Result::Ok(std::alloc::alloc_zeroed(layout))
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, alignment: usize) -> Result<*mut u8> {
        if old_size == new_size {
            Result::Ok(ptr)
        } else if old_size < self.mmap_threshold && new_size < self.mmap_threshold && alignment <= MALLOC_MIN_ALIGNMENT {
            let layout = std::alloc::Layout::from_size_align_unchecked(new_size, alignment);
            let realloc_memory = std::alloc::realloc(ptr, layout, new_size);

            if new_size > old_size {
                realloc_memory.offset(old_size as isize).write_bytes(0, (new_size - old_size) as usize);
            }

            Result::Ok(realloc_memory)
        } else if old_size >= self.mmap_threshold && new_size >= self.mmap_threshold {
            DefaultAllocator::mremap(ptr, new_size, old_size, MREMAP_MAYMOVE as u32, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS)
        } else {
            let alloc_memory = self.alloc(new_size, alignment);
            alloc_memory.map(|alloc_memory_ptr| {
                libc::memcpy(alloc_memory_ptr as *mut _ as *mut libc::c_void, ptr as *mut _ as *mut libc::c_void, min(old_size, new_size));
                self.free(ptr, old_size);
                alloc_memory_ptr
            })
        }
    }
}
