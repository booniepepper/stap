# 🛑 (stap)

Stap (STAck Processor) is an experimental riff on Lisp (LISt Processor) but
uses a persistent global stack like your favorite concatenative programming
language. Pronounce it like "stop" and perhaps also stop before trying anything
too crazy with this language.

The following are all equivalent in stap:

```
# Note: "pl" is "print line"

(pl (+ 1 1))

1 (pl (+ 1))

1 1 (pl (+))

(+ 1 1) (pl)

1 (+ 1) (pl)

1 1 (+) (pl)
```

# Y Tho?

I wanted to see if I could use the [Rail](https://github.com/hiljusti/rail)
virtual machine for a Lisp.

# More explanation

TODO: What is this madness?

# Etc

A 2023 side quest of [J.R. Hill](https://so.dang.cool).
