dofile("values.lua")
-- SERVICE FUNCTIONS
-- === === === === === ===
function sleep(secs)
	r_sleep(secs*1000)
end

function goto_point(point)
	r_goto_point(point)
	r_wait_till_arrival()
end

function s_goto_point(point)
	r_set_lspeed(FAST_LINE_SPEED)
	goto_point(point)
	set_defaults()
end

function rotate_to_point(point)
	r_rotate_to_point(point)
	r_wait_till_arrival()
end

function line_degrees(degrees, speed)
	if speed then
		r_set_lspeed(speed)
	end
	r_ride_line_degrees(degrees)
	r_wait_till_arrival()
	if speed then
		set_defaults()
	end
end

function set_rotate(pos)
	r_set_rotate(pos * -90, ROTATE_LIFT_SPEED)
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
	if v > BLACK_WHITE_CS_LIMIT then
		return 'white'
	else
		return 'black'
	end
end


function ride_joystick_degrees()
	r_joystick_write()
	r_wait_till_arrival()
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
	r_set_lspeed(FIRST_LINE_SPEED)
	r_rolls()
	--read_markers()
	fake_read()
	r_wait_till_arrival()
	set_defaults()
	ride_degrees(100)
end

function start_degrees_ride()
	ride_degrees(START_DEGREES_UP)
	ride_degrees_steer(-100, START_DEGREES_LEFT)
	ride_degrees(START_DEGREES_DIAG)
	ride_degrees_steer(100, START_DEGREES_RIGHT)
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
	elseif where == "take_wire"   then degrees = LIFT_TAKE_WIRE
	elseif where == "put_wire"    then degrees = LIFT_PUT_WIRE
	elseif where == "take_router"  then degrees = LIFT_TAKE_ROUTER
	elseif where == "shake_router" then degrees = LIFT_SHAKE_ROUTER
	elseif where == "put_router"   then degrees = LIFT_PUT_ROUTER
	elseif where == "finish" then degrees = LIFT_FINISH
	elseif where == "pre_put" then degrees = LIFT_PRE_PUT
	end

	r_set_lift(-degrees, LIFT_SPEED)
end

function shake_lift()
	local up = LIFT_PUT_WIRE + WIRE_SHAKE
	local down = LIFT_PUT_WIRE - WIRE_SHAKE
	for i = 1, 3 do
		r_set_lift(-up, LIFT_SPEED)
		r_set_lift(-down, LIFT_SPEED)
	end
	set_lift("put_wire")
end

function shake_lift_get()
	local up = LIFT_TAKE_WIRE + WIRE_SHAKE
	local down = LIFT_TAKE_WIRE - WIRE_SHAKE
	for i = 1, 3 do
		r_set_lift(-up, LIFT_SPEED)
		r_set_lift(-down, LIFT_SPEED)
	end
	set_lift("take_wire")
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

	goto_point(gt)
	r_set_rspeed(ROUTER_ROTATE_SPEED)
	rotate_to_point(rt)
	if MAGIC_ON then
		if cub_n == 3 then
			if blue_magic then
				ride_degrees_steer(-100, MAGIC_RGR, ROUTER_ROTATE_SPEED)
			end
		end
		if (cub_n == 1) then -- or (cub_n == 2) then -- or (cub_n == 3) then
			ride_degrees_steer(-100, MAGIC_RGR, ROUTER_ROTATE_SPEED)
		end
	end
	set_defaults()

	set_rotate(0)
	
	ride_degrees(ROUTER_DEGREES_BACK, -ROUTER_BACK_SPEED)
	set_lift("take_router")
	ride_degrees(ROUTER_DEGREES_BACK+ROUTER_DEGREES_FORWARD, ROUTER_GET_SPEED)
	set_lift("up")
	ride_degrees(ROUTER_DEGREES_FORWARD, -ROUTER_BACK_SPEED)
end

function pr_long(color)
end

function pr_short(color)
end

