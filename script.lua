function gotop(point)
	goto_point(point)
	wait_till_arrival()
end

function line_degrees(degrees)
	ride_line_degrees(degrees)
	wait_till_arrival()
end

function ride_degrees(degrees, speed)
	speed = speed or 20
	r_ride_degrees(degrees)
	reset()
end


local function has_value (tab, val)
	for index, value in ipairs(tab) do
		if value == val then
			return true
		end
	end

	return false
end

function get_color()
	h, s, v = get_cs_hsv()
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

function take_cable()
	set_lspeed(20)
	line_degrees(250)
	set_vars()
	
	set_lift(-570)
	set_lift(0)

	unmacro("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0")
	wait_till_arrival()
end

function give_cable()
	set_lspeed(20)
	ride_line_degrees(410)
	wait_till_arrival()
	macro("0,0,0,0,0,0,0,0,0,0,0,0")
	wait_till_arrival()

	set_vars()
	
	set_lift(-460)
	set_lift(0)

	unmacro("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0")
	wait_till_arrival()
end

function read_colors()
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

function black_white_cs()
	h, s, v = get_cs_hsv()
	if v > 40 then
		return 'white'
	else
		return 'black'
	end
end

function get_1_wire()
	gotop("44")
	rotate_to_point("39")
	wait_till_arrival()
	take_cable()
end

function get_2_wire()
	gotop("45")
	rotate_to_point("43")
	wait_till_arrival()
	take_cable()
end

function put_1_wire()
	gotop("7")
	rotate_to_point("23")
	wait_till_arrival()
	give_cable()
end

function put_2_wire()
	gotop("38")
	rotate_to_point("16")
	wait_till_arrival()
	give_cable()
end

function get_colors()
	rolls()
	colors = read_colors()
	print(colors[1], colors[2], colors[3], colors[4])
	wait_till_arrival()
	macro("0,0,0,0")
	wait_till_arrival()
end

function set_vars()
	set_pid(1, 0, 2)
	set_pidb(0.5, 0, 10)

	set_lspeed(30)
	set_rspeed(30)
	set_mspeed(20)

	set_white(60)
	set_middle_grey(35)
	set_black(20)
end

function get_router(cub_n)
	gt = ""
	rt = ""
	if     cub_n == 1 then gt = "6"; rt = "58";
	elseif cub_n == 2 then gt = "7"; rt = "57";
	elseif cub_n == 3 then gt = "8"; rt = "56";
	elseif cub_n == 4 then gt = "1"; rt = "40";
	elseif cub_n == 5 then gt = "2"; rt = "41";
	elseif cub_n == 6 then gt = "3"; rt = "42";
	end

	goto_point(gt)
	set_rotate(0)
	wait_till_arrival()
	rotate_to_point(rt)
	wait_till_arrival()

	set_lspeed(10)

	unmacro("0,0,0,0,0")
	wait_till_arrival()
	set_lift(-580)

	ride_line_degrees(120)
	wait_till_arrival()
	set_lift(0)
	unmacro("0")
	wait_till_arrival()
	set_vars()
end

function check_router(cub_n)
	gt = ""
	rt = ""
	if     cub_n == 1 then gt = "6"; rt = "7";
	elseif cub_n == 2 then gt = "7"; rt = "8";
	elseif cub_n == 3 then gt = "8"; rt = "37";
	elseif cub_n == 4 then gt = "1"; rt = "2";
	elseif cub_n == 5 then gt = "2"; rt = "3";
	elseif cub_n == 6 then gt = "3"; rt = "45";
	end

	goto_point(gt)
	wait_till_arrival()
	rotate_to_point(rt)
	wait_till_arrival()

	return black_white_cs()
end

function shake()
	set_mspeed(5)
	unmacro("-100,100,0")
	wait_till_arrival()
	sleep(5)
	macro("-100,100,0")
	wait_till_arrival()
	sleep(5)
	set_vars()
