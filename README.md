rpwgen
==

## What's this?

Generates password. Picked characters can be configured from command-line argument.

## Usage

Once you cloned;
```
$ git clone https://github.com/shinmili/rpwgen.git
$ cd rpwgen
```
you can generate one by this oneliner (with hiraganas);
```
$ cargo run -- 16 --chars ぁ-ん
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    Running `target/debug/rpwgen 16 --chars 'ぁ-ん'`
ひえすゑゃぇえぅそぇどゎぎぅふい
```
or by build & run scheme (with alnums excluding confusing characters):
```
$ cargo build --release
$ ./target/release/rpwgen 16 --chars A-HJ-NP-Za-km-z1-9
ftEz9e8MfSVYiVXx
```
Want some hyphens in your password? You can escape it:
```
$ ./target/release/rpwgen 16 --chars 'A-Za-z0-9\-'
lleT5a1fnF-5rdbl
```
Don't forget single quotes, or your **shell** will parse the backspace.

Once built, `rpwgen --help` will show the help, but you don't need to take a look; The example above shows all the features implemented for now...

## License

[MIT License](https://github.com/shinmili/rpwgen/blob/master/LICENSE).

## Author

[Kuroki Keiichiro](https://github.com/shinmili)
