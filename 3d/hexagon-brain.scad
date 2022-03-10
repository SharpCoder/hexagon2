include <hexagon-core.scad>;

// The end result looks silly but
// it will work.
module power_cord_hole() {
    translate([4,0,10])
    rotate([30,90,0])
    translate([0,0,30])
    linear_extrude(50)
    circle(d=11.5, $fn=100);
}

difference() {
    hexagon_core();
    power_cord_hole();
}

rotate(60)
linear_extrude(10)
square([40,74], center=true);
