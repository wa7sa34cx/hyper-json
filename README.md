# Hyper-JSON

I just try to demonstrate my parallel and asynchronous programming skills using [tokio] and [hyper] libraries.

[tokio]: https://github.com/tokio-rs/tokio
[hyper]: https://github.com/hyperium/hyper

## Deployment

1. Fork this repository to your folder
1. Edit `.env.example` by putting there your `ENDPOINT`
1. Rename `.env.example` to `.env`
1. Now, just execute `cargo run` from your terminal.

## Usage

Just type this command in your terminal:

`cargo run`

You will get something like this

```
INFO  hyper_json > Task 4: price=5631 and max_price=4315
INFO  hyper_json > Task 5: price=18279 and max_price=9490
INFO  hyper_json > Task 8: price=2000 and max_price=1217
INFO  hyper_json > Task 9: price=5849 and max_price=4457
INFO  hyper_json > Task 1: price=10685 and max_price=7833
INFO  hyper_json > Task 6: price=11964 and max_price=6672
INFO  hyper_json > Task 7: price=2717 and max_price=1674
INFO  hyper_json > Task 10: price=16898 and max_price=8890
INFO  hyper_json > Task 3: price=13584 and max_price=8973
INFO  hyper_json > Task 2: price=11907 and max_price=6032
```

The program runs 10 tasks in parallel mode. Whichever task is faster, the result of that one will be printed first.

## License

This project is licensed under the [MIT license](LICENSE).
