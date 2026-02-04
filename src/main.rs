mod estado;
mod grid;
mod pecas;
mod visual;

fn main() {
    let mut estado = estado::Estado::new();
    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        let _ = stdin.read_line(&mut input);

        match input.chars().next().unwrap_or_default() {
            'a' => estado.mover(-1),
            'd' => estado.mover(1),
            'e' => estado.girar(1),
            'q' => estado.girar(-1),
            _ => (),
        }
        estado.tick();
        estado.render();
    }
}
