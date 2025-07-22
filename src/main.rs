use base64::prelude::*;
use std::io::{self, Write};
use std::fs::File;
use std::process::{exit, Command};
use std::env::current_dir;
fn main() {
    print!("
    ████████████████████████████████████████████████████████████████████████    
  ██                                                                        ██  
██                                                                            ██
██    ████████████████████████████████████████████████████████████████████    ██
██  ██      ██                                                    ██      ██  ██
██  ██  ██  ██                                                    ██  ██  ██  ██
██  ██      ██    ██    ████    ██      ██  ██  ██████      ████  ██  ██  ██  ██
██  ██  ██  ██  ██  ██  ██  ██  ████  ████  ██    ██      ██████  ██      ██  ██
██  ██      ██  ██████  ██  ██  ██  ██  ██  ██    ██        ████  ██  ██  ██  ██
██  ██  ██  ██  ██  ██  ████    ██      ██  ██    ██        ████  ██      ██  ██
██  ██  ██  ██                                                    ██  ██  ██  ██
██  ██      ██                                                    ██      ██  ██
██    ████████████████████████████████████████████████████████████████████    ██
██                                                                            ██
  ██                                                                        ██  
    ████████████████████████████████████████████████████████████████████████    
    

");
    println!("please paste in the raw base64 output of your kerberos ticket.");
    println!("end the data with END in all caps");
    let mut ticket_data = String::new();
    loop{
        io::stdin().read_line(&mut ticket_data).unwrap();
        if ticket_data.contains("END"){
            break;
        }
    }
    let mut raw_ticket_vec: Vec<&str> = ticket_data.split("\n").collect();
    let mut trimmed_ticket_vec = Vec::new();
    for ticket_line in &mut raw_ticket_vec{
        trimmed_ticket_vec.push(ticket_line.trim_end().trim_start());
    }
    trimmed_ticket_vec.pop();
    trimmed_ticket_vec.pop();
    let ticket_b64 = trimmed_ticket_vec.join(""); 
    let decode_result = BASE64_STANDARD.decode(&ticket_b64);
    if decode_result.is_err(){
        let out = decode_result.err();
        println!("error decoding base 64!");
        dbg!(out);
        exit(1);
    }
    let ticket_bytes= decode_result.unwrap();
    print!("Ticket data Loaded\n\n{}\n\nOutputting to kirbi file\n", &ticket_b64);
    let kirbi = File::create("ticket.kirbi");
    if kirbi.is_err(){
        println!("error creating ticket.kirbi file");
        exit(1);
    }
    let mut kirbi_file = kirbi.unwrap();
    kirbi_file.write_all(&ticket_bytes).unwrap();
    let convert_result = Command::new("ticketConverter.py").arg("./ticket.kirbi").arg("./ticket.ccache").status(); 
    if convert_result.is_err(){
        println!("error running ticketConverter.py");
        println!("attempted Command: ticketConverter.py ./ticket.kirbi ./ticket.ccache");
        println!("error occured: {}", convert_result.unwrap());
        println!("please re-run the command manually and troubleshoot");
        exit(1);
    }
    let cwd = current_dir().unwrap();
    let variabl_string = format!("KRB5CCNAME={}/ticket.ccache", cwd.display());
    println!("ccache created! please copy and paste the following command into your terminal");
    println!("export {}", variabl_string);
}
