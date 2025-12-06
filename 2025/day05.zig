const std = @import("std");

const database = @embedFile("day05.txt");

const FreshIdRange = struct {
    low: usize,
    high: usize,

    fn cmpByLow(_: void, a: FreshIdRange, b: FreshIdRange) bool {
        return a.low < b.low;
    }
};

fn count_fresh(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var fresh_ranges = std.ArrayList(FreshIdRange).empty;
    defer fresh_ranges.deinit(alloc);

    var it = std.mem.splitScalar(u8, input, '\n');
    while (it.next()) |line| {
        // blank line separating fresh ranges and available ingredients
        if (line.len == 0) break;

        var rangeIt = std.mem.tokenizeScalar(u8, line, '-');
        const low = rangeIt.next().?;
        const high = rangeIt.next().?;
        std.debug.assert(rangeIt.next() == null);

        try fresh_ranges.append(alloc, .{
            .low = try std.fmt.parseInt(usize, low, 10),
            .high = try std.fmt.parseInt(usize, high, 10),
        });
    }
    std.mem.sortUnstable(FreshIdRange, fresh_ranges.items, {}, FreshIdRange.cmpByLow);

    var num_fresh: u64 = 0;
    while (it.next()) |line| {
        if (line.len == 0) break;

        const food_id = try std.fmt.parseInt(usize, line, 10);
        if (isFresh(fresh_ranges.items, food_id)) num_fresh += 1;
    }

    return num_fresh;
}

fn isFresh(ranges: []const FreshIdRange, food_id: usize) bool {
    for (ranges) |range| {
        if (food_id >= range.low and food_id <= range.high) {
            return true;
        }
    }
    return false;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {
            @panic("Memory leak detected!");
        }
    }
    const alloc = gpa.allocator();

    const soln = try count_fresh(alloc, database);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\3-5
        \\10-14
        \\16-20
        \\12-18
        \\
        \\1
        \\5
        \\8
        \\11
        \\17
        \\32
    ;
    try std.testing.expectEqual(3, count_fresh(alloc, input));
}
