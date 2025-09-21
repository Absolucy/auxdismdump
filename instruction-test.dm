/world/New()
	. = ..()
	call_ext("./target/i686-pc-windows-msvc/release/auxdismdump.dll", "auxtools_init")()
	do_the_dumping()
	del(src)

PROC_DUMP(len_test, list/meow) {
	return meow.len
}

PROC_DUMP(length_test, list/meow) {
	return meow.len
}
