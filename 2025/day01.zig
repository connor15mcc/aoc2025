const std = @import("std");

const instructions = @embedFile("day01-part1.txt");

const Safe = struct {
    dial_size: i32,
    dial_pos: i32 = 0,
    times_past_zero: i32 = 0,

    fn rotate(self: *Safe, rot: i32) !void {
        const next: i32 = self.dial_pos + rot;

        // adjust for backwards rotations from zero
        if (next <= 0 and next != rot) {
            self.times_past_zero += 1;
        }

        self.times_past_zero += @divFloor(@as(i32, @intCast(@abs(next))), self.dial_size);
        self.dial_pos = @mod(next, self.dial_size);
    }
};

fn password(alloc: std.mem.Allocator, input: []const u8) !i32 {
    var safe = Safe{ .dial_size = 100, .dial_pos = 50 };

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const mut_line = try alloc.dupe(u8, line);
        defer alloc.free(mut_line);

        std.mem.replaceScalar(u8, mut_line, 'L', '-');
        std.mem.replaceScalar(u8, mut_line, 'R', '+');
        const trimmed = std.mem.trim(u8, mut_line, " ");

        const rot = try std.fmt.parseInt(i32, trimmed, 10);
        try safe.rotate(rot);
    }

    return safe.times_past_zero;
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
    try std.testing.expectEqual(6, result);
}
