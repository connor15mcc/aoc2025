const std = @import("std");

const positions = @embedFile("day04.txt");

const dirs = [_][2]i32{
    .{ -1, -1 }, .{ -1, 0 }, .{ -1, 1 },
    .{ 0, -1 },  .{ 0, 1 },  .{ 1, -1 },
    .{ 1, 0 },   .{ 1, 1 },
};

fn accessible_paper(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var grid = std.ArrayList([]u8).empty;
    defer grid.deinit(alloc);

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const owned = try alloc.dupe(u8, line);
        try grid.append(alloc, owned);
    }

    const grid_owned = try grid.toOwnedSlice(alloc);
    defer {
        for (grid_owned) |line| {
            alloc.free(line);
        }
        alloc.free(grid_owned);
    }

    var total_removed: u64 = 0;
    while (try remove_accessible(grid_owned)) |removed| {
        total_removed += removed;
    }
    return total_removed;
}

fn remove_accessible(grid: [][]u8) !?u64 {
    var accessible: u64 = 0;

    const rows = grid.len;
    std.debug.assert(rows > 0);
    const cols = grid[0].len;

    for (0.., grid) |i, row| {
        for (0.., row) |j, cell| {
            if (cell != '@') continue;

            var num_neighbors: usize = 0;
            for (dirs) |dir| {
                const ni = @as(i32, @intCast(i)) + dir[0];
                const nj = @as(i32, @intCast(j)) + dir[1];

                if (ni < 0 or ni >= rows or nj < 0 or nj >= cols) continue;

                const ni_usize = @as(usize, @intCast(ni));
                const nj_usize = @as(usize, @intCast(nj));
                if (grid[ni_usize][nj_usize] == '@') {
                    num_neighbors += 1;
                }
            }

            if (num_neighbors < 4) {
                accessible += 1;
                grid[i][j] = '.';
            }
        }
    }
    if (accessible == 0) return null;
    return accessible;
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

    const soln = try accessible_paper(alloc, positions);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;
    try std.testing.expectEqual(43, accessible_paper(alloc, input));
}
