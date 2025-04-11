use std::io::{ self, Write };
use std::process::Command;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;

fn main() {
    let flag_vypnuti = Arc::new(Mutex::new(false));
    let mut vypnuti_nastaveno = false;

    loop {
        print!(">_ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("stop") {
            if vypnuti_nastaveno {
                let mut flag = flag_vypnuti.lock().unwrap();
                *flag = true;
                vypnuti_nastaveno = false; // resetovani casovace
                println!("planovane vypnuti bylo zruseno");
            } else {
                println!(
                    "jeste sis nenastavil vypnuti pocitace, takze nemuzes pouzit prikaz 'stop'"
                );
            }
            continue;
        }

        match input.parse::<u64>() {
            Ok(minuty) => {
                if vypnuti_nastaveno {
                    println!(
                        "uz mas nastavenej casovac na vypnuti pocitace, pokud chces jinej cas, nejdriv pouzij prikaz 'stop'"
                    );
                    continue;
                }

                let flag_vypnuti = Arc::clone(&flag_vypnuti);
                vypnuti_nastaveno = true;

                println!("pocitac ti vypnu za {} minut", minuty);

                thread::spawn(move || {
                    for _ in 0..minuty * 60 {
                        thread::sleep(Duration::from_secs(1));
                        let flag = flag_vypnuti.lock().unwrap();
                        if *flag {
                            return;
                        }
                    }

                    println!("vypinam pocitac");

                    #[cfg(target_os = "windows")]
                    {
                        Command::new("shutdown")
                            .args(["/s", "/t", "0"])
                            .output()
                            .expect("nemuzu ti vypnout pociatc");
                    }

                    #[cfg(target_os = "linux")]
                    {
                        Command::new("shutdown")
                            .arg("now")
                            .output()
                            .expect("nemuzu ti vypnout pocitac");
                    }
                });
            }
            Err(_) => {
                println!("vubec nevim co to je za sracku, ale tohle '{}' urcite neni pocet minut", input);
            }
        }
    }
}
