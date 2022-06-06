# Summer of Rust Lab 5: First Martians - Mission 1

This week, we'll be learning more about modules and crates in Rust. We'll also
get an overview of error handling and common collections.

In this week's lab, we're going to work on a Rust crate from scratch, and write
tests for it. The implementation is a bit more open-ended than in previous labs,
and you might run into some walls when working through the sections. Be sure to
ask questions in the `Lab 5 Questions` channel.

Here's a reminder of the different sections headers:

- (info): This section teaches something
- (impl): This section wants you to write some code
- (next): Something to try after the lab

## Mission 1

<p align="center">
<img
src="https://cdn.discordapp.com/attachments/968184771102507031/980953696248664194/unknown.png"
width="400" />
</p>

In this first part of the Mars Mission 🚀, we're going to rank candidates
through an advanced scoring system. There are a few steps to this ranking
system: getting their job score, and calculating their candidate score.

Unlike previous labs, you'll write much of the code for this one yourself.
You'll also need to write some tests to make sure your code is working
correctly.

There are two crates that are part of this lab: `space-agency` and `personnel`.
You won't need to make any changes to the `personnel` crate, but you should
certainly read through it. That said, you're free to change any parts of it if
you want to experiment. The `space-agency` crate uses `personnel` as a
dependency, so you'll be able to use the `Candidate` and `AstronautJob` types
from that crate.

### (impl) Generate candidates file

First, you'll need to generate the candidates file. There will be about 30,000
candidates in this file, but the code to generate them has already been made. To
generate the file, run the following from the root of the lab:

```bash
cargo run --example generate-candidates
```

This will place the `candidates.txt` file at the root of the project. Note, this
file has been added to the `.gitignore` file, so it won't show up when you go to
commit your work.

Also note, when running your program with `cargo run`, make sure you're doing it
from the root of the project, and you're not in the `space-agency` directory.
This is because the program will look for the `candidates.txt` file in the same
folder you're in, and it won't find it if you're in the `space-agency` directory.

The file that generates the candidates is part of the `personnel` crate. You
don't need to make any changes to it, but feel free to [take a look at it
here](personnel/examples/generate-candidates.rs).

### (impl) Scoring candidates

The candidate file has the following syntax:

```text
Primary skill, Secondary skill, Age, Health
```

You'll be doing your work inside the `main.rs` file in the `space-agency` crate,
and you'll be able to load the candidate file with an already-implemented
`load_candidates_file` function. This function is an **associated function** on
the `Candidate` struct, and you'll need to import it. You can do that with the
following at the top of the `main.rs` file:

```rust
use personnel::Candidate;
```

Next, let's calculate the score of a candidate. There are two parts to the
ranking system: the job score, and the candidate score.

#### (impl) Job score

First, you'll have to get the code of each job. The job codes are as follows:

```text
Biogeochemist => 251
Biologist => 257
Engineer => 263
Geologist => 269
Mechanic => 271
Medic => 277
RoverOp => 281
Scientist => 283
```

To calculate the job score, multiply the code of the primary and secondary
skills. Since each candidate has a primary skill, but might not have a secondary
skill, you'll need to handle the case where the secondary skill is missing. In
this case, just multiply the code of the primary skill by itself.

Finally, you'll need to make sure that the job score is between 0 and 576. If it
is above 576, it should wrap back around to 0. For example, if the job score is
577, it should wrap back around to 0. Similarly, 432_632_341 would wrap around
to 49.

#### (impl) Candidate score

Now we can calculate the candidate score. This should be nice and easy, you just
need to sum the candidate's health with the job score from the previous step.
Next, multiply this sum by the age of the candidate.

Finally, you'll need to make sure that the candidate's score is between 0 and
3928. Same as above, if it is above 3928, it should wrap back around to 0.

### (impl) Ranking candidates

Now that we have a system to score candidates, we can use it to rank them. We
should already have a vector of Candidates (`Vec<Candidates>`), so now we'll
need to sort it. We'll be able to use the functions we created above to do this.