function shake()
	do return end
	--r_set_mspeed(60)

	set_lift("shake_router")
	for i=1,1 do
		ride_degrees_steer(0, FORWARD_SHAKE_DEGREES, SHAKE_SPEED)
		ride_degrees_steer(0, FORWARD_SHAKE_DEGREES, -SHAKE_SPEED)

		ride_degrees_steer(-100, SIDE_SHAKE_DEGREES, SHAKE_SPEED)
		ride_degrees_steer(-100, SIDE_SHAKE_DEGREES*2, -SHAKE_SPEED)
		ride_degrees_steer(-100, SIDE_SHAKE_DEGREES, SHAKE_SPEED)
	end
	for i =1, 3 do
		r_set_lift(-LIFT_SHAKE_ROUTER+SHAKE_ROUTER_LIFT_DELTA, LIFT_SPEED)
		r_set_lift(-LIFT_SHAKE_ROUTER-SHAKE_ROUTER_LIFT_DELTA, LIFT_SPEED)
	end
	--set_defaults()
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
	set_lift("pre_put")
	set_rotate(0)

	r_set_lspeed(PUT_ROUTER_SPEED)
	r_set_mspeed(PUT_ROUTER_SPEED)
	if side == "long" then
		-- longride
		line_degrees(LONG_DEGREES) -- forward
		
	elseif side == "short" then
		-- nothing
	end


	local SLEEP_TIME = ROUTER_SLEEP_TIME
	if where == 0 then
		set_rotate(0)
		line_degrees(ROUTER_0) -- forward

		sleep(SLEEP_TIME)
		set_lift("shake_router")
		shake()
		set_lift("put_router")

		ride_degrees(ROUTER_0, -20) -- return
		set_lift("up")

	elseif where == 1 then
		set_rotate(1)
		line_degrees(ROUTER_1) -- forward

		--ride_degrees_steer(-20, ROUTER_1S) -- steering right

		sleep(SLEEP_TIME)
		set_lift("shake_router")
		shake()
		set_lift("put_router")
		set_rotate(0)
		set_rotate(3)
		set_lift("up")

		--ride_degrees_steer(-20, ROUTER_1S, -20) -- steering back right
		ride_degrees(ROUTER_1, -20) 

		set_lift("up")
		set_rotate(0)

	elseif where == 3 then
		set_rotate(3)
		line_degrees(ROUTER_3) -- forward

		--ride_degrees_steer(20, ROUTER_3S) -- steering right

		sleep(SLEEP_TIME)
		set_lift("shake_router")
		shake()
		set_lift("put_router")
		set_rotate(0)
		set_rotate(1)
		set_lift("up")

		--ride_degrees_steer(20, ROUTER_3S, -20) -- steering back right
		ride_degrees(ROUTER_3, -20) 

		set_lift("up")
		set_rotate(0)

	elseif where == 2 then
		set_rotate(2) 
		line_degrees(ROUTER_2) -- forward

		sleep(SLEEP_TIME)
		set_lift("shake_router")
		shake()
		set_lift("put_router")

		set_rotate(0)
		set_lift("up")
		ride_degrees(ROUTER_2, -20) -- return
	end
	set_defaults()

	if side == "long" then
		-- longride
		ride_degrees(LONG_DEGREES - BACK_LONG_NOTRIDE, -20) -- return
		
	elseif side == "short" then
		-- nothing
	end

end

function set_defaults()
	r_set_pid(P_M, I_M, D_M)
	r_set_pidb(P_S, I_S, D_S)

	r_set_lspeed(LINE_SPEED)
	r_set_ldegrees(LINE_UDEGREES)

	r_set_rspeed(ROTATE_SPEED)
	r_set_mspeed(MACRO_SPEED)

	r_set_white(WHITE)
	r_set_middle_grey(MIDLE_GREY)
	r_set_black(BLACK)


	D_ride_degrees_speed = RIDE_DEGREES_SPEED
