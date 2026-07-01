#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Represents a free block of memory in our physical RAM.
struct ListNode {
    size: usize,
    next: *mut ListNode,
}

/// The Vault: Our autonomous Free-List memory allocator.
pub struct SentinelAllocator {
    head: ListNode,
}

impl SentinelAllocator {
    /// Initializes an empty Vault.
    pub const fn new() -> Self {
        Self {
            head: ListNode {
                size: 0,
                next: null_mut(),
            },
        }
    }

    /// Injects the raw physical memory map provided by UEFI into The Vault.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // Aligns the raw RAM into our operational structure.
        let mut node = ListNode { size, next: self.head.next };
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = node_ptr;
    }
}

unsafe impl GlobalAlloc for SentinelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let mut current = &self.head as *const ListNode as *mut ListNode;
        let mut best_fit_prev: *mut ListNode = null_mut();
        let mut best_fit: *mut ListNode = null_mut();
        let mut min_waste = usize::MAX;

        // --- THE HUNT: Best-Fit Traversal ---
        // The OS scans every available block of RAM to find the optimal sector.
        while let Some(ref mut node) = (*current).next.as_mut() {
            let node_addr = *node as *mut ListNode as usize;
            
            // Calculate alignment offset
            let alloc_start = (node_addr + core::mem::align_of::<ListNode>() + align - 1) & !(align - 1);
            let alloc_end = alloc_start.saturating_add(size);
            let node_end = node_addr + node.size;

            if alloc_end <= node_end {
                let waste = node.size - size;
                
                // Identify the block that wastes the absolute minimum amount of space.
                if waste < min_waste {
                    min_waste = waste;
                    best_fit_prev = current;
                    best_fit = *node;
                }
            }
            current = *node;
        }

        // --- MUTATION ---
        // If a perfect sector is found, we extract it and adjust the list.
        if !best_fit.is_null() {
            let node = &mut *best_fit;
            (*best_fit_prev).next = node.next;
            
            // Return the raw pointer for the Sentinel to consume.
            return best_fit as *mut u8;
        }

        // If memory is exhausted, standard OS architectures crash. 
        // We will eventually trigger an aggressive Sentinel purge here.
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // When a threat is neutralized or a process dies, the memory is reclaimed and re-linked.
        let mut mutable_self = self as *const _ as *mut SentinelAllocator;
        (*mutable_self).add_free_region(ptr as usize, layout.size());
    }
}

// Registers The Vault as the absolute authority over system memory.
#[global_allocator]
static ALLOCATOR: SentinelAllocator = SentinelAllocator::new();