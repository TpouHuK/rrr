TRANS_LINE = {}
SUPER_FAST_LINE = {}

ACCUR_LINE = {
	p_fast = 0.8,
	d_fast = 30,
	speed_fast = 20,

	p_slow = 1,
	d_slow = 30,
	speed_slow = 10,

	top_cap = 900,
	bot_cap = 50,
	lx_coff = 0.9,
}

---[[
SUPER_FAST_LINE = {
	p_fast = 0.4,
	d_fast = 20,
	speed_fast = 80,

	p_slow = 2.5,
	d_slow = 50,
	speed_slow = 20,

	top_cap = 400,
	bot_cap = 50,
	lx_coff = 0.9,
}
--]]

TRANS_LINE = {
	p_fast = 0.7,
	d_fast = 30,
	speed_fast = 30,

	p_slow = 0.5,
	d_slow = 30,
	speed_slow = 20,

	top_cap = 900,
	bot_cap = 100,
	lx_coff = 0.9,
}

-- SUPER_FAST_LINE = TRANS_LINE


-- Magic
MAGIC_ON = true 
MAGIC_DSC = 95
MAGIC_RGR = 10

BLACK_WHITE_CS_LIMIT = 40

-- Line coeff
P_M = 0.8
I_M = 0
D_M = 20

P_S = 0.4
I_S = 0 
D_S = 8

-- Line detection
WHITE = 45 + 5
MIDLE_GREY = 35
BLACK = 20 - 5 

-- Start
START_DEGREES_UP = 320
START_DEGREES_LEFT = 110
START_DEGREES_DIAG = 90
START_DEGREES_RIGHT = START_DEGREES_LEFT

-- Line speed
FIRST_LINE_SPEED = 40
LINE_SPEED = 40
FAST_LINE_SPEED = LINE_SPEED

-- Line cross degrees
LINE_UDEGREES = 85
RIDE_DEGREES_SPEED = 20

-- Rotate speed
ROTATE_SPEED = 30
-- Degrees speed (does nothing)
MACRO_SPEED = 30


-- Router put
ROUTER_SLEEP_TIME = 2
LONG_DEGREES = 247
BACK_LONG_NOTRIDE = 0

ROUTER_GLOBAL = 13 + 5

ROUTER_0 = 150 + ROUTER_GLOBAL
ROUTER_1 = ROUTER_0
ROUTER_2 = ROUTER_0
ROUTER_3 = ROUTER_0

-- Router
ROUTER_GET_SPEED = 25
ROUTER_BACK_SPEED = 25
ROUTER_ROTATE_SPEED = 20
ROUTER_DEGREES_BACK = 100
ROUTER_DEGREES_FORWARD = 50 + 50
PUT_ROUTER_SPEED = 30
SHAKE_ROUTER_LIFT_DELTA = 10

-- Wire put
WIRE_SHAKE = 20
WIRE_PUT_DEGREES = 231
WIRE_PUT_LINE_DEGREES = 410
WIRE_PUT_SPEED_CUBICS = 20
WIRE_PUT_SPEED_NOTHING = 60
WIRE_PUT_OVERSHOOT_COMP = -10
WIRE_PUT_SPEED_F = 20
WIRE_PUT_SLEEP = 1


WIRE_GET_DEGREES = 255
WIRE_GET_U_SPEED = 25
WIRE_GET_D_SPEED = 55
WIRE_GET_SLEEP = 0

-- Shake
SHAKE_SPEED = 15
FORWARD_SHAKE_DEGREES = 48
SIDE_SHAKE_DEGREES = 30

-- Finish
FINISH_DEGREES_ROTATE = 48
FINISH_DEGREES = 700

-- Lift setup
LIFT_TAKE_WIRE    = 651
LIFT_PUT_WIRE     = 415  
LIFT_TAKE_ROUTER  = 483
LIFT_SHAKE_ROUTER = 0
LIFT_PUT_ROUTER   = 210
LIFT_FINISH       = 540
LIFT_PRE_PUT      = 0


LIFT_SPEED = 100
ROTATE_LIFT_SPEED = 20
