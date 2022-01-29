include <config.scad>

difference() {
    $fn = 100;
    linear_extrude(10)
    square([ 10, 10 ], center=true);
    linear_extrude(10)
    circle(d=4.2);


    translate([0, 0, 3])
    union() {
        $fn = 6;
        linear_extrude(3.2)
        circle(d=7.2);

        translate([0, 5, 0])
        linear_extrude(3.2)
        square([7.2, 12], center=true);
    }  
}