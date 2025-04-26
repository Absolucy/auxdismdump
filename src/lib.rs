pub mod env;

use auxtools::{Proc, Value, hook, raw_types::procs::ProcId};
use dmasm::disassembler::DisassembleError;

#[hook("/proc/dump_disms")]
pub fn dump_disms() {
	let mut idx = 0_u32;
	let mut names = Vec::<String>::new();
	while let Some(proc) = Proc::from_id(ProcId(idx)) {
		idx += 1;
		let name = proc.path.trim();
		if name.is_empty()
		/* || !name.starts_with("/_") */
		{
			// uncomment that check if you only wanna disassemble global procs that start
			// with _
			continue;
		}
		let mut env = env::AuxtoolsDisassembleEnv;
		let bytecode = unsafe { proc.bytecode() }.to_vec();
		let (nodes, error) = dmasm::disassembler::disassemble(&bytecode, &mut env);
		let dism = dmasm::format_disassembly(&nodes, None);
		let result = match error {
			Some(DisassembleError::UnknownOpcode { offset, opcode }) => {
				format!(
					"--- {name} ---\n{dism}\nUnknown Opcode {opcode:#04x} starting at offset \
					 {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(DisassembleError::UnknownAccessModifier { offset, value }) => {
				format!(
					"--- {name} --- \n{dism}\nUnknown Access Modifier {value:#04x} starting at \
					 offset {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(DisassembleError::UnknownFieldAccessModifier { offset, value }) => {
				format!(
					"--- {name} --- \n{dism}\nUnknown Field Access Modifier {value:#04x} starting \
					 at offset {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(DisassembleError::UnknownIsInOperand { offset, value }) => {
				format!(
					"--- {name} --- \n{dism}\nUnknown Is In Operand {value:#04x} starting at \
					 offset {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(DisassembleError::UnknownValue { offset, tag }) => {
				format!(
					"--- {name} --- \n{dism}\nUnknown Value (tag: {tag:#04x}) starting at offset \
					 {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(DisassembleError::UnknownTypeFilter { offset, value }) => {
				format!(
					"--- {name} --- \n{dism}\nUnknown Type Filter {value:#04x} starting at offset \
					 {offset}\n{bytecode:#04x?}",
					bytecode = &bytecode[offset as usize..]
				)
			}

			Some(error) => {
				format!("--- {name} ---\n{dism}\n\tError: {error:?}")
			}

			None => {
				continue;
			}
		};
		names.push(result);
	}
	Value::from_string(names.join("\n\n"))
}
