## ChatCmd 
A command line tool that lets you become a PowerShell or Bash expert by providing commands and arguments for anything you might want done in a terminal.  
Just run `chatcmd <question for a command>`, and it will print the desired command (*and also copy it to clipboard*)

**Example**

- `chatcmd install rustup using choco`  
`choco install rustup.install`

- `chatcmd list all files in the current directory older than a month but newer than a year`  
`find . -type f -mtime +30 -mtime -365`

- `chatcmd list all files but pring only the file and date`  
`ls -l --time-style=+"%Y-%m-%d" | awk '{print $6, $7}'`


### Installation
ChatCmd requires an OpenAI account. It expects `OPENAI_API_KEY` as environment variable. Otherwise it will ask you for your API key. (The API key can also be set through an .env file)

To build you need a [rust toolchain](https://rustup.rs/)  
Build with `cargo build --release`

It is recommended to add chatcmd to the path so it can be invoked easily.   
`chatcmd add chatcmd to path`  
`echo 'export PATH=$PATH:/path/to/chatcmd' >> ~/.bashrc && source ~/.bashrc`

### Usage
As mentioned just do `chatcmd <question for a command>`. Depending on your platform it will provide Bash or PowerShell commands, though you can also specicy what type of command you want:

`chatcmd what is the grep command for powershell`  
`Select-String`


ChatCmd also has an optional argument `-dev`, that slightly changes the system prompt to produce simple scripts or small answers for other languages.  
`chatcmd -dev write me python program that can find leap years`

```python
def is_leap_year(year):
    return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)

start_year = int(input("Enter start year: "))
end_year = int(input("Enter end year: "))

for year in range(start_year, end_year + 1):
    if is_leap_year(year):
        print(year)

```

It can also accept input from sdin:  
`chatcmd -dev make a simple hello world in python > test.out`

Then
`chatcmd improve this by adding a date < test.out`
OR
`cat test.out | chatcmd improve this by adding a date`

