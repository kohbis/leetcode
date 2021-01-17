#include <limits.h>

int reverse(int x){
    long long res = 0;

    while(x != 0)
    {
        res = res * 10 + x % 10;
        x /= 10;
    }

    return (res > INT_MAX || INT_MIN > res) ? 0 : res;
}
