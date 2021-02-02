/**
 * Copyright (c) woodser
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 */

#include <stdio.h>
#include <iostream>
#include "atomic_swap_utils.h"
#include "atomic_swap_rust_bridge.h"

void atomic_swap_utils::say_hello() {
  std::cout << "HELLO WORLD!" << std::endl;

  // invoke rust functions
  greetings();
  int sum = add(3, 5);
  std::cout << "SUM FROM RUST: " << sum << std::endl;
}
