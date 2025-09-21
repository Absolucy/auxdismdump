#define PROC_DUMP(name, args...) \
	/world/do_the_dumping() { \
		. = ..() ; \
		world.log << "dumping " + #name; \
		dump_proc_by_name(#name); \
	}; \
	/proc/##name(##args)


/world/proc/do_the_dumping()
	return

/proc/dump_proc_by_name(proc_name)
	world.log << "dump_proc_by_name([proc_name])"
	var/filename = "out/[proc_name].txt"
	fdel(filename)
	var/f = file(filename)
	f << "=== /proc/[proc_name] ===\n"
	f << dump_proc("/[proc_name]")