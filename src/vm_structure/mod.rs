/// Author: Tenton Lien
/// Date: 2022/03/27

pub struct MemoryModel {
    method_area: MethodArea,
    stack: Stack,
    native_stack: NativeStack,
    program_counter: ProgramCounter,
    heap: Heap,
}

pub struct MethodArea {

}

pub struct Stack {
    size: u32
}

pub struct NativeStack {
    size: u32
}

pub struct ProgramCounter {

}

pub struct Heap {
    size: u64
}
