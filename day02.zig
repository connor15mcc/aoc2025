const std = @import("std");

const database = @embedFile("day02-part1.txt");

fn solve(input: []const u8) !usize {
    var sum: usize = 0;

    const trimmed = std.mem.trim(u8, input, "\n");
    var rangesIter = std.mem.tokenizeScalar(u8, trimmed, ',');
    while (rangesIter.next()) |range| {
        // std.debug.print("range: {s}\n", .{range});
        var idsIter = std.mem.tokenizeScalar(u8, range, '-');
        const first = idsIter.next().?;
        const second = idsIter.next().?;
        std.debug.assert(idsIter.next() == null);

        const firstID = try std.fmt.parseInt(usize, first, 10);
        const secondID = try std.fmt.parseInt(usize, second, 10);

        for (firstID..secondID + 1) |id| {
            if (!try valid_id(id)) {
                // std.debug.print("  invalid: {d}\n", .{id});
                sum += id;
            }
        }
    }

    return sum;
}

fn valid_id(id: usize) !bool {
    var buf: [20]u8 = undefined;
    const id_str = try std.fmt.bufPrint(&buf, "{}", .{id});

    if (@mod(id_str.len, 2) != 0) {
        return true;
    }

    const mid = @divFloor(id_str.len, 2);
    return !std.mem.eql(u8, id_str[0..mid], id_str[mid..]);
}

pub fn main() !void {
    const soln = try solve(database);
    std.debug.print("{d}\n", .{soln});
}

test "provided example (pt1)" {
    const input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    try std.testing.expectEqual(4174379265, solve(input));
}
