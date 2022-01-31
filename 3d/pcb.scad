pcb_width = 40.64;
pcb_height = 12.7;
pcb_thickness = 2.6;

linear_extrude(pcb_thickness)
difference() {
    // PCB
    square([pcb_width, pcb_height], center=true);
   
    // Mounting hole
    translate([pcb_width/2 -5.08, 0, 0])
    circle(d=4.2, $fn=100);
    
    // LED bores
    translate([-6, 0, 0])
    union() {
        translate([0,-4.5,0])
        circle(d=2, $fn=100);
        
        translate([0,-1.25,0])
        circle(d=2, $fn=100);
        
        translate([0,1.5,0])
        circle(d=2, $fn=100);
        
        translate([0,4.5,0])
        circle(d=2, $fn=100);
    }
}