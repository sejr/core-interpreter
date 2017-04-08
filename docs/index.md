## Introduction

Core is a simple programming language described in CSE 5341, Principles of Programming Languages at The Ohio State University. It has common features like variables, control flow, and input and output handling. This website hosts an interpreter for the Core language.

## The Core Programming Language

Here is an example of a quick program that prints out N numbers of the fibonacci sequence.

```
program
  int N, A, B, C;
begin

  A = 0;
  B = 1;
  
  read N; // Fetch the # of fibs we want to print.
  
  while (N > 0) loop
  
    /*
     * This is a block comment. Stuff inside it is ignored.
     * An example of a line comment is shown above.
     */
  
    write A;
    
    C = A + B;
    A = B;
    B = C;
    
    N = N - 1;
    
  end;
end
```

We see that there are three keywords that encapsulate every program: `program`, `begin`, and `end`. Before we begin our program we must declare all variables that will be use; this creates space for them in memory. Once our program is inside the `begin` block, we can see examples of fetching input from the user, variable assignment, and writing output to the terminal. Note that assignments and control flow `end`s require semicolons.
