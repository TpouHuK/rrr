dofile("values.lua")
-- SERVICE FUNCTIONS
-- === === === === === ===
function sleep(secs)
	r_sleep(secs*1000)
end

function setup_transport_line()
	set_line_args(H.line.trans)
end

function setup_accurate_line()
	set_line_args(H.line.accurate)
end

function setup_super_fast_line()
	set_line_args(H.line.fast)
end

function goto_point(point)
	setup_transport_line()
	r_goto_point(point)
	r_wait_till_arrival()
end

function s_goto_point(point)
	setup_super_fast_line()
	-- goto_point(point)
	r_goto_point(point)
	r_wait_till_arrival()
	-- set_defaults()
end

function rotate_to_point(point)
	r_rotate_to_point(point)
	r_wait_till_arrival()
end

function line_degrees(degrees)
	setup_accurate_line()
	r_ride_line_degrees(degrees)
	r_wait_till_arrival()
end

function set_rotate(pos)
	r_set_rotate(pos * -90, H.hand_speed)
end

function ride_degrees(degrees, speed)
	speed = speed or D_ride_degrees_speed
	r_set_mspeed(speed)

	r_ride_degrees(0, degrees)
	r_wait_till_arrival()
	set_defaults()
end

function ride_degrees_steer(steer, degrees, speed)
	speed = speed or D_ride_degrees_speed
	r_set_mspeed(speed)

	r_ride_degrees(steer, degrees)
	r_wait_till_arrival()

	set_defaults()
end

function black_white_cs()
	h, s, v = r_get_cs_hsv()
	if v > H.router_bw_limit_pt then
		return "white"
	else
		return "black"
	end
end

function has_value (tab, val)
	for index, value in ipairs(tab) do
		if value == val then
			return true
		end
	end

	return false
end

function get_color()
	h, s, v = r_get_cs_hsv()
	if (s < 50) or (v < 30) then
		return "none"
	end
	if (h < 30) or (h > 330)then
		return "red"
	end
	if (h < 90) then
		return "yellow"
	end
	if (h < 150) then
		return "green"
	end
	if (h < 270) then
		return "blue"
	end
	return "none"
end

function set_current_pos(POINT, ROTATION)
	CUR_POINT = POINT
	CUR_ANG = ROTATION
end
-- === === === === === ===

-- MAIN FUNCTIONS 
-- === === === === === ===
function start_line_ride()
	setup_transport_line()
	r_rolls()
	read_markers()
	--fake_read()
	r_wait_till_arrival()
	set_defaults()
	ride_degrees(100)
end

function start_degrees_ride()
	ride_degrees(H.start.up_dg)
	ride_degrees_steer(-100, H.start.left_dg)
	ride_degrees(H.start.diag_dg)
	ride_degrees_steer(100, H.start.right_dg)
end

