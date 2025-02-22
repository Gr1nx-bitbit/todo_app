<span style="font-family:'Amiri'">

# AWS-SDK for Rust
The SDK for Rust seems a bit more complicated than the one for Go. It's just way harder to read, man. Anyways, the ReadMe for the SDK is linked [here](https://github.com/awslabs/aws-sdk-rust). I'm just going to kind of summarize what I need to be able to use the Rust SDK and also what I want to do with the SDK.

## Dependencies
The Rust SDK requires the `tokio` crate in order to perform `async` operations. A couple things I have to learn about before I start touching this SDK are what the `async` keyword does and how it's generally used (probably analogous to Go's `go func` and `Sync` stuff) as well as learn some stuff about the `tokio` crate. `tokio` is a runtime library for Rust which is kind of modeled after JS. It has a singular event loop which listens for different events and then schedules their operations (I'll learn about this in more depth).

Anyways, the other essential crate needed for this SDK to function is the `aws-config` crate. At least this is similar to the Go examples, lol. This crate, just like the package in Go, looks for your credentials in a few different places depending on the programs host. If it's running on your machine, it checks your environment variables and `~/.aws/[config || credentials]` file. If you're on a container, or an EC2 instance, you have to provide different info. 

You **NEED** these crates in your cargo.toml file:
```
[dependencies]
aws-config = { version= "1.5.16", features = ["behavior-version-latest"] }
tokio = { version = "1", features = ["full"] }
```
You can then add whatever other service you want as a crate. 

# CLI Stuff
I'm just going to have some CLI examples in this sectoin so we have some stuff to work with on LocalStack.

Creating a table in DynamoDB (S stand for String):
```
aws dynamodb create-table \
--table-name bok \
--attribute-definitions="AttributeName=Author,AttributeType=S" \
--key-schema="AttributeName=Author,KeyType=HASH" \
--billing-mode=PAY_PER_REQUEST \
--endpoint-url http://localhost:4566
```

# Comments
## 02/21/25
Holy majoly. Finally got the aws thing to run with Rust. Turns out you have to wait for asynchronous functions to finish... just like in any other languages. No wonder my program wasn't producing any ouput, it exited before the call to localstack could even be made. Anyways, now that we have some basic stuff with dynamo, I think we can try and look at some larger things. If we want to be able to store stuff in DynamoDB we're going to have to look at the service a bit more to understand what we need for it. This means we have to understnad how AWS makes it work since its a NoSQL db and because we want to stay in free tier. The next things we should do then is just research more about Dynamo and make a schema for it. While the types we have in our test file is cool, we have to base them off an actual database!
</span>