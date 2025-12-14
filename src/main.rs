fn main() {
    let print_usage = ||{
        print!("USAGE:\n ms add [SERVICE]     -  add service\n ms remove [SERVICE]  -  remove service\n ms list              -  list all services\n") //функция, где пишем сообщение как использовать
    };
    let arguments: Vec<String> = std::env::args().collect(); // парсим аргументы
    if arguments.len() == 2 && arguments[1] == "list".to_string() {  list_services()  }// если 2 аргумента и 2аргумент равен list то пишем все сервисы
    else if arguments.len() == 3 { // иначе если 3 аргумента И
        if arguments[1] == "add" { // если 2 аргумент add то
            add_service(arguments[2].clone()) //берем 3 аргумент и добавляем сервис
        } else if arguments[1] == "remove"{ // если 2 аргумент remove ещ
            remove_service(arguments[2].clone()) // удаляем сервис
        } else {
            print_usage() // иначе выведи как использовать
        }
    }
    else {
        print_usage() // если не 2 и не 3 аргумента то выведи использование
    }
}
fn add_service(service: String) {
    println!("Adding Service..."); 
    let files: Vec<String> = std::fs::read_dir("/etc/sv/") //читаем директорию
    .unwrap() // 
    .map(|e| e.unwrap().file_name().into_string().unwrap())
    .collect();

    let services: Vec<String> = std::fs::read_dir("/var/service/") // читаем директорию 2
    .unwrap()
    .map(|e| e.unwrap().file_name().into_string().unwrap())
    .collect();
    if files.contains(&service) && !services.contains(&service)  { // если файл с названием сервиса
                                                                   // есть в дир1 но нет в дир2 то
        let mut path = String::new(); // создаем строку
        path.push_str("/etc/sv/"); //добавляем часть путя
        path.push_str(&service); // добавляем вторую, будет /etc/sv/service
        let ln_command = std::process::Command::new("ln") // делаем команду ln
        .arg("-s")
        .arg(path)
        .arg("/var/service/")
        .output()
        .expect("[ERROR] can't add link");
        if !ln_command.status.success() {
            println!("[ERROR] Unknown error"); // если не нормально выполнилась то пишем error
        }
        else {
            println!("Service added") //если все ок то пишем что добавлено
        }
    }
    else {
        println!("[ERROR] check /etc/sv or /var/service");
    }
}
fn remove_service(service: String) {
    println!("Removing...");
    let services: Vec<String> = std::fs::read_dir("/var/service/")
    .unwrap()
    .map(|e| e.unwrap().file_name().into_string().unwrap())
    .collect();

    if services.contains(&service) {
        let mut path = String::new();
        path.push_str("/var/service/");
        path.push_str(&service);
        std::fs::remove_file(path).unwrap_or_else(|why| { // удаляем ссылку
        println!("[ERROR] {:?}", why.kind());
    });
}
}
fn list_services() {
    let services: Vec<String> = std::fs::read_dir("/etc/sv/")
    .unwrap()
    .map(|e| e.unwrap().file_name().into_string().unwrap())
    .collect();
    for service in services {
        println!("{}", service);
    }
}
