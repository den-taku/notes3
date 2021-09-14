#include <queue>
#include <vector>
#include <iostream>

int main() {
    std::priority_queue<
        int,
        std::vector<int>,
        std::greater<int>>
        priority_queue;
    priority_queue.push(3);
    priority_queue.push(4);
    priority_queue.push(5);
    std::cout << priority_queue.top() << std::endl;
}