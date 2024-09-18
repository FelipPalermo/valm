use sysinfo::{ProcessExt, System, SystemExt};

fn is_program_running(program_name: &str) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    // Itera sobre todos os processos em execução
    for (pid, process) in system.processes() {
        if process.name().to_lowercase() == program_name.to_lowercase() {
            println!("Processo encontrado! PID: {}", pid);
            return true;
        }
    }

    false
}

fn main() {
    let program_name = "Safari"; // Defina o nome do processo a ser buscado

    if is_program_running(program_name) {
        println!("O programa '{}' está em execução!", program_name);
    } else {
        println!("O programa '{}' não está em execução.", program_name);
    }
}