/datum/test
	var/meow

/datum/test/proc/thing()
	return null

/world/New()
	. = ..()
	call_ext("./target/i686-pc-windows-msvc/release/auxdismdump.dll", "auxtools_init")()
	fdel("dism.txt")
	file("dism.txt") << dump_disms()
	del(src)

/*
/world/Del()
	auxcleanup()
	return ..()
*/

/proc/_load_ext()
	return load_ext("cat.dll", "meow")

/proc/_call_ext(loaded)
	return call_ext(loaded)()

/proc/_alist()
	return alist(
		1 = "meow",
		"mrrp" = "mrrow",
	)

/proc/_spaceship(a, b)
	return a <=> b

/proc/_k_v_in_list(list/list)
	for(var/k, v in list)
		world.log << "[k]: [v]"

/datum/template
	var/width
	var/height

/proc/_block_turfs(atom/true_spawn, datum/template/template)
	return block(true_spawn.x, true_spawn.y, true_spawn.z, true_spawn.x + template.width, true_spawn.y + template.height, true_spawn.z)
