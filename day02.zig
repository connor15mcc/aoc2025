const std = @import("std");

const database = @embedFile("day02-part1.txt");

fn solve(alloc: std.mem.Allocator, input: []const u8) !usize {
    var sum: usize = 0;

    const trimmed = std.mem.trim(u8, input, "\n");
    var rangesIter = std.mem.tokenizeScalar(u8, trimmed, ',');
    while (rangesIter.next()) |range| {
        std.debug.print("range: {s}\n", .{range});
        var idsIter = std.mem.tokenizeScalar(u8, range, '-');
        const first = idsIter.next().?;
        const second = idsIter.next().?;
        std.debug.assert(idsIter.next() == null);

        const firstID = try std.fmt.parseInt(usize, first, 10);
        const secondID = try std.fmt.parseInt(usize, second, 10);

        for (firstID..secondID + 1) |id| {
            if (!try valid_id(alloc, id)) {
                std.debug.print("  invalid: {d}\n", .{id});
                sum += id;
            }
        }
    }

    return sum;
}

fn valid_id(alloc: std.mem.Allocator, id: usize) !bool {
    var buf: [20]u8 = undefined;
    const id_str = try std.fmt.bufPrint(&buf, "{}", .{id});

    if (id_str.len == 1) {
        return true;
    }

    const d = try divisors(alloc, id_str.len);
    defer alloc.free(d);
    for (d) |i| {
        if (i == id_str.len) {
            continue;
        }

        const seq = id_str[0..i];

        // replace seq in id_str throughout
        const replaced = try std.mem.replaceOwned(u8, alloc, id_str, seq, "");
        defer alloc.free(replaced);

        if (std.mem.eql(u8, replaced, "")) {
            return false;
        }
    }

    return true;
}

fn divisors(alloc: std.mem.Allocator, n: usize) ![]usize {
    var d = std.ArrayList(usize).empty;
    defer d.deinit(alloc);

    for (1..std.math.sqrt(n) + 1) |i| {
        if (try std.math.mod(usize, n, i) != 0) {
            continue;
        }

        if (n / i == i) {
            try d.append(alloc, i);
        } else {
            try d.append(alloc, i);
            try d.append(alloc, n / i);
        }
    }

    return d.toOwnedSlice(alloc);
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

    const soln = try solve(alloc, database);
    std.debug.print("{d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    try std.testing.expectEqual(4174379265, solve(alloc, input));
}

test "simple divisors" {
    const alloc = std.testing.allocator;
    const result = try divisors(alloc, 8);
    defer alloc.free(result);

    try std.testing.expectEqualSlices(usize, &[_]usize{ 1, 8, 2, 4 }, result);
}

test "real divisor" {
    const alloc = std.testing.allocator;
    const result = try divisors(alloc, 10);
    defer alloc.free(result);

    try std.testing.expectEqualSlices(usize, &[_]usize{ 1, 10, 2, 5 }, result);
}