end

function putout(l, where)
	set_lspeed(10)
	set_rotate(where * -90)

	if l == "long" then
		ride_line_degrees(400)
	else
		ride_line_degrees(120)
	end
	wait_till_arrival()

	if where == 2 then
		ride_line_degrees(60)
	elseif (where == 1) or (where == 3) then
		ride_line_degrees(20)
	end
	wait_till_arrival()

	sleep(5)
	set_lift(-190)
	sleep(5)
	shake()
	set_lift(-500)
	sleep(5)
	set_rotate(0)

	if where == 2 then
		set_lift(0)
	end

	if l == "long" then
		unmacro("0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0")
		ride_degrees(150)
		
	else
		unmacro("0,0,0,0,0,0,0")
	end

	wait_till_arrival()

	if where == 2 then
		unmacro("0,0,0")
	elseif (where == 1) or (where == 3) then
		unmacro("0")
	end
	wait_till_arrival()

	set_lift(0)
	set_vars()
end

function put_router_long(color)
	-- LOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOONG
	gt = ""
	rt = ""
	rp = 0
	if     color == "red"    then gt = "6";  rt = "22"; rp = 1;
	elseif color == "blue"   then gt = "8";  rt = "24"; rp = 1;
	elseif color == "yellow" then gt = "14"; rt = "15"; rp = 3;
	elseif color == "green"  then gt = "12"; rt = "17"; rp = 3;
	end


	a = current_colors_i[color]
	if     a == 1 then tp = 3
	elseif a == 2 then tp = 0
	elseif a == 3 then tp = 1
	elseif a == 4 then tp = 2
	end

	where = (tp - rp)
	if where < 0 then
		where = where + 4
	end

	goto_point(gt)
	set_rotate(0)
	wait_till_arrival()
	rotate_to_point(rt)
	wait_till_arrival()

	-- Putout
	putout("long", where)
end

function put_router_short(color)
	-- SHOOOORT SHORT SHORT SHRORT
	gt = ""
	rt = ""
	rp = 0
	if     color == "red"    then gt = "5";  rt = "20"; rp = 2;
	elseif color == "blue"   then gt = "25";  rt = "26"; rp = 0;
	elseif color == "yellow" then gt = "33"; rt = "36"; rp = 0;
	elseif color == "green"  then gt = "10"; rt = "18"; rp = 2;
	end


	a = current_colors_i[color]
	if     a == 1 then tp = 3
	elseif a == 2 then tp = 0
	elseif a == 3 then tp = 1
	elseif a == 4 then tp = 2
	end

	where = (tp - rp)
	if where < 0 then
		where = where + 4
	end

	goto_point(gt)
	wait_till_arrival()
	rotate_to_point(rt)
	wait_till_arrival()

	-- Putout
	putout("short", where)
end

-- START HERE

curr_router_num = 1
current_routers = {
	"u", "u", "u",
	"u", "u", "u",
}

black_routers_count = 0

CUR_POINT = "13"
CUR_ANG = 0
set_vars()

-- CUR_POINT = "38"
-- CUR_ANG = 180
-- current_colors_i = {
-- 	red=1,
-- 	green=2,
-- 	blue=3,
-- 	yellow=4,
-- }
-- put_router("green")
-- exit()

get_colors()
--get_1_wire()
--put_1_wire() 

for i=1,6 do
	r = check_router(i)
	if r == "black" then
		get_router(i)
		black_routers_count = black_routers_count + 1
		if     black_routers_count == 1 then put_router_long("red")
		elseif black_routers_count == 2 then put_router_long("blue")
		elseif black_routers_count == 3 then put_router_short("green")
		elseif black_routers_count == 4 then put_router_short("yellow"); 
		end
	end
	if black_routers_count == 4 then
		break
	end
end
--get_2_wire()
--put_2_wire() 

goto_point("30")
wait_till_arrival()