end


function put_wire(num)
	if num == 1 then
		goto_point("7")
		rotate_to_point("23")
	elseif num == 2 then
		goto_point("38")
		rotate_to_point("16")
	end

	line_degrees(WIRE_PUT_LINE_DEGREES, WIRE_PUT_SPEED_F)
	ride_degrees(WIRE_PUT_DEGREES, WIRE_PUT_SPEED_F)
	set_lift("put_wire")
	-- shake_lift()
	set_lift("up")
	if num == 2 then
		ride_degrees(WIRE_PUT_LINE_DEGREES + WIRE_PUT_DEGREES - WIRE_PUT_OVERSHOOT_COMP, -WIRE_PUT_SPEED_NOTHING) 
	elseif num == 1 then
		ride_degrees(WIRE_PUT_LINE_DEGREES + WIRE_PUT_DEGREES - WIRE_PUT_OVERSHOOT_COMP, -WIRE_PUT_SPEED_CUBICS) 
	end
end

function get_wire(wire_num)
	if wire_num == 1 then
		goto_point("44")
		rotate_to_point("39")
	elseif wire_num == 2 then
		goto_point("45")
		rotate_to_point("43")
	end

	line_degrees(WIRE_GET_DEGREES, WIRE_GET_U_SPEED)
	set_lift("take_wire")
	-- shake_lift_get()
	-- sleep(WIRE_GET_SLEEP)
	set_lift("up")
	ride_degrees(WIRE_GET_DEGREES, -WIRE_GET_D_SPEED) 
end


function test_line()
	while true do
		goto_point("0")
		goto_point("4")
		goto_point("32")
		goto_point("11")
		goto_point("21")
		goto_point("37")
		goto_point("4")
	end
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
	goto_point("12")
	s_goto_point("14")
	goto_point("32")
	rotate_to_point(33)
	set_lift("finish")
	ride_degrees_steer(100, FINISH_DEGREES_ROTATE)
	ride_degrees(FINISH_DEGREES, -100)
	--set_lift("up")
end

function testline()
	goto_point("11")
	goto_point("32")
	goto_point("37")
	goto_point("21")

	goto_point("11")
end

function fulltest_run()
	get_router(1)
	put_router("red", "long")
	get_router(2)
	put_router("blue", "long")
	get_router(4)
	put_router("green", "short")
	get_router(3)
	put_router("yellow", "short")
	sleep(999)
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
-- === === === === === ===
set_defaults()

-- ==TESTING ZONE== --
--set_current_pos("8", 0)
--get_router(1)
--put_router("red", "long")
--sleep(999)

--set_current_pos("21", 0)
--fake_read()
--fulltest_run()
-- ==TESTING ZONE== --

for i=1, 20 do
	print("REEEEEEEEEEEEEEEEEEEEEEEEadddy")
end
r_wait_center()

-- fake_read()
--set_current_pos("8", 0)
--get_router(1)
--put_router("red", "long")
--sleep(999)


start()
-- get_wire(1)
-- put_wire(1)
-- get_wire(2)
-- put_wire(2)
-- sleep(888)


s_goto_point("21")
routers = {}
for i=1, 6 do
	routers[i] = "unknown"
end

is_router_taken = false
for i=4,6 do
	routers[i] = check_router(i)
	print(i, routers[i])
	if (routers[i] == "black") and (not is_router_taken) then
		routers[i] = "empty"
		get_router(i)
		is_router_taken = true
	end
end

get_wire(2)
goto_point("37")
s_goto_point("34")
put_router("yellow", "short")
put_wire(2)
if MAGIC_ON then
	r_set_ldegrees(MAGIC_DSC)
	goto_point("11")
	set_defaults()
end
goto_point("11")
s_goto_point("21")

get_wire(1)

routers[1] = check_router(1)
routers[2] = check_router(2)
put_wire(1)

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

print_routers()
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
put_router("green", "short")
finish()
