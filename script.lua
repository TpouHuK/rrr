function goto_point(point)
	r_goto_point(point)
	r_wait_till_arrival()
end

function rotate_to_point(point)
	r_rotate_to_point(point)
	r_wait_till_arrival()
end

function line_degrees(degrees)
	r_ride_line_degrees(degrees)
	r_wait_till_arrival()
end

function set_rotate(pos)
	r_set_rotate(pos * -90)
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

function start_line_ride()
	r_rolls()
	read_markers()
	r_wait_till_arrival()
	ride_degrees(100)
end

function start_degrees_ride()
	ride_degrees(320)
	ride_degrees_steer(-100, 110)
	ride_degrees(90)
	ride_degrees_steer(100, 110)
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
	elseif where == "take_cable"   then degrees = 570
	elseif where == "put_cable"    then degrees = 460
	elseif where == "take_router"  then degrees = 400
	elseif where == "shake_router" then degrees = 150
	elseif where == "put_router"   then degrees = 500
	end

	r_set_lift(-degrees)
end

function get_router(cub_n)
	if     cub_n == 1 then gt = "6"; rt = "58";
	elseif cub_n == 2 then gt = "7"; rt = "57";
	elseif cub_n == 3 then gt = "8"; rt = "56";
	elseif cub_n == 4 then gt = "1"; rt = "40";
	elseif cub_n == 5 then gt = "2"; rt = "41";
	elseif cub_n == 6 then gt = "3"; rt = "42";
	end

	goto_point(gt)
	rotate_to_point(rt)

	set_rotate(0)
	ride_degrees(100, -10)
	set_lift("take_router")
	ride_degrees(100)
	set_lift("up")
end

function pr_long(color)
end

function pr_short(color)
end

function shake()
	local DEGREES = 20
	ride_degrees_steer(-100, DEGREES, 10)
	ride_degrees_steer(-100, DEGREES*2, -10)
	ride_degrees_steer(-100, DEGREES, 10)
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
	set_lift("up")
	set_rotate(0)

	if side == "long" then
		-- longride
	elseif side == "short" then
		if where == 0 then
			DEGREES = 160
			set_rotate(0)
			line_degrees(DEGREES) -- forward

			sleep(1)
			set_lift("shake_router")
			shake()
			set_lift("put_router")

			ride_degrees(DEGREES, -20) -- return
			set_lift("up")

		elseif where == 1 then
			DEGREES = 130
			set_rotate(1)
			line_degrees(DEGREES) -- forward

			ride_degrees_steer(-20, 50) -- steering right

			sleep(1)
			set_lift("shake_router")
			shake()
			set_lift("put_router")
			set_rotate(0)
			set_rotate(3)
			set_lift("up")

			ride_degrees_steer(-20, 50, -20) -- steering back right
			ride_degrees(DEGREES, -20) 

			set_lift("up")
			set_rotate(0)

		elseif where == 3 then
			DEGREES = 150
			set_rotate(3)
			line_degrees(DEGREES) -- forward

			ride_degrees_steer(20, 50) -- steering right

			sleep(1)
			set_lift("shake_router")
			shake()
			set_lift("put_router")
			set_rotate(0)
			set_rotate(1)
			set_lift("up")

			ride_degrees_steer(20, 50, -20) -- steering back right
			ride_degrees(DEGREES, -20) 

			set_lift("up")
			set_rotate(0)

		elseif where == 2 then
			DEGREES = 180
			set_rotate(2) 
			line_degrees(DEGREES) -- forward

			sleep(1)
			set_lift("shake_router")
			shake()
			set_lift("put_router")

			set_rotate(0)
			set_lift("up")
			ride_degrees(DEGREES, -20) -- return
		end
	end

end

function set_defaults()
	r_set_pid(0.5, 0, 20.0)
	r_set_pidb(0.5, 0, 20.0)

	r_set_lspeed(30)
	r_set_ldegrees(90)

	r_set_rspeed(10)
	r_set_mspeed(10)

	r_set_white(50)
	r_set_middle_grey(35)
	r_set_black(20)

	D_ride_degrees_speed = 10
end

function main()
	-- CUR_POINT = "13"
	-- CUR_ANG = 0
	CUR_POINT = "9"
	CUR_ANG = 270
	set_defaults()

end

function take_cable()
	line_degrees(250)
	set_lift("take_cable")
	set_lift("up")
	ride_degrees(250, -20) 
end


function get_wire(num)
	if num == 1 then
	elseif num == 1 then
	end
end

function get_1_wire()
	goto_point("44")
	rotate_to_point("39")
	take_cable()
end

function put_1_wire()
	gotop("7")
	rotate_to_point("23")
	wait_till_arrival()
	give_cable()
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

function fake_read()
	current_colors_i = {red=1, blue=2, green=3, yellow=4}
end

main()

-- start_degrees_ride()
-- start_line_ride()

-- set_lift("take_router")
-- sleep(3)
-- set_lift("up")
-- exit()

-- get_router(1)
fake_read()
put_router("red", "short")
-- ride_degrees(360, 20) 
-- ride_degrees(360, -20) 
-- get_router(2)
-- put_router("blue", "short")
-- get_router(3)
-- put_router("yellow", "short")
-- get_router(4)
-- put_router("green", "short")
