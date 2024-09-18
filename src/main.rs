use sysinfo::{ProcessExt, System, SystemExt, Pid};
use std::process::Command;

fn find_pid(program_name: &str) -> Option<Pid> {
    let mut system = System::new_all();
    system.refresh_all();

    // Itera sobre todos os processos em execução
    for (pid, process) in system.processes() {
        if process.name().to_lowercase() == program_name.to_lowercase() {
            println!("Processo encontrado! PID: {}", pid);
            return Some(*pid); // Retorna uma cópia do valor de `pid`
        }
    }

    // Se não encontrou o processo, retorna None
    None
}



fn find_ports_for_pid(pid: Pid) -> Vec<u16> {
    let mut ports = Vec::new();

    // Executa o comando netstat para listar as conexões
    let output = Command::new("netstat")
        .arg("-ano") // Mostra todas as conexões, endereços e PIDs
        .output()
        .expect("Falha ao executar netstat");

    // Converte a saída em string
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Filtra as linhas que contém o PID e extrai as portas
    for line in stdout.lines() {
        if line.contains(&pid.to_string()) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                // A segunda coluna contém o endereço local (IP:Porta)
                if let Some(port_str) = parts[1].split(':').last() {
                    if let Ok(port) = port_str.parse::<u16>() {
                        ports.push(port);
                    }
                }
            }
        }
    }

    ports
}

fn block_port(port: u16) {
    // Comando para bloquear a porta usando PowerShell
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "New-NetFirewallRule -DisplayName \"BlockPort{}\" -Direction Outbound -LocalPort {} -Protocol TCP -Action Block",
            port, port
        ))
        .output()
        .expect("Falha ao executar comando do Firewall");

    if output.status.success() {
        println!("Porta {} bloqueada com sucesso!", port);
    } else {
        println!(
            "Falha ao bloquear a porta {}: {}",
            port,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}


fn main() {
    let program_name = "LeagueClient.exe"; // Nome do programa

    let pid = match find_pid(program_name) {
        Some(pid) => pid,  // Se encontrou o PID, retorna o valor
        None => {
            println!("Programa não encontrado");
            return; // Encerra a execução do `main`
        }
    };

    // Continue com o uso de `pid` após o `match`
    let ports = find_ports_for_pid(pid);

    for port in ports {
        block_port(port);
    }
}