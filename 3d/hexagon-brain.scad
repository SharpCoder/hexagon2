include <hexagon-core.scad>;

module power_cord_hole() {
    translate([4,0,10])
    rotate([30,90,0])
    translate([0,0,30])
    linear_extrude(50)
    circle(d=18, $fn=100);
}

module wire_channel() {
    translate([14, 0, 11.6])
    translate([-14, 0, -11])
    linear_extrude(8)
    translate([0,50,0])
    square([10, 50], center=true);
    
}

difference() {
    hexagon_core();
    union() {
        translate([-30,-2,5.5+5])
        power_cord_hole();
        
        translate([0,0,6])
        rotate(60)
        wire_channel();
    }
}


