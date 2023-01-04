An experiment to see if I can run something like Lisp as the frontend of
the [rail machine](https://github.com/hiljusti/rail).


Some ideas:

```
These should be identical

1 1 +       # Rail
(+ 1 1)     # Lisp

1 dup +     # Rail
(+ (dup 1)) # Lisp

These should be ... translatable

1 [[n]-> n dup] do +            # Rail      (Note: -> is not a builtin... Should it be?)
(+ (do (fn n -> (dup n)) 1))    # Lisp      (Note: Lambda syntax could be different)

(+ (do (-> [n] (dup n)) 1))     # More literal translation, but wonky... may need an alias for `->` or some other `def` sugar for lambdas?

+
|
do____
|     \
(->)   1
|   \
[n]  dup
     |
     n
```
