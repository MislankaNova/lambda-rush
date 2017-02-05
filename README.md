# lambda-rush
Shooting game written in Rust, with trace amounts of Haskell and Lisp.

Made during ICHack17.

[Get it here](https://github.com/MislankaNova/lambda-rush/releases/tag/0.1.0).
# How to play?

+ Press <kbd>arrow key</kbd>s to move.
+ Press <kbd>z</kbd> to fire.
+ Press <kbd>x</kbd> to `unsafePerformIO`.
+ Press <kbd>esc</kbd> to exit.

Every time an enemy is destroyed, you gain some scores. The more enemies you have destroyed the more score you get from a single hit. Hitting enemies also raises your rank. Higher the rank, the more enemies there are. Once you have reached a high rank, you may use `unsafePerformIO` to remove enemies close to you, and also to lower your rank for a little bit.

You are warned that, however, when `unsafePerformIO`ing, your score does not increase. `unsafePerformIO` also removes all score multiplier you have previously gained. Be safe!

# Why?

Because monadic bind operator (>>=) in Haskell looks cool so I thought that it would be great to be a part of my game graphics.

# How?

I started writing a shooting game a few weeks ago, from which I borrowed most codes of lambda-rush. The game is written in Rust, which is both highly efficient and fault-proof, ie. no time wasted solving segmentation faults. For the graphical part I used SFML which provided a nice layer of abstraction over openGL and such.

# Anything Else?

No.
