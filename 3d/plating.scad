include <config.scad>

PLATE_THICKNESS = 3;

linear_extrude(PLATE_THICKNESS)
square([HEX_HEIGHT, 65], center=true);

color("red")
translate([HEX_HEIGHT/2 - 3.4 - 1.6 + .1, 0, 0])
union() {
    linear_extrude(HEX_BORDER + PLATE_THICKNESS)
    square([8-.75, 10-.2], center=true);

    /*
    translate([.1, 0, HEX_BORDER + PLATE_THICKNESS])
    linear_extrude(1)
    square([8-.55, 10-.2], center=true);
    */
}