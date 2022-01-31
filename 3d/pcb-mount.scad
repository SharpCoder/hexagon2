include <config.scad>

module pcb_mount() {
    
    linear_extrude(10)
    translate([0, -9, 0])
    square([13, 3], center=true);
    
    difference() {
        $fn = 100;
        linear_extrude(10)
        square([ 13, 15 ], center=true);
        linear_extrude(10)
        circle(d=4.2);


        translate([0, 0, 3])
        union() {
            $fn = 6;
            linear_extrude(3.2)
            translate([0, -1, 0])
            circle(d=7.2);

            translate([0, 5, 0])
            linear_extrude(3.2)
            square([7.2, 12], center=true);
        }  
    }
}

//pcb_mount();