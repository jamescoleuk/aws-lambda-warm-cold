# aws-lambda-warm-cold
This repo shows you how to take advantage of warm/cold code in Rust lambdas.

After AWS runs a lambda it keeps it "warm" for a while. If the lambda is invoked before that while is up then it runs faster.

There are several steps in AWS running a lambda, e.g. downloading your code, provisioning the execution environment, running the initialisation code, and finally running the handler code.

This repo is concerned with the initialisation code and the handler code, and how these run in rust and how you can make use of this. It's not complicated but it's also not clear in existing documentation and blogs. Where warm/cold execution is mentioned it's not clear how to get references from the initialization code to the handler code.

## Running this

To build a lambda zip you'll need to install [cargo-lamda](https://github.com/cargo-lambda/cargo-lambda):
`cargo install cargo-lambda`

You can test this locally using `cargo-lambda`. First you need to start the `watch` process:
`cargo lambda watch`

Then you need to invoke your lambda:
`cargo lambda invoke --data-ascii "{}" aws-lambda-warm-cold"`

You'll see the initialization message on this first run. But if you run the `invoke` again you won't see it.