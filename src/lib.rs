pub mod env;

use auxtools::{List, Proc, Value, hook, raw_types::procs::ProcId, runtime};
use dmasm::disassembler::DisassembleError;

#[hook("/proc/all_proc_names")]
pub fn all_proc_names() {
	let list = List::new();
	let mut idx = 0_u32;
	while let Some(proc) = Proc::from_id(ProcId(idx)) {
		idx += 1;
		let name = Value::from_string(proc.path)?;
		list.append(name);
	}
	Ok(Value::from(list))
}

#[hook("/proc/dump_proc")]
pub fn dump_proc(name: Value) {
	let name = name.as_string()?;
	let proc = Proc::find(&name).ok_or_else(|| runtime!("could not find {}", name))?;
	let mut env = env::AuxtoolsDisassembleEnv;
	let bytecode = unsafe { proc.bytecode() }.to_vec();
	let (nodes, _error) = dmasm::disassembler::disassemble(&bytecode, &mut env);
	let dism = dmasm::format_disassembly(&nodes, None);
	Value::from_string(dism)
}

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
