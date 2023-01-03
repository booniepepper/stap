An experiment to see if I can run something like Scheme as the frontend of
the [rail machine](https://github.com/hiljusti/rail).


Some ideas:

```
These should be identical

1 1 +       # Rail
(+ 1 1)     # Scheme

1 dup +     # Rail
(+ (dup 1)) # Scheme

1 [[n]-> n dup] do +            # Rail      (Note: -> is not a builtin... Should it be?)
(+ (do (fn n -> (dup n)) 1))    # Scheme    (Note: Lambda syntax could be different)

```