function read_markers()
	current_colors = {}
	while (#current_colors ~= 4) do
		color = get_color()
		if color == "none" then
			goto continue
		end
		if has_value(current_colors, color) then
			goto continue
		end
		print(color)
		table.insert(current_colors, color)
		::continue::
	end
	current_colors_i = {}
	for k,v in ipairs(current_colors) do
		current_colors_i[v] = k
	end

	return current_colors
end

function get_color_rotation(color)
	a = current_colors_i[color]
	if     a == 1 then tp = 3
	elseif a == 2 then tp = 0
	elseif a == 3 then tp = 1
	elseif a == 4 then tp = 2
	end
	return tp
end

function set_lift(where)
	if where == "up" then degrees = 0
	elseif where == "take_wire"   then degrees = H.lift.take_wire
	elseif where == "put_wire"    then degrees = H.lift.put_wire
	elseif where == "take_router"  then degrees = H.lift.take_router
	elseif where == "shake_router" then degrees = H.lift.shake_router
	elseif where == "put_router"   then degrees = H.lift.put_router
	elseif where == "finish" then degrees = H.lift.finish
	elseif where == "pre_put" then degrees = H.lift.pre_put
	elseif where == "back_take" then degrees = H.lift.back_take
	end

	r_set_lift(-degrees, H.lift.speed)
end

function get_router(cub_n)
	if     cub_n == 1 then gt = "6"; rt = "58";
	elseif cub_n == 2 then gt = "7"; rt = "57";
	elseif cub_n == 3 then gt = "8"; rt = "56";
	elseif cub_n == 4 then gt = "1"; rt = "40";
	elseif cub_n == 5 then gt = "2"; rt = "41";
	elseif cub_n == 6 then gt = "3"; rt = "42";
	end
	if (CUR_ANG == 90) and (CUR_POINT == "8")then
		blue_magic = true
	end
	routers[cub_n] = "empty"

	goto_point(gt)
	r_set_rspeed(H.get_router.rotate_sd)
	rotate_to_point(rt)

	-- FIXME weird logic on turning to routers on first line
	-- omg idk how it is even working slightly
	-- so awfull
	if H.magic.on then
		if cub_n == 3 then
			if blue_magic then
				ride_degrees_steer(-100, H.magic.rgr, H.get_router.rotate_sd)
			end
		end
		if (cub_n == 1) then -- or (cub_n == 2) then -- or (cub_n == 3) then
			ride_degrees_steer(-100, H.magic.rgr, H.get_router.rotate_sd)
		end
	end

	set_defaults()
	set_rotate(0)
	ride_degrees(H.get_router.back_dg, -H.get_router.back_sd)
	set_lift("take_router")
	ride_degrees(H.get_router.back_dg+H.get_router.forward_dg, H.get_router.get_sd)
	set_lift("up")
	ride_degrees(H.get_router.forward_dg, -H.get_router.back_sd)
end

function put_router(color, side)
	if side == "long" then 
		if     color == "red"    then gt = "6";  rt = "22"; rp = 1;
		elseif color == "blue"   then gt = "8";  rt = "24"; rp = 1;
		elseif color == "yellow" then gt = "14"; rt = "15"; rp = 3;
		elseif color == "green"  then gt = "12"; rt = "17"; rp = 3;
		end
	elseif side == "short" then
		if     color == "red"    then gt = "5";  rt = "20"; rp = 2;
		elseif color == "blue"   then gt = "25";  rt = "26"; rp = 0;
		elseif color == "yellow" then gt = "33"; rt = "36"; rp = 0;
		elseif color == "green"  then gt = "10"; rt = "18"; rp = 2;
		end
	end
	tp = get_color_rotation(color)
	where = (tp - rp)
	if where < 0 then
		where = where + 4
	end

	goto_point(gt)
	rotate_to_point(rt)

	r_set_mspeed(H.put_router.speed_sd)
	if side == "long" then
		-- longride
		line_degrees(H.put_router.long_dg) -- forward
	elseif side == "short" then
		-- nothing
	end


	local LH = H.put_router

	set_rotate(where)
	line_degrees(LH.r0_dg)
	sleep(LH.wait_sc)
	set_lift("put_router")
	set_lift("up")
	set_rotate(0)
	ride_degrees(LH.r0_dg, -20)

	set_defaults()

	if side == "long" then
		ride_degrees(LH.long_dg, -20)
	end
end

function set_line_args(a)
	r_set_pid(a.pf_cf, a.df_cf, a.sf_sd, 
			a.ps_cf, a.ds_cf, a.ss_sd, 
			a.lx_cf, a.top_pt, a.bot_pt)
	r_set_pidb(a.pf_cf, a.df_cf, a.sf_sd, 
			a.ps_cf, a.ds_cf, a.ss_sd, 
			a.lx_cf, a.top_pt, a.bot_pt)
end

function set_defaults()
	r_set_ldegrees(H.line.udegrees_dg)
	r_set_rspeed(H.speed.rotate_sd)
	r_set_mspeed(H.speed.macro_sd)
	r_set_white(H.colors.white_pt)
	r_set_middle_grey(H.colors.grey_pt)
	r_set_black(H.colors.black_pt)
	D_ride_degrees_speed = H.speed.degrees_sd
end


function put_wire(num)
	-- |field_png|
	if num == 1 then
		goto_point("7")
		rotate_to_point("23")
	elseif num == 2 then
		goto_point("38")
		rotate_to_point("16")
	end

	-- |forward_mov|
	local speed = H.put_wire.forward_sd
	line_degrees(H.put_wire.line_dg)
	ride_degrees(H.put_wire.forward_dg, speed)

	-- |lift__mov|
	set_lift("put_wire")
	set_lift("up")

	-- |backward_mov|
	if num == 2 then
		speed = -H.put_wire.nothing_sd
	elseif num == 1 then
		speed = -H.put_wire.cubics_sd
	end
	local degrees = H.put_wire.line_dg + H.put_wire.forward_dg - H.put_wire.overshoot_dg

	ride_degrees(degrees, speed) 
end

function get_wire(wire_num)
	-- |field_png|
	if wire_num == 1 then
		goto_point("44")
		rotate_to_point("39")
	elseif wire_num == 2 then
		goto_point("45")
		rotate_to_point("43")
	end

	-- |forward_mov|
	line_degrees(H.get_wire.forward_dg)

	-- |lift_mov|
	set_lift("take_wire")
	sleep(H.get_wire.wait_sc)
	set_lift("up")

	-- |back_mov|
	ride_degrees(H.get_wire.forward_dg, -H.get_wire.back_sd) 
end

function check_router(cub_n)
	if     cub_n == 1 then gt = "6"; rt = "7";
	elseif cub_n == 2 then gt = "7"; rt = "8";
	elseif cub_n == 3 then gt = "8"; rt = "37";
	elseif cub_n == 4 then gt = "1"; rt = "2";
	elseif cub_n == 5 then gt = "2"; rt = "3";
	elseif cub_n == 6 then gt = "3"; rt = "45";
	end

	goto_point(gt)
	rotate_to_point(rt)

	return black_white_cs()
end


function fake_read()
	current_colors_i = {red=1, blue=2, green=3, yellow=4}
end

function start()
	start_degrees_ride()
	start_line_ride()
	set_current_pos("13", 0)
end

function finish()
	-- goto_point("12")
	-- s_goto_point("14")
	goto_point("32")
	rotate_to_point(33)
	-- set_lift("finish")
	-- ride_degrees_steer(100, FINISH_DEGREES_ROTATE)
	-- ride_degrees(FINISH_DEGREES, -100)
end

function print_routers()
	print(
		routers[1],
		routers[2],
		routers[3],
		routers[4],
		routers[5],
		routers[6]
	)
end

function print_smile()
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
	print(" $$$$$$                     $$$$$$  ")
	print(" $$$$$$                     $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$                     $$$$$$  ")
	print(" $$$$$$                     $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$       $$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$$$$$$$$$$$$$  $$$$$$  ")
	print(" $$$$$$  $$$$$$$$$$$$$$$$$  $$$$$$  ")
	print(" $$$$$$                     $$$$$$  ")
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
	print(" $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$  ")
end

function get_available_router(order)
	for _,v in ipairs(order) do
		if routers[v] == "black" then
			routers[v] = "empty"
			get_router(v)
			break
		end
	end
end
function white_count()
	local wc = 0
	for _, v in ipairs(routers) do
		if v == "white" then
			wc = wc + 1
		end
	end
	return wc
end

function main2()
	print_smile()
	set_defaults()
	start()

	--goto_point("11")
	s_goto_point("21")

	routers = {
		"unknown", 
		"unknown",
		"unknown",
		"unknown",
		"unknown",
		"unknown",
	}

	local router_taken = false
	local back = false
	local everything_known = false

	for i=4,6 do
		routers[i] = check_router(i)
		print(i, routers[i])
		if (routers[i] == "black") and (not router_taken) then
			routers[i] = "empty"
			if not back then
				back = true
				back_take()
			else
				router_taken = true
				get_router(i)
			end
		end
	end
	get_wire(2)

	if not router_taken then
		routers[1] = "black"
		routers[2] = "black"
		routers[3] = "black"
		
		put_wire(1)

		goto_point(21)
		get_router(1)
		put_router("red", "long")
		get_router(3)
		put_router("blue", "long")
		-- get_router(1)
	else
		put_router("blue", "long")
		put_wire(1)
		
		goto_point(21)
		routers[1] = check_router(1)
		if routers[1] == "black" then
			get_router(1)
			put_router("red", "long")
			if white_count() == 0 then
				routers[2] = "white"
				routers[3] = "white"
				-- get_available_router({1,2,3,4,5,6})
			elseif white_count() == 1 then
				routers[2] = check_router(2)
				if routers[2] == "black" then
					routers[3] = "white"
					-- get_router(2)
				else
					routers[3] = "black"
					-- get_router(3)
				end
			end
		else
			routers[2] = check_router(2)
			if routers[2] == "black" then
				get_router(2)
				put_router("red", "long")

				if white_count() == 2 then
					routers[3] = "black"
				else
					routers[3] = "white"
				end
			else
				routers[3] = "black"
			end
		end
		
		
	end
	get_available_router({1,2,3,4,5,6})
	get_wire(1)
	put_router("green", "short")
	put_wire(2)
	goto_point("14")
	sleep(1)
	swap_router()
	put_router("yellow", "long")
	finish()
end

function main()
	print_smile()
	set_defaults()
	start()

	s_goto_point("21")
	routers = {}
	for i=1, 6 do
		routers[i] = "unknown"
	end

	is_router_taken = false
	is_first = false
	for i=4,6 do
		routers[i] = check_router(i)
		print(i, routers[i])
		if (routers[i] == "black") and (not is_router_taken) then
			routers[i] = "empty"
			if not is_first then
				is_first = true
				back_take()
			else
				is_router_taken = true
				get_router(i)
			end
		end
	end

	get_wire(2)
	goto_point("37")
	s_goto_point("34")
	put_router("yellow", "short")
	put_wire(2)
	if H.magic.on then
		r_set_ldegrees(H.magic.dsc)
		goto_point("11")
		set_defaults()
	end
	goto_point("11")
	s_goto_point("21")

	get_wire(1)

	routers[1] = check_router(1)
	routers[2] = check_router(2)
	put_wire(1)

	-- == -- ==
	white_count = 0
	for _, v in ipairs(routers) do
		if v == "white" then
			white_count = white_count + 1
		end
	end

	if white_count >= 2 then
		routers[3] = "black"
	else
		routers[3] = "white"
	end
	-- == -- ==

	-- blue
	for _,v in ipairs{2,3,1,4,5,6} do
		if routers[v] == "black" then
			routers[v] = "empty"
			get_router(v)
			break
		end
	end
	put_router("blue", "long")


	-- red 
	for _,v in ipairs{3,6,2,1,4,5} do
		if routers[v] == "black" then
			routers[v] = "empty"
			get_router(v)
			break
		end
	end

	local white_first_row_count = 0
	for _,v in ipairs{1,2,3} do
		if routers[v] == "white" then
			white_first_row_count = white_first_row_count + 1	
		end
	end


	if white_first_row_count == 2 then
		put_router("red", "short")
	else
		put_router("red", "long")
	end


	-- green 
	for _,v in ipairs{1,2,3,4,5,6} do
		if routers[v] == "black" then
			routers[v] = "empty"
			get_router(v)
			break
		end
	end

	-- goto_point("21")
	-- s_goto_point("9")
	goto_point("12")
	swap_router()
	put_router("green", "long")
	finish()
end

function back_take()
	local forward_degrees = 110
	set_lift("back_take")
	ride_degrees_steer(0, forward_degrees, 20) -- forward
	ride_degrees_steer(50, 200, -30) -- take
	set_lift("up")
	ride_degrees_steer(50, 200, 30) -- back rotate

	if not H.magic.back_take then
		ride_degrees_steer(0, forward_degrees, -20) -- backward
	end
end

function swap_router()
	set_lift("back_take")
	ride_degrees_steer(50, 50, 20)
	set_lift("up")
	ride_degrees_steer(50, 20, -20)
	ride_degrees_steer(-50, 790, 40) --megaturn
	set_lift("take_router")
	ride_degrees_steer(0, 150 + 10, 20)
	set_lift("up")
	ride_degrees_steer(-100, 255, 20)
	line_degrees(120)
end

function test_backtake()
	set_defaults()
	back_take()
end

function test_backfront()
	set_defaults()
	swap_router()
end

--test_backfront()
main2()
