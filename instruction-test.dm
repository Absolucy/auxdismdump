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

/*
/proc/_isnan(x)
	return isnan(x)

/proc/_isinf(x)
	return isinf(x)

/proc/_fract(x)
	return fract(x)

/proc/_trunc(x)
	return trunc(x)

/proc/_ceil(x)
	return ceil(x)

/proc/_floor(x)
	return floor(x)

/proc/_refcount(x)
	return refcount(x)

/proc/_get_steps_to(a, b, c)
	return get_steps_to(a, b, c)

/proc/_noise_hash(list/nums)
	return noise_hash(nums)

/proc/_trimtext(x)
	return trimtext(x)

/proc/_ptr_ref(x)
	return &x

/proc/_ptr_ref_datum_var(datum/test/x)
	return &x.meow

/proc/_ptr_deref(x)
	*x = 1
*/

/*
/proc/_ftime(x)
	return ftime(x)

/proc/_equals_modulo(x, y)
	x %%= y
	return x

/proc/_x_pow_2(x)
	return x ** 2

/proc/_x_pow_neg_1(x)
	return x ** -1

/proc/_x_times_x(x)
	return x * x
*/

/proc/_double_colon(datum/test/x)
	return x::meow
