This repo illustrate an issue I'm having with `argonautica`: the same code
produces different outputs depending on whether it's running directly on my
machine or in a docker container.

Locally (Archlinux, kernel `5.3.1-arch1-1-ARCH`)

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/argonautica-issue`
$argon2id$v=19$m=4096,t=192,p=8$EvpazrlY0ApjLxAXz/Xgv301w1BVvpf8iACX57If9/I$uQFyf1OfHwfuoNNt4RwR1p9Esbtqh8mfPY8B66N9rLY
```

In docker:

```
$ docker build . -t argonautica-issue
$ docker run -t argonautica-issue
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/argonautica-issue`
$argon2id$v=19$m=4096,t=192,p=8$na6G3tOLyjQOZLfQkNFPJagLKzNkCX1sTtn0L9TJN0s$8C0R7jRHbUrck67r0wneJxG2g2U1phUW1ObK1UNd+eY
```
