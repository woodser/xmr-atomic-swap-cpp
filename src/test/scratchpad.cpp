#include <stdio.h>
#include <iostream>
#include "atomic_swap_utils.h"

using namespace std;

/**
 * Scratchpad main entry point.
 */
int main(int argc, const char* argv[]) {

  // print header
  std::cout << "===== Scratchpad =====" << std::endl;
  for (int i = 0; i < argc; i++) {
    std::cout << "Argument" << i << ": " << argv[i] << std::endl;
  }

  // demonstrate atomic swap utils
  atomic_swap_utils::say_hello();
}
