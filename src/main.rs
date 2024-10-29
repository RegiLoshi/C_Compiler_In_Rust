use std::env;
use std::fs;
use std::path::Path;
use std::process;
use c_compiler_lib::lex;
use c_compiler_lib::parser;
use c_compiler_lib::assembly;
use c_compiler_lib::tac;


fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path is provided
    // if args.len() > 10 {
    //     // eprintln!("Usage: {} <input_file>", args[0]);
    //     eprintln!("ARGUMENTS: {:?}", args);
    //     process::exit(1);
    // }


    // Get the input file path
    let input_file = Path::new(&args[1]);

    // Read the input file
    let input = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", input_file.display(), err);
            process::exit(1);
        }
    };

    

    // Create a lexer instance and get tokens
    let mut lexer = lex::Lex::new(&input);
    let mut tokens = lexer.get_tokens();

    //Remove comments from tokens
    tokens.retain(|token| token.token_type != lex::TokenType::COMMENT && token.token_type != lex::TokenType::LongComment);

    // Parse the program
    match parser::parse_program(&mut tokens) {
        Ok(program) => {
            println!("Parsing successful");
            let tac = tac::generate_tac(program);
            let mut assembly = assembly::generate_assembly_AST(tac);
            println!("{:?}", assembly);
            assembly.applyFixes();
            println!("{:?}", assembly);
            let assembly_code = assembly.to_assembly_file();
            println!("{}", assembly_code);
            
            // Generate output file name (same as input but without extension)
            let output_file = input_file.with_extension("");
            
            // Write assembly to a temporary file
            let asm_file = output_file.with_extension("s");
            if let Err(e) = fs::write(&asm_file, assembly_code) {
                eprintln!("Error writing assembly file: {}", e);
                process::exit(1);
            }

            // Use GCC to assemble and link
            let status = process::Command::new("clang")
                .arg("-o")
                .arg(&output_file)
                .arg(&asm_file)
                .status()
                .expect("Failed to execute GCC");

            if !status.success() {
                eprintln!("GCC failed to assemble and link");
                process::exit(1);
            }

            // Remove the temporary assembly file
            fs::remove_file(asm_file).expect("Failed to remove temporary assembly file");

            println!("Compilation successful. Output: {}", output_file.display());

            // Now execute the compiled binary and capture its exit status
            let run_status = process::Command::new(output_file.to_str().unwrap())
                .status()
                .expect("Failed to execute the compiled program");

            // Print the exit status of the compiled program
            if run_status.success() {
                println!("Program executed successfully with exit status: 0");
            } else if let Some(code) = run_status.code() {
                println!("Program exited with status code: {}", code);
            } else {
                println!("Program terminated by signal");
            }
        }
        Err(e) => {
            // Parsing failed, print error and exit with non-zero code
            eprintln!("Text input: {}", input);
            eprintln!("Tokens: {:?}", tokens);
            eprintln!("Parsing error: {}", e);
            process::exit(1);
        }
}
}