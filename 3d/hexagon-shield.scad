include <config.scad>;
$fn = 6;
linear_extrude(SHIELD_THICKNESS)
circle(d=HEX_WIDTH);

linear_extrude(SHIELD_THICKNESS + SHIELD_DIFFUSER_PADDING)
circle(d=HEX_WIDTH-6);