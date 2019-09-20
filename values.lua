-- Magic
MAGIC_ON = true
MAGIC_DSC = 95
MAGIC_RGR = 10

BLACK_WHITE_CS_LIMIT = 40

-- General
P_M = 0.8
I_M = 0
D_M = 20

P_S = 0.4
I_S = 0 
D_S = 8

FIRST_LINE_SPEED = 60                           --кубики

FAST_LINE_SPEED = 80
LINE_SPEED = 40 + 5
LINE_UDEGREES = 90 - 5
RIDE_DEGREES_SPEED = 20

ROTATE_SPEED = 30
MACRO_SPEED = 30

WHITE = 45
MIDLE_GREY = 35
BLACK = 20

-- Router put
LONG_DEGREES = 250
BACK_LONG_NOTRIDE = 10
ROUTER_0 = 160 - 10
ROUTER_2 = 180

ROUTER_1 = 130 + 10 + 10
ROUTER_1S = 50 - 10
ROUTER_3 = ROUTER_1
ROUTER_3S = ROUTER_1S

-- Router
ROUTER_GET_SPEED = 25                            --подъезд
ROUTER_BACK_SPEED = 25 + 10                      --отъезд
ROUTER_ROTATE_SPEED = 20                         --поворот к роутору
ROUTER_DEGREES_BACK = 100
ROUTER_DEGREES_FORWARD = 50
PUT_ROUTER_SPEED = 30

-- Wire put
WIRE_PUT_DEGREES = 220 + 10                      --
WIRE_PUT_LINE_DEGREES = 410                      --
WIRE_PUT_SPEED_CUBICS = 20                       --
WIRE_PUT_SPEED_NOTHING = 50 + 10                 --
WIRE_PUT_OVERSHOOT_COMP = -15                    --
WIRE_PUT_SPEED_F = 30                            -- подъезд кабеля ложь
WIRE_PUT_SLEEP = 0.8                          


WIRE_GET_DEGREES = 265 - 3                       --
WIRE_GET_U_SPEED = 25                            --
WIRE_GET_D_SPEED = 55                            --
WIRE_GET_SLEEP = 0.4
-- Shake
SHAKE_SPEED = 10 + 5
FORWARD_SHAKE_DEGREES = 50 - 10
SIDE_SHAKE_DEGREES = 30 - 10

-- Finish
FINISH_DEGREES = 300

-- Lift setup
LIFT_TAKE_WIRE    = 600 + 40 - 10
LIFT_PUT_WIRE     = 460 + 10
LIFT_TAKE_ROUTER  = 400 + 40 + 10 + 10
LIFT_SHAKE_ROUTER = 100 + 40
LIFT_PUT_ROUTER   = 500 + 40
LIFT_FINISH       = 500
LIFT_PRE_PUT      = 120 - 15

LIFT_SPEED = 100                                 -- скорость поднималки
ROTATE_LIFT_SPEED = 20                           -- скорость вращалки
-- Start
START_DEGREES_UP = 320
START_DEGREES_LEFT = 110
START_DEGREES_DIAG = 90
START_DEGREES_RIGHT = START_DEGREES_LEFT
