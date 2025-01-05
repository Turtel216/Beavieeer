let fold = fun(f, init, lst) {
  if (len(lst) == 0) {
    init
  } else {
    let newInit = f(init, first(lst));
    fold(f, newInit, rest(lst));
  }
};
