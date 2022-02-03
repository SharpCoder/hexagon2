include <config.scad>;
$fn = 6;
linear_extrude(SHIELD_THICKNESS)
circle(d=HEX_WIDTH-.5);
