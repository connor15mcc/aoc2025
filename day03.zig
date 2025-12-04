const std = @import("std");

const batteries = @embedFile("day03.txt");

fn max_joltage(input: []const u8) !usize {
    var joltage: usize = 0;

    const trimmed = std.mem.trim(u8, input, "\n");
    var banksIter = std.mem.tokenizeScalar(u8, trimmed, '\n');
    while (banksIter.next()) |bank| {
        const max_idx = std.mem.indexOfMax(u8, bank[0 .. bank.len - 1]);
        const max_battery_s = bank[max_idx];
        const next_battery_s = std.mem.max(u8, bank[max_idx + 1 ..]);

        joltage += (max_battery_s - '0') * 10 + (next_battery_s - '0');
    }

    return joltage;
}

pub fn main() !void {
    const soln = try max_joltage(batteries);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const input =
        \\ 987654321111111
        \\ 811111111111119
        \\ 234234234234278
        \\ 818181911112111
    ;

    try std.testing.expectEqual(357, max_joltage(input));
}
