include <config.scad>

module pcb_mount() {
    
    difference() {
        $fn = 100;
        linear_extrude(8)
        square([ 20, 15 ], center=true);
        linear_extrude(10)
        circle(d=4.2);


        translate([0, 0, 1.6])
        union() {
            $fn = 6;
            linear_extrude(3.2)
            translate([0, -2, 0])
            circle(d=7.2);

            translate([0, 4, 0])
            linear_extrude(4.2)
            square([7.2, 12], center=true);
        }  
    }
}

//pcb_mount();