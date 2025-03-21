let fibonacci = fun(x) {
  if (x <= 1) {
    x;
  } else {
      fibonacci(x - 1) + fibonacci(x - 2);
  }
};

fibonacci(30);
