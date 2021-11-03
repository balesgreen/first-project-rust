/*
This project has been created by Rafael (mark security)
02/11/2021
*/

use std::io;
use ansi_term::Colour;


fn main(){
    println!("{}{}",
        Colour::Yellow.paint("\n\tWelcome to my "),
        Colour::Green.bold().paint("World!\n"));


   println!("{}\n {}\n {}\n\n", 
    Colour::Green.bold().paint("\t[1] - Conhecer meu Canal!"),
    Colour::Green.bold().paint("\t[2] - Conhecer meu GitHub!"),
    Colour::Green.bold().paint("\t[3] - Conhecer meu Linkedin!")
    );

    println!("Escolha a opção: ");
    let mut question = String::new();

  io::stdin()
  .read_line(&mut question)
  .expect("Não consegui ler esta linha!");


  let question: i32 = question.trim().parse().unwrap(); /* Transformando a string "question" em inteiro para não alertar
   nenhum erro na hora da compilação*/

    if question == 1 {
        println!("\nIrei te levar até meu Canal!");
            youtube();
    }

    else if question == 2 {
        println!("\nIrei te levar até meu GitHub!");
            github();
    }

    else if question == 3 {
        println!("\nIrei te levar até meu Linkedin!");
            linkedin();
    }

    fn youtube(){
        println!("{}",
        Colour::Yellow.bold().paint("Welcome to my Channel! https://www.youtube.com/c/MarkSecurity/videos")
    );
    }

    fn github(){
        println!("{}",
        Colour::Yellow.bold().paint("Welcome to my GitHub! https://github.com/MarktwainSTDLL/")
    );
    }

    fn linkedin(){
        println!("{}",
        Colour::Yellow.bold().paint("Welcome to my Linkedin! https://www.linkedin.com/in/rafael-rom%C3%A3o-17a9481ba/")
    );
    }
}