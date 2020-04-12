#!/usr/bin/env python

def fib(n):
  a = 0
  b = 1
  while True:
    if n == 0:
      return a
    elif n == 1:
      return b
    else:
      a, b = b, a+b
      n -= 1

print(fib(15))