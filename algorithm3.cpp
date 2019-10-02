
#include <iostream>
#include <sstream>
#include <vector>

using namespace std;

void gen(vector<string> &result, int pos, int n, int open, int close);

void run(vector<string> &result, int n)
{
    if (n > 0)
        gen(result, 0, n, 0, 0);
    return;
}

void gen(vector<string> &result, int pos, int n, int open, int close)
{
    static char str[100];

    if (close == n)
    {
        result.push_back(str);
        // printf("%s \n", str);
        return;
    }
    else
    {
        if (open > close)
        {
            str[pos] = ')';
            gen(result, pos + 1, n, open, close + 1);
        }

        if (open < n)
        {
            str[pos] = '(';
            gen(result, pos + 1, n, open + 1, close);
        }
    }
}

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        cout << "you need specify number of parentheses pairs" << endl;
        return 1;
    }

    char *ptr;
    int n = strtol(argv[1], &ptr, 10);

    vector<string> result;

    run(result, n);

    cout << "count of generated variants is " << result.size() << endl;

    return 0;
}
