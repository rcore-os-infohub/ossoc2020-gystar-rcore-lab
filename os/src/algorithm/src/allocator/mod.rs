//! 负责分配 / 回收的数据结构

mod bitmap_vector_allocator;
mod segment_tree_allocator;
mod stacked_allocator;

///每次只分配一个单位，回收一个单位
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，返回被分配的单元的下标，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}

/// 向量分配器：固定容量，每次分配 / 回收一个带有对齐要求的连续向量
///
/// 参数和返回值中的 usize 表示第 n 个字节，不需要考虑起始地址
pub trait VectorAllocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配指定长度的空间，无法分配则返回 `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// 回收指定空间（一定是之前分配的）
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use bitmap_vector_allocator::BitmapVectorAllocator;
pub use segment_tree_allocator::SegTreeAllocator;
pub use stacked_allocator::StackedAllocator;

/// 默认使用的分配器
//单个物理页的分配器
pub type AllocatorImpl = SegTreeAllocator;
//
pub type VectorAllocatorImpl = BitmapVectorAllocator;
