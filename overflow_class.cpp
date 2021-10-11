#ifndef MAIN
#define MAIN

#include <iostream>
#include "class.h"

class B;

class A {
    public:
        A(){};
    private:
        B b;
};

int main() {
    std::cout << "Hello, World!" << std::endl;
}
#endif
