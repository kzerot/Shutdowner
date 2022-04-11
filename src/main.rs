
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::process::Command;
use std::env;
mod volume;

fn handle_client(mut stream: TcpStream) {
    let mut cmd_shutdown = Command::new("cmd");
    cmd_shutdown.arg("/C").arg("shutdown").arg("/s").arg("/f").arg("/t").arg("00");


    let mut cmd_reboot = Command::new("cmd");
    cmd_reboot.arg("/C").arg("shutdown").arg("/r").arg("/f").arg("/t").arg("00");

    let mut data = [0 as u8; 128]; // using 50 byte buffer
    println!("Handling...");
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut fin = true;
                if size > 0{
                println!("Received!");
                println!("{}", size);
                let s = from_utf8(&data).unwrap();
                let string = &s[..size];
                println!("{}", &string);
                let answer;
                let temp_string;
                
                if "SHUTDOWN".eq(string){
                    println!("SHUTDOWN IN PROGRESS");
                    answer = "OK";
                    cmd_shutdown.output().expect("Failed to shutdown");
                }
                else if "REBOOT".eq(string){

                    println!("REBOOT IN PROGRESS");
                    answer = "OK";
                    cmd_reboot.output().expect("Failed to reboot");
                }
                else if "VOLUP".eq(string){
                    let vol = volume::get_volume() + 0.05f32;
                    volume::change_volume(vol.clamp(0.0, 1.0));
                    temp_string = format!("{}%", (vol * 100f32) as i32);
                    answer =  temp_string.as_str();

                }
                else if "VOLDOWN".eq(string){
                    let vol = volume::get_volume() - 0.05f32;
                    volume::change_volume(vol.clamp(0.0, 1.0));
                    temp_string = format!("{}%", (vol * 100f32) as i32);
                    answer =  temp_string.as_str();
                    
                }

                else if string.starts_with("VOL"){
                    let str_number = string.replace("VOL", "");
                    let number = str_number.parse::<f32>();
                    match number{
                        Ok(ok) => {
                            answer = "OK";
                            let vol = ok.clamp(0.0, 100.0f32);
                            volume::change_volume(vol/100.0f32);
                        }
                        Err(_e) => {
                            answer = "Invalid number";
                        }
                    }
                }
 
                else if "GETVOL".eq(string){
                    temp_string = format!("{}%", (volume::get_volume() * 100f32) as i32);

                    answer =  temp_string.as_str();
                }
                else if "MUTE".eq(string){
                    volume::mute(true);
                    answer = "OK";
                }
                else if "UNMUTE".eq(string){
                    volume::mute(false);
                    answer = "OK";
                }
                else{
                    answer = "UNKNOWN COMMAND";
                }
                let res = stream.write(answer.as_bytes());
                match res {
                    Ok(_) => println!("Sent"),
                    Err(_) => {
                        println!("Already disconnected");
                        fin = false;
                    },
                }
            }
            fin
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


fn main() {
    // tray();
    let args: Vec<String> = env::args().collect();
    let mut port = "3333";

    match args.len() {
        3 => {
            if &args[1] == "-p"{
                port = &args[2];
            }
        },
        _ => {}
    }
    // volume::mute(false);
    // volume::change_volume(0.1);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    println!("Try to handle...");
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}