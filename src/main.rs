use std::env;

use std::io;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Error;

#[cfg(dotenv_available)]
use dotenv_codegen::dotenv;

use atty::Stream;
use serde_json::json;

extern crate json;

const MAX_INPUT_SIZE: usize = 1024;

fn main() -> Result<(), Error> {
    let api_key = ask_for_key();
    if api_key.is_none() {
        return Ok(());
    }

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 || (args[1] == "-dev" && args.len() < 3) {
        println!("Usage: {} [-dev] <question>", args[0]);
        return Ok(());
    }
    let mut dev_mode = args[1] == "-dev";
    let idquery = if dev_mode { 2 } else { 1 };

    let mut user_input = env::args().skip(idquery).collect::<Vec<String>>().join(" ");

    if !atty::is(Stream::Stdin) {
        let mut stdin_input = String::new();
        io::stdin().read_to_string(&mut stdin_input).unwrap();

        if !stdin_input.is_empty()  {
            eprintln!("Reading from stdin");
            dev_mode = true;

            if stdin_input.len() > MAX_INPUT_SIZE {
                eprintln!("Input is too long. Truncating at {} characters.", MAX_INPUT_SIZE);
                stdin_input.truncate(MAX_INPUT_SIZE);
            }

            user_input += &(": ".to_owned() + &stdin_input);
        }
    }

    let client = Client::new();
    let api_key = api_key.unwrap();

    #[cfg(target_os = "windows")]
    let osspec = "You are an expert on windows batch and know the intrincate details of running programs in windows through it's command line versions. I'll ask you for help with some command of some program and you will return just one command line result without providing any explanation except that you are explicitily asked for it.Don't quote or escape the output.";
    #[cfg(target_os = "macos")]
    let osspec = "You are an expert on mac osx bash and know the intrincate details of running programs in windows through it's command line versions. I'll ask you for help with some command of some program and you will return just one command line result without providing any explanation except that you are explicitily asked for it.Don't quote or escape the output.";
    #[cfg(target_os = "linux")]
    let osspec = "You are an expert on linux bash and know the intrincate details of running programs in windows through it's command line versions. I'll ask you for help with some command of some program and you will return just one command line result without providing any explanation except that you are explicitily asked for it.Don't quote or escape the output.";

    let system = match dev_mode {
        true => "You are an expert developer. The user is also an experienced developer and need to ask a very specific question and need a consise answer providing only code without comments or explanations. Name variables and funcitons appropietly.Don't quote or escape the output. The output needs to be able to piped into a file. just print the code, no markdown block",
        false => osspec,
    };

    let max_tokens = if dev_mode { MAX_INPUT_SIZE/2 } else { 256 };
    let query = json!({
        "model": "gpt-4-turbo-preview",
        "messages": [
            {
                "role": "system",
                "content": system,
            },
            {
                "role": "user",
                "content": user_input,
            }
        ],
        "temperature": 1,
        "max_tokens": max_tokens,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .body(query.to_string())
        .send()?;

    // Check the response
    if response.status().is_success() {
        let parsed = json::parse(&response.text()?).unwrap();
        let result = parsed["choices"][0]["message"]["content"].to_string();
        copy_to_clipboard(&result).unwrap();
        println!("{}", result);
    } else {
        println!("Failed to call API: {}", response.status());
    }

    Ok(())
}

fn ask_for_key() -> Option<String> {
    let env_var_name = "OPENAI_API_KEY";
    let api_key = env::var(env_var_name);

    if api_key.is_ok() {
        return api_key.ok();
    } 

    #[cfg(dotenv_available)]
    {
        let dot_env_key = dotenv!("OPENAI_API_KEY");
        if dot_env_key.len() > 0 {
            return Some(dot_env_key.to_string());
        }
    }

    println!("Environment variable {} not found.", env_var_name);

    #[cfg(target_os = "windows")]
    {
        print!("Please enter the key: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim_end();

        if input.len() == 0 {
            println!("No key entered, Bye!");
            return None;
        }

        println!("You entered: {}", input);
        print!("Do you want to set the key? Type Y or Yes to confirm: ");
        io::stdout().flush().unwrap();

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation).unwrap();

        let confirmation = confirmation.trim();

        match confirmation {
            "y" | "Y" | "Yes" => {
                println!("Setting the key...");

                let _ = Command::new("cmd")
                    .args(&["/C", "setx", env_var_name, &input])
                    .output()
                    .unwrap();
                println!("Key set. Please restart the terminal to use it. (or call refreshenv)");
            }
            _ => {
                println!("Not setting the key... Bye!");
            }
        }
    }
    return None;
}

fn copy_to_clipboard(text: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    let process = Command::new("clip").stdin(Stdio::piped()).spawn();
    #[cfg(target_os = "macos")]
    let process = Command::new("pbcopy").stdin(Stdio::piped()).spawn();
    #[cfg(target_os = "linux")]
    let process = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn();
    if let Ok(mut child) = process {
        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to start clipboard command",
        ))
    }
}
