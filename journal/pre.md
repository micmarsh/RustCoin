# RustCoin Development Journal (Pre-work Thoughts)

Today I decided I was going to write a Bitcoin full node client in Rust. I know more about the nitty-gritty of Bitcoin's protocol than I do about Rust, (which means I know every little about either), but both would be valuable things to do, so why not?

The first thing to figure out is how to figure out how to write a Bitcoin client. That's not a typo, that a legitimate concern. Let's start with what you can think of all by your lonesome:
* Start working on writing [this](http://www.followthecoin.com/build-bitcoin-script-interpreter-javascript/) in Rust. This is mostly because you really want to write some Rust, who knows if you'll even use this.
* Read the [protocol specification](https://en.bitcoin.it/wiki/Protocol_specification) a few times. You probably won't understand everything, but as you read you may start getting a vague picture of how it all fits together, and things you could start working on.
* Read the source of [bitcoind](https://github.com/bitcoin/bitcoin) (if you dare), [bitcoinj](https://github.com/bitcoinj/bitcoinj) (will probably be the easiest for you), and [btcd](https://github.com/conformal/btcd/blob/master/btcd.go) (possibly difficult). At the very least, you'll be able to see the way the namespaces are generally divided up, giving you a better idea of how to break things down for yourself.
