# issue-91671-a64-doctestfail
Repro for [Rust issue 91671].

Basically, i've been making edits following the strategies from
my [MCVE patterns blog post], and then using the check-mcve.sh script to confirm that the problem described
in the issue persists, and if so, automatically commits the state. (I decided to start automatically commiting
it, because I previously did the bad thing of making a lot of aggressive edits and then lost track of the last good state.
So now I'm checkpointing aggressively.

[MCVE patterns blog post]]: http://blog.pnkfx.org/blog/2019/11/18/rust-bug-minimization-patterns/

[Rust issue 91671]: https://github.com/rust-lang/rust/issues/91671
