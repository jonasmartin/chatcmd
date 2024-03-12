### ChatCmd 
It's a command line tool intended to help running other programs or native tools.

It requires an OpenAI account. It will ask you for your API if it's not set on the enviornment.

Build with `cargo build --release`

It's recommended to add the path so it can be invoked easily. 

To use it:
`chatcmd question`, it will provide an answer (and also copy the response to to clipboard so it's ready to use)

It has also an optional argument `/dev` that changes a little bit the system prompt aiming to produce simple scripts or small answers for other languages.

### Examples:

```
> chatcmd how to install rustup using choco
choco install rustup.install
```


