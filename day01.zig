const std = @import("std");

const instructions = @embedFile("day01-part1.txt");

fn password(alloc: std.mem.Allocator, input: []const u8) !u32 {
    var dial: i32 = 50;
    var countAtZero: u32 = 0;

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const mut_line = try alloc.dupe(u8, line);
        defer alloc.free(mut_line);

        std.mem.replaceScalar(u8, mut_line, 'L', '-');
        std.mem.replaceScalar(u8, mut_line, 'R', '+');
        const trimmed = std.mem.trim(u8, mut_line, " ");

        const rot = try std.fmt.parseInt(i32, trimmed, 10);
        dial = try std.math.mod(i32, dial + rot, 100);
        if (dial == 0) {
            countAtZero += 1;
        }
    }

    return countAtZero;
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

    const soln = try password(alloc, instructions);
    std.debug.print("{d}\n", .{soln});
}

test "the provided example" {
    const alloc = std.testing.allocator;

    const input =
        \\ L68
        \\ L30
        \\ R48
        \\ L5
        \\ R60
        \\ L55
        \\ L1
        \\ L99
        \\ R14
        \\ L82
    ;

    const result = try password(alloc, input);
    try std.testing.expectEqual(result, @as(u32, 3));
}
