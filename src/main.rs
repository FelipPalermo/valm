use sysinfo::{ProcessExt, System, SystemExt};
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

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

    let delay = 10 * 60; // 20 minutos em segundos
    let program_name = "Safari"; // Defina o nome do processo a ser buscado

    if is_program_running(program_name) {
        println!("Programa '{}' esta em execucao!", program_name);

        let output = Command::new("shutdown")
        .arg("-s")
        .arg("-t")
        .arg(delay.to_string())
        .output()
        .expect("Falha ao executar o comando");

        if output.status.success(){
            println!("Comando deu certo, seu computador desligara em '{}' segundos!", delay);
        }else {
            print!("Comando falhou, o computador nao desligara! {}", delay);
        }



    } else {
        println!("O programa '{}' não está em execução.", program_name);
    }
}