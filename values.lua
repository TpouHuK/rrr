H = {}

-- #Line
H.line = {
	-- Distance from sensors to wheels
	udegrees_dg = 85 + 10,

	-- ##Coefficients
	-- Accurate line for actions with objects
	accurate = {
		pf_cf = 0.8,
		df_cf = 30,
		sf_sd = 30,

		ps_cf = 1,
		ds_cf = 30,
		ss_sd = 10,

		top_pt = 900,
		bot_pt = 50,
		lx_cf = 0.9,
	},

	-- Fast line where we moving on long distances
	fast = {
		pf_cf = 0.7,
		df_cf = 65,
		sf_sd = 80,
		      
		ps_cf = 2.0,
		ds_cf = 20,
		ss_sd = 20,
		      
		top_pt = 500,
		bot_pt = 200,
		lx_cf = 0.99,
	},

	-- Typical line to move around without speed/precision
	trans = {
		pf_cf = 0.7,
		df_cf = 30,
		sf_sd = 40,
		     
		ps_cf = 0.8,
		ds_cf = 30,
		ss_sd = 20,
		     
		top_pt = 900,
		bot_pt = 100,
		lx_cf = 0.9,
	},
}

-- #Line detection
H.colors = {
	white_pt = 50 - 1,
	grey_pt = 35,
	black_pt = 15 + 3,
}


-- #Magic
H.magic = {
	on = true ,
	dsc = 95,
	rgr = 10,
	back_take = true,
}

-- #Router color reading
H.router_bw_limit_pt = 40

-- #Start move
local turn_dg = 110
H.start = {
	up_dg = 320,
	left_dg = turn_dg,
	diag_dg = 90,
	right_dg = turn_dg,
}

H.speed = {
	degrees_sd = 20, --+ 10,
	rotate_sd = 30, --+ 10,
	macro_sd = 30, --+ 10,
	degrees_sd = 30, --+ 10,
}


-- #Router put
local router_global = 18 - 4 - 4 - 5 - 5
local asdf = 3

H.put_router = {
	wait_sc = 1,
	long_dg = 247 + asdf,
	backlongnoride_dg = 0,

	r0_dg = 150 + router_global - asdf,
	r1_dg = router_0,
	r2_dg = router_0,
	r3_dg = router_0,

	speed_sd = 50,
}


-- #Router get
H.get_router = {
	get_sd = 25,
	back_sd = 25,
	rotate_sd = 20,

	back_dg = 100,
	forward_dg = 50 + 50,
	put_sd = 30,
}

-- #Wire put
H.put_wire = {
	forward_sd = 20,
	cubics_sd = 20,
	nothing_sd = 40,

	line_dg = 410,
	forward_dg = 231,
	overshoot_dg = -10,

	wait_sc = 1,
}

-- #Wire get
H.get_wire = {
	forward_dg = 255,

	forward_sd = 25,
	back_sd = 55,

	wait_sc = 0,
}

-- #Finish
H.finish = {
	rotate_dg = 48,
	forward_dg = 700,
}

-- #Lift setup

H.lift = {
	take_wire    = 651,
	put_wire     = 415,
	take_router  = 483,
	shake_router = 0,
	put_router   = 210 + 40,
	finish       = 540,
	pre_put      = 0,
	back_take    = 530,
	speed = 100,
}

-- #Hand
H.hand_speed = 20
