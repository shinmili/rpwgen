rpwgen
==

## What's this?

Generates password. Picked characters can be configured from command-line argument.

## Usage

oneliner:
```
$ cargo run -- 16 --chars あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもらりるれろわをん
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    Running `target/debug/rpwgen 16 'あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもらりるれろわをん'`
めにしらめえあこわあしらねとらそ
```

or build & run:
```
$ cargo build --release
$ ./target/release/rpwgen 16 --chars あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもらりるれろわをん
ほろろるるにそなせれもほぬちれけ
```

The first argument (16 here) is the length of the generated password. The second argument (あいうえお... here) is the list of characters used in the generated password.

## License

[MIT License](https://github.com/shinmili/rpwgen/blob/master/LICENSE).

## Author

[Kuroki Keiichiro](https://github.com/shinmili)
