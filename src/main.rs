use std::collections::HashMap;

use simulador::*;

fn main() {
    let instructions = HashMap::new();

    let p = Processador::new(
        Registers::default(),
        Memory::default(),
        ProgCounter(0),
        instructions,
        HashMap::new(),
    );
}
