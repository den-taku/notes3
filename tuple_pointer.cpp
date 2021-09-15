#include <iostream>
#include <tuple>
#include <list>

typedef long long integer;

typedef enum Kind {
    Incoming,
    Retrieve,
    Relocate,
    Nope,
} Kind;

typedef std::tuple<std::list<Kind>, std::list<Kind>, std::list<Kind>, integer, integer> Operations;

int main () {
    Operations o;
    std::cout << std::get<3>(o) << std::endl;
}

