# Trivial Compression of Genes

In this project, a long gene sequence of nucleotides A, C, G, and T is transformed from a string of characters into a big integer. In this big integer every two bits represent a nucleotide, so the necessary bytes of memory are reduced.

As a remainder, an integer is an instance of the class `int` which is a representation of the `PyLongObject` class. You can read more in this blog [“Python behind the scenes #8: how Python integers work”](https://tenthousandmeters.com/blog/python-behind-the-scenes-8-how-python-integers-work/). 

To run the program use this command:

```shell
$ poetry run main
```
  