In Rust, you can sort a vector by using the `sort` method. However, this
requires that the type of the vector "implements `Ord`". For example, `i32`
[already implements
`Ord`](https://doc.rust-lang.org/std/primitive.i32.html#impl-Ord). Basically,
any `i32` knows how to compare itself to other `i32`s, and tell which one is
bigger.

For us though, we want to compare `Candidates`. Rust doesn't know how to do this
by default, so we need to implement the `Ord` trait for `Candidates`. But wait!
We won't be learning about traits until next week, so instead let's just use
`sort_by` to specify how the vector should be sorted.

```rust
let mut numbers = vec![523, 274, 742, 163, 984, 567, 890];

// This will work since i32 implements Ord, but let's do it with `sort_by` anyway
// numbers.sort();

// Sort the list
numbers.sort_by(|a, b| {
    // We want to sort in descending order
    b.cmp(a)

    // If we wanted to instead sort with the lowest number first, we could use this instead:
    // a.cmp(b)
});
```

<p align="center">
<img
src="https://cdn.discordapp.com/attachments/968184771102507031/980955031480205352/unknown.png"
width="400" />
</p>

### (info) Interesting syntax

One more thing to point out is the interesting syntax that we see inside the
`sort_by` function above. Let's break it down to a simpler form:

```rust
var.method(|parameter| {
    // Code
});
```

So what we can see here is that we have some method that is being called on a
variable. But what's more interesting is what the method is taking as a
parameter. Specifically, we can see that the parameter is a closure:

```rust
|parameter| {
    // Code
}
```

A closure is an "anonymous function", or a function that doesn't have a name. We
learned earlier that Rust allows us to return values from functions without the
`return` keyword. Closures are another example of how we can compact code that
is required in certain situations.

Closures also have many other implications, such as how they can be passed
around, or where they're useful. We'll get to those later when we explore
iterators and multithreading.

<details>
<summary>If you dare dive further...</summary>
Let's look at what sort_by's signature looks like:

```rust
pub fn sort_by<F>(&mut self, mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{ /* ... */ }
```

Link: https://doc.rust-lang.org/stable/src/alloc/slice.rs.html#329-331

We won't break down the whole signature, but let's look at some interesting
parts:

- `mut self`: This is the `&mut self` parameter. This is the reference to the
  vector that we want to sort.
- `mut compare`: This is the `mut compare` parameter. This is the closure that
    we want to use to sort the vector.
- `mut compare` is of type `F`. We also see `F` beside `sort_by<F>`, and as part
  of a `where` clause.
- In the `where` clause, we specify that `F` must implement the `FnMut` trait.
  Further, whatever this `FnMut` trait is, it takes in two `&T`s, and returns a
  `Ordering`. The two `&T`s are the two elements that we want to compare (we saw
  this as `|a, b|` in the example above).

That's a lot! We'll explore this more in the future, but it's kind of cool to
see what's happening under the hood of `sort_by`.
</details>

### (impl) Testing

Now that we're at the end of the ranking of candidates, we should test our code.
This week, there aren't any tests to start with. Since this lab didn't have as
much base code, it would have been difficult to write tests, since the function
names you came up with might not have matched the existing tests.

So for this week, you get to write some tests! Take inspiration from previous
labs, feel free to copy code, and choose whatever tests you like. Try to write 5
or more.

When you commit your code, the normal `cargo fmt` and `cargo clippy` checks will
be run to make sure the code looks good and compiles.

## Useful tips

### Converting numerical types

You can convert between number types in Rust with this syntax:

```rust
let num1: u32 = 1;
let num2: u64 = num1 as u64;
```

## (next) Rustlings

After this week's session, we should be able to do everything up to
`error_handling`. If you haven't worked that much on Rustlings yet, don't worry!
You can go at your pace, and always get to them when you have time.

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
