# aws-lambda-warm-cold
This repo shows you how to take advantage of warm/cold code in Rust lambdas.

After AWS runs a lambda it keeps it "warm" for a while. If the lambda is invoked before that while is up then it runs faster, and costs less.

![Dory from the film Finding Nemo saying "I should save my money, oh look shiny things!"]( save_money.jpg )

There are several steps in AWS running a lambda, e.g. downloading your code, provisioning the execution environment, running the initialisation code, and finally running the handler code.

This repo is concerned with the initialisation code and the handler code, and how these run in rust and how you can make use of this. It's not complicated but as of 2022-09-28 it's also not clear in existing documentation and blogs. And where warm/cold execution is mentioned it's not clear how to get references from the initialization code to the handler code.

See [main.rs](https://github.com/jamescoleuk/aws-lambda-warm-cold/blob/main/src/main.rs) for how to do this.



## Running this locally

It's easy to test this locally, rather than deploying it on AWS.

You'll need to install the excellent [cargo-lamda](https://github.com/cargo-lambda/cargo-lambda):

```shell
cargo install cargo-lambda
```

First you need to start the `watch` process. This is the local lambda control plane:

```shell
cargo lambda watch
```

Then you need to invoke your lambda:
```shell
cargo lambda invoke --data-ascii "{}" aws-lambda-warm-cold"
```

You'll see the initialization message on this first run. But if you run the `invoke` again you won't see it.