const std = @import("std");

const manifold = @embedFile("day07.txt");
const src = 'S';
const beam = '|';
const splitter = '^';

fn count_splits(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var front_it = std.mem.tokenizeScalar(u8, input, '\n');
    const src_line = front_it.next().?;

    var counts = try alloc.alloc(u64, src_line.len);
    defer alloc.free(counts);
    @memset(counts, 1);

    var back_it = std.mem.splitBackwardsScalar(u8, input, '\n');
    while (back_it.next()) |line| {
        for (0.., line) |idx, char| {
            if (char == splitter) {
                counts[idx] = counts[idx - 1] + counts[idx + 1];
            }
        }
    }

    const src_idx = std.mem.findScalar(u8, src_line, src).?;
    return counts[src_idx];
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

    const soln = try count_splits(alloc, manifold);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\.......S.......
        \\...............
        \\.......^.......
        \\...............
        \\......^.^......
        \\...............
        \\.....^.^.^.....
        \\...............
        \\....^.^...^....
        \\...............
        \\...^.^...^.^...
        \\...............
        \\..^...^.....^..
        \\...............
        \\.^.^.^.^.^...^.
        \\...............
    ;
    try std.testing.expectEqual(40, count_splits(alloc, input));
}
