#include <mutex>

using namespace std;

class Foo {
    mutex mtx_second;
    mutex mtx_third;

  public:
    Foo() {
        mtx_second.lock();
        mtx_third.lock();
    }

    void first(function<void()> printFirst) {

        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        mtx_second.unlock();
    }

    void second(function<void()> printSecond) {
        mtx_second.lock();
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        mtx_third.unlock();
    }

    void third(function<void()> printThird) {
        mtx_third.lock();
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
    }
};
