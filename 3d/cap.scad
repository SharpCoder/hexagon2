$fn=100;

    difference() {
        union() {
            linear_extrude(1.5)
            circle(d=37);
            
            color("red")
            linear_extrude(3)
            circle(d=35);
        }
        
        linear_extrude(100)
        translate([0, 25/2+2, 0])
        circle(d=12);
        
    }
