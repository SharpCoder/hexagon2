include <config.scad>;
$fn = 6;
tol = .2;

SHIELD_THICKNESS = 3;
SHIELD_DIFFUSER_PADDING = .5;

linear_extrude(SHIELD_THICKNESS)
circle(d=HEX_WIDTH+tol);

linear_extrude(SHIELD_THICKNESS + SHIELD_DIFFUSER_PADDING)
circle(d=HEX_WIDTH - 6 + tol);