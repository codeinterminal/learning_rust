# TODO:

- give different probabilities to the pieces to show


- make it one against another, with network support (and
    NAT punch through or something like that :) )


# How to handle rotations

```
  Stick
 ______________
    .           .
     1
     2         1234
     3
     4

    Edge case: when close to wall

  |
  |
  |
  |

  | ####
  |    #
  | ####
  | ####


  Square:
 -------------
    .           .
     12          41
     34          32


  Stair left
 ------------
    .           .
     12           1
      34         32
                 4


  Stair right
 --------------
    .           .
      12         3
     34          41
                  2

```
