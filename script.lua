C = {}

C.l_deg = 90
C.r_sp = 40
C.m_sp = 70

C.l_white = 50
C.l_grey = 35
C.l_black = 20

C.usual_line = {
	pf_cf = 0.7,
	df_cf = 1.2,
	sf_sp = 50,

	ps_cf = 1,
	ds_cf = 0,
	ss_sp = 50,

	top_pt = 656554,
	bot_pt = 544654,
	lx_cf = -1,
}

C.accurate_line = {
	pf_cf = 0.7,
	df_cf = 1.2,
	sf_sp = 30,

	ps_cf = 1,
	ds_cf = 0,
	ss_sp = 50,

	top_pt = 656554,
	bot_pt = 544654,
	lx_cf = -1,
}

function line_degrees(degrees)
	r_ride_line_degrees(degrees)
	r_wait_till_arrival()
end

function ride_degrees(degrees, speed)
	r_set_mspeed(C.m_sp)

	if speed ~= nil then
		r_set_mspeed(speed)
	end
	r_ride_degrees(0, degrees)
	r_wait_till_arrival()

	r_set_mspeed(C.m_sp)
end

function set_line_args(args)
	r_set_pid(args.pf_cf, args.df_cf, args.sf_sp, 
			args.ps_cf, args.ds_cf, args.ss_sp, 
			args.lx_cf, args.top_pt, args.bot_pt)
	r_set_pidb(args.pf_cf, args.df_cf, args.sf_sp, 
			args.ps_cf, args.ds_cf, args.ss_sp, 
			args.lx_cf, args.top_pt, args.bot_pt)
end

function start()
	r_set_ldegrees(C.l_deg)
	r_set_rspeed(C.r_sp)
	r_set_mspeed(C.m_sp)

	r_set_white(C.l_white)
	r_set_middle_grey(C.l_grey)
	r_set_black(C.l_black)
	
	set_line_args(C.usual_line)
end


function get_color()
	h1, s1, v1 = r_get_cs_hsv()
	h2, s2, v2 = r_get_cs_hsv()
	h3, s3, v3 = r_get_cs_hsv()
	h4, s4, v4 = r_get_cs_hsv()
	h5, s5, v5 = r_get_cs_hsv()
	
	h = (h1 + h2 + h3 + h4 + h5) / 5
	s = (s1 + s2 + s3 + s4 + s5) / 5
	v = (v1 + v2 + v3 + v4 + v5) / 5
	-- print(tostring(h)..", "..tostring(s)..", "..tostring(v))

	if s < 180 then
		return "white"
	end
	if (h > 140) and (h < 270) then
	      return "blue"
	end
	if (h > 330) or ((h < 15) and (v < 60)) then
		return "red"
	end
	if (h < 60) then
	      return "yellow"
	end
	return "none"
end

function set_current_pos(POINT, ROTATION)
	CUR_POINT = POINT
	CUR_ANG = ROTATION
end

function goto_point(point)
	r_goto_point(point)
	r_wait_till_arrival()
end

function rotate_to_point(point)
	r_rotate_to_point(point)
	r_wait_till_arrival()
end

function set_lift(where)
	if where == "down" then degrees = 0
	elseif where == "take" then degrees = 635
	elseif where == "trans" then degrees = 300
	elseif where == "put_up" then degrees = 1550
	elseif where == "put_down1" then degrees = 1320
	elseif where == "put_down2" then degrees = 1300
	elseif where == "put_down3" then degrees = 1280
	end

	r_set_lift(degrees, 100)
end

function l_take_ring()
	set_lift("take")
	set_line_args(C.accurate_line)
	line_degrees(200)

	for i=1, 3 do
		set_lift("down")
		set_lift("trans")
	end

	ride_degrees(200, -C.m_sp)
	set_line_args(C.usual_line)
end

function get_ring(ring_num)
	tp = "c" .. ring_num
	tr = "c" .. ring_num .. "t"

	goto_point(tp)
	rotate_to_point(tr)
	l_take_ring()
end

function check_ring(ring_num)
	tp = "c" .. ring_num
	tr = "l" .. ring_num

	goto_point(tp)
	rotate_to_point(tr)

	color = get_color()
	return color
end

function put_ring(ring_num)
	goto_point("pc")
	rotate_to_point("pm")

	set_line_args(C.accurate_line)
	set_lift("put_up")
	line_degrees(365)

	for i=1, 3 do
		set_lift("put_down" .. i)
		set_lift("put_up")
	end

	ride_degrees(365, -C.m_sp)
	set_lift("trans")
	set_line_args(C.usual_line)
end

-- MAIN PROGRAM FUNCS
function move_from_start()
	ride_degrees(200)
	set_current_pos("b", 90)
end

function finish()
	goto_point("b")
	ride_degrees(260)
        set_lift("down")
end

function gnfc(c)
	if c == "red" then return 1
	elseif c == "yellow" then return 2
	elseif c == "blue" then return 3
	elseif c == "white" then return 4
	else return 0
	end
end

function gcfn(n)
	if n == 1 then return "red"
	elseif n == 2 then return "yellow"
	elseif n == 3 then return "blue"
	elseif n == 4 then return "white"
	else panic()
	end
end

function calc_fourth_ring(rings)
	fourth_ring = 10 - (gnfc(rings[1]) + gnfc(rings[2]) + gnfc(rings[3]))
	return gcfn(fourth_ring)
end

-- MAIN PROGRAM EXEC
start()
move_from_start()

order = {"red", "blue", "white", "yellow"}
rings = {}

we_took_ring = false
for i=1, 3 do
	if i == 1 then
		check_ring(i)
		r_ride_degrees(-100, 20)
		r_wait_till_arrival()
	end
	rings[i] = check_ring(i)
	print(i)
	print(rings[i])
	if rings[i] == order[1] then
		get_ring(i)
		we_took_ring = true
	end
end

rings[4] = calc_fourth_ring(rings)

if not we_took_ring then
	get_ring(4)
end
put_ring()

for i=2, 4 do
	cur_target = order[i]

	for j=1, 4 do
		if rings[j] == cur_target then
			target_num = j
			break
		end
	end
	get_ring(target_num)
	if i == 4 then
		finish()
	else
		put_ring()
	end
end
