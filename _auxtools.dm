/proc/auxtools_stack_trace(msg)
	CRASH(msg)

/proc/auxtools_expr_stub()
	CRASH("auxtools not loaded")

/proc/enable_debugging(mode, port)
	CRASH("auxtools not loaded")

/proc/dump_disms()
	CRASH("auxtools not loaded")

/proc/dump_proc(proc_name)
	CRASH("auxtools not loaded")

/proc/all_proc_names()
	CRASH("auxtools not loaded")

/world/proc/init_debugger()
	var/dll = GetConfig("env", "AUXTOOLS_DEBUG_DLL")
	world.log << "AUXTOOLS_DEBUG_DLL: [dll]"
	if(dll)
		call_ext(dll, "auxtools_init")()
		enable_debugging()

/world/proc/auxcleanup()
	var/debug_server = world.GetConfig("env", "AUXTOOLS_DEBUG_DLL")
	if(debug_server)
		call_ext(debug_server, "auxtools_shutdown")()
