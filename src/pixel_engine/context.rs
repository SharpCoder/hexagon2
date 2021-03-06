use teensycore::clock::uNano;

/// Context represents attributes for a single hexagon node.
/// Each unit in the overall design will have its own corresponding
/// context instance.
#[derive(Copy, Clone)]
pub struct Context { 
    pub initialized: bool,
    pub node_id: uNano, // These are time units because they will be integrated with time in math
    pub total_nodes: uNano,
    pub temperature: i32,
    pub registers: [i32; 10],
    pub offset: uNano,
}

impl Context {
    pub fn empty() -> Self {
        return Context {
            initialized: false,
            node_id: 0,
            total_nodes: 0,
            temperature: 0,
            registers: [0; 10],
            offset: 0,
        };
    }
}