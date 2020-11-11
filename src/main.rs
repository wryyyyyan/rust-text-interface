use std::io;
use std::collections::HashMap;


fn parse_command() -> u32 {
    loop {
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("failed to read line");

        match option.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue
        }
    }
}


fn view(company: &mut HashMap<String, Vec<String>>, keys_list: &Vec<String>) {
    loop { 
        println!("\nVisualizar");
        println!("0) Voltar\n1) Escolher Departamento\n2) Todos\n");
        
        match &parse_command() {
            0 => { break;  },
            1 => { 
                println!("\nDepartamentos");
                println!("0) Voltar");
                for (index, item) in keys_list.iter().enumerate() {
                    println!("{}) {}", index + 1, item);
                }
                let command = parse_command();
                if command == 0 {
                    return;
                } 
                else {
                    match keys_list.get(command as usize - 1) {
                        Some(item) => { 
                            println!("\n{}\n", item);
                            let employee_list = &mut company.get(item).expect("erro").to_vec();
                            employee_list.sort_unstable();
                            for employee in employee_list.iter() {
                                println!("{}", employee);
                            }
                        },
                        None => println!("Indice Invalido")
                    }
                }
            },
            2 => {
                for item in keys_list.iter() {
                    println!("\n{}\n", item);
                    let employee_list = &mut company.get(item).expect("erro").to_vec();
                    employee_list.sort_unstable();
                    for employee in employee_list.iter() {
                        println!("{}", employee);
                    }
                }

            },
            _ => println!("Opção Inválida")
        }
    }
}


fn update_departments_list(keys_list: &mut Vec<String>, company: &HashMap<String, Vec<String>>) {
    for key in company.keys() {
        if !keys_list.contains(&key) {
            keys_list.push(String::from(key));
        }
    }
}


fn main() {
    
    let mut lista = HashMap::new();

    //lista.insert("Engineering".to_string(), vec!["Jose".to_string(), "Marco".to_string(), "Yuri".to_string(), "Eduardo".to_string(), "Alisson".to_string()]);
    //lista.insert("Sales".to_string(), vec!["Felipe".to_string(), "Vinicius".to_string(), "Antonio".to_string(), "Jonatan".to_string(), "Ismael".to_string()]);
    //lista.insert("Human Resources".to_string(), Vec::new());
    
    let mut keys_list = Vec::new();
    update_departments_list(&mut keys_list, &lista);

    loop {
        println!("\nEscolha um comando");
        println!("0) Sair\n1) Adicionar Departamento\n2) Adicionar Funcionário\n3) Visualizar\n");
        
        match &parse_command() {
            0 => { break; },
            1 => {
                println!("\nDigite o nome do departamento");
                let mut command = String::new();
                io::stdin().read_line(&mut command).expect("Failed to read line");

                lista.insert(command.trim().to_string(), Vec::new());
                update_departments_list(&mut keys_list, &lista);
            },
            2 => {
                loop {
                    if keys_list.is_empty() {
                        println!("Não há departamentos");
                        break;
                    }
                    println!("\nEscolha um departamento");
                    for (index, item) in keys_list.iter().enumerate() {
                        println!("{}) {}", index + 1, item);
                    }

                    match keys_list.get(parse_command() as usize - 1) {
                        Some(item) => {    
                            println!("\nDigite o nome do funcionario");
                            let mut funcionario = String::new();
                            io::stdin().read_line(&mut funcionario).expect("Failed to read line");

                            lista.get_mut(item).expect("item inexistente").push(funcionario.trim().to_string());
                            break
                        },
                        None => { 
                            println!("Indice Invalido");
                            continue
                        }
                    }
                }
            },
            3 => { view(&mut lista, &keys_list);  },
            _ => { println!("erro"); }
        }
    }
}
