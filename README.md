# museum-brute-force

Brute force solver for the Online Reality Game Proditum's reoccuring puzzle.
The name ended up becoming a bit of a misnomer, since it was made for the Museum's last puzzle, but later got reused throughout each escape room.
Standard output will only contain the answer, and standard error will indicate progress updates.

---

- [Usage](#usage)
- [Building](#building)
- [Refactor](#refactor)
- [License](#license)

---

## Usage

- The main program is to brute force the best possible answer, you are intended to edit the `puzzle.txt`.
  - The first line represents all the initial amount of BP/Gold that each user has, comma-separated. This must be **EXACTLY** 9 elements long to compile successfully.
  - The second line represents the target. If less than 9 elements are specified, the users to the rightmost are tested for maximization.
- Tests exist to validate an answer, you are intended to additionally edit the `vaults` declaration and assertion.

While editing source is inconvenient, this program is specifically designed for performance first, ease-of-use second.
This program runs best if you run on a CPU with 32 cores, such as the code author's `AMD Ryzen 9 7950X`. If you don't have one, it should still run reasonably fast.
That being said, the program is designed to use 27 cores to divide the task quickly and evenly. Not all cores are used since many applications tend to be single-threaded, so this is a considered a best compromise.

## Building

As with any rust project, you simply run with `cargo run --release`, no special dependencies.
Be sure to compile the program in release mode, as without optimizations, this program is comparable to the old C# implementation.
```sh
cargo run --release
```

You may also compile the program in `VERBOSE` mode which logs every time a thread finds a new best answer for its specific task.
The environment variable is checked at compile-time, not at runtime, hence the command:
```sh
VERBOSE=1 cargo run --release
```

Bear in mind, for huge computations the program will not output anything for a while, as the array is cycled from the right.
This means that it assumes that users to the left that have some target will all submit 0, while the users to the right permute.

## Refactor

This repository is a refactor of my original implementation written in C#, also available in the repository as a [`.csx` file](src/OldImplementation.csx).
The old implementation worked for the first time, but was too slow, only computing 300 thousand simulations per second.
The new implementation which was required when this puzzle was reused computes over 1 billion simulations per second.
To run the old implementation, use [`dotnet-script`](https://github.com/dotnet-script/dotnet-script) or [`CSharpRepl`](https://github.com/waf/CSharpRepl) on it, or any other C# scripting language.
Alternatively, replace the `#r` import with the returned contents from the URL and run it as a non-scripted C# program.

## License

This repository falls under the [Unlicense](https://unlicense.org). Do whatever you want with the code. No credit ever required, but it would still be appreciated.

Bare in mind, while the [`.csx` file](src/OldImplementation.csx) itself still falls under the license, the [`Emik.Morsels`](https://github.com/Emik03/Emik.Morsels) source code it imports **still falls under its license**, and is **<ins>not</ins> overriden by this license**.
