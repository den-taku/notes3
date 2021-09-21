#include <vector>
#include <algorithm>
#include <iostream>
#include <iomanip>
#include <stdio.h>

int main() {
    std::cout << ((0.6 - 0.0) <= 0.6) << std::endl;
    std::cout << ((1.6 - 1.0) <= 0.6) << std::endl;
    std::cout << std::fixed << std::setprecision(50) << (1.6 - 1.0)  << std::endl;
    std::cout << ((2.6 - 2.0) <= 0.6) << std::endl;
    std::cout << std::fixed << std::setprecision(30) << (2.6 - 2.0)  << std::endl;
    std::cout << ((3.6 - 3.0) <= 0.6) << std::endl;
    std::cout << ((4.6 - 4.0) <= 0.6) << std::endl;
    std::cout << std::fixed << std::setprecision(30) << (4.6 - 4.0)  << std::endl;
    std::cout << ((5.6 - 5.0) <= 0.6) << std::endl;
    std::cout << ((6.6 - 6.0) <= 0.6) << std::endl;
    std::cout << ((7.6 - 7.0) <= 0.6) << std::endl;
    std::cout << ((8.6 - 8.0) <= 0.6) << std::endl;
    std::cout << ((9.6 - 9.0) <= 0.6) << std::endl;
    std::cout << ((10.6 - 10.0) <= 0.6) << std::endl;
    std::cout << ((11.6 - 11.0) <= 0.6) << std::endl;
    std::cout << ((12.6 - 12.0) <= 0.6) << std::endl;
    std::cout << ((13.6 - 13.0) <= 0.6) << std::endl;
    std::cout << ((14.6 - 14.0) <= 0.6) << std::endl;
    std::cout << ((15.6 - 15.0) <= 0.6) << std::endl;
    std::cout << ((16.6 - 16.0) <= 0.6) << std::endl;
    std::cout << std::fixed << std::setprecision(30) << (16.6 - 16.0)  << std::endl;

    std::vector<int> v = {8, 7, 6, 5, 4, 3, 2, 1};

    std::cout << std::upper_bound(v.rbegin(), v.rend(), 5) - v.rbegin() << std::endl;
    std::cout << std::upper_bound(v.rbegin(), v.rend(), 7) - v.rbegin() << std::endl;
    std::cout << std::upper_bound(v.rbegin(), v.rend(), 8) - v.rbegin() << std::endl;
    std::cout << std::upper_bound(v.rbegin(), v.rend(), 9) - v.rend() << std::endl;

    std::vector<int> u = {};
    std::cout << std::upper_bound(u.rbegin(), u.rend(), 5) - u.rbegin() << std::endl;
    std::cout << std::upper_bound(u.rbegin(), u.rend(), 7) - u.rbegin() << std::endl;
    std::cout << std::upper_bound(u.rbegin(), u.rend(), 8) - u.rbegin() << std::endl;
    std::cout << std::upper_bound(u.rbegin(), u.rend(), 9) - u.rend() << std::endl;

    if (3 > 4) {
        goto tets;
    }
    std::cout << "false" << std::endl;

tets:
    std::cout << "end" << std::endl;
}
