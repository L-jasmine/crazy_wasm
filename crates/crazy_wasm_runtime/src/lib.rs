mod types;

pub struct Runtime {
    stack: Vec<u64>,
    stack_pos: usize,
    globals: Vec<WasmVal>,
    func: Vec<fn(&mut Runtime)>,
}

#[derive(Debug, Clone, Copy)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy)]
pub enum WasmVal {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl WasmVal {
    pub fn to_type(&self) -> ValType {
        match self {
            WasmVal::I32(_) => ValType::I32,
            WasmVal::I64(_) => ValType::I64,
            WasmVal::F32(_) => ValType::F32,
            WasmVal::F64(_) => ValType::F64,
        }
    }
}
