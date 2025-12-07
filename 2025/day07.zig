const std = @import("std");

const manifold = @embedFile("day07.txt");
const src = 'S';
const beam = '|';
const splitter = '^';

fn count_splits(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    const source_line = it.next().?;

    // TODO: maybe better to explicitly use bits instead? unsure how this will
    // be represented by zig
    var beams = try alloc.alloc(bool, source_line.len);
    defer alloc.free(beams);

    const prev_beams = try alloc.alloc(bool, source_line.len);
    defer alloc.free(prev_beams);

    const src_idx = std.mem.findScalar(u8, source_line, src).?;
    beams[src_idx] = true;

    var num_split: u64 = 0;
    while (it.next()) |line| {
        defer @memcpy(prev_beams, beams);
        num_split += split_beams(beams, prev_beams, line);
    }

    return num_split;
}

fn split_beams(beams: []bool, prev_beams: []const bool, line: []const u8) u64 {
    var num_split: u64 = 0;
    var start: usize = 0;
    while (std.mem.findScalarPos(u8, line, start, splitter)) |i| {
        defer start = i + 1;

        // no-op if no beam above
        if (!prev_beams[i]) continue;

        if (i - 1 >= 0) beams[i - 1] = true;
        if (i + 1 < line.len) beams[i + 1] = true;
        beams[i] = false;
        num_split += 1;
    }
    return num_split;
}

fn printBeams(beams: []const bool) void {
    for (beams) |b| {
        const c: u8 = if (b) '|' else '.';
        std.debug.print("{c}", .{c});
    }
    std.debug.print("\n", .{});
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
    try std.testing.expectEqual(21, count_splits(alloc, input));
}
