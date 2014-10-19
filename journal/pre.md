# RustCoin Development Journal

Today I decided I was going to write a Bitcoin full node client in Rust. I know more about the nitty-gritty of Bitcoin's protocol than I do about Rust, (which means I know every little about either), but both would be valuable things to do, so why not?

The first thing to figure out is how to figure out how to write a Bitcoin client. That's not a typo, that a legitimate concern. Let's start with what you can think of all by your lonesome:
* Read the [protocol specification](https://en.bitcoin.it/wiki/Protocol_specification) a few times. You probably won't understand everything, but as you read you may start getting a vague picture of how it all fits together, and things you could start working on.