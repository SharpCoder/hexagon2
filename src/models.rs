pub struct SystemCommand {
    pub command: [u8;4],
    pub args: [i32; 10],
    pub checksum: u64,
}

impl SystemCommand {
    pub fn new() -> Self {
        return SystemCommand {
            command: [0; 4],
            args: [i32::MAX; 10],
            checksum: 0,
        };
    }
}