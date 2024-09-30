#include "cpp_rust_bridge/lib.h"
#include <iostream>
#include <format>
#include <numbers>

int main()
{
    std::cout << "Reading rows from database...\n";

    const auto results = read_all_rows_ffi();

    std::cout << std::format("{} rows found.\n", results.size());

    for (const auto &r : results)
    {
        std::cout << std::format("Id: {}, Info: {}, Time: {}, Data: {}\n",
                                 r.id,
                                 std::string(r.info),
                                 std::string(r.time),
                                 std::string(r.data));
    }

    return 0;
}
