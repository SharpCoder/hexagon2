include <config.scad>;

thickness = 12;
h = 116;
w = 5;
tol = 1;
lip = 4;

difference() {
    linear_extrude(thickness)
    union() {        
        square([h, w], center=true);

        translate([-h/2,-(HEX_HEIGHT+tol)/2,0])
        square([w,HEX_HEIGHT+tol+w], center=true);

        translate([-h/2+(lip/2), -(HEX_HEIGHT+tol+w), 0])
        square([w+lip, w], center=true);
    }
    
        
    translate([h/2 - 10, 50, thickness/2])
    union() {
        rotate([90, 90, 0])
        linear_extrude(100)
        circle(d=4.5, $fn=100);
        
        translate([-3, 0, 0])
        rotate([90, 90, 0])
        linear_extrude(100)
        circle(d=6, $fn=100);
    }
}
