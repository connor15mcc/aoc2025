const std = @import("std");

const homework = @embedFile("day06.txt");

const Problem = struct {
    numbers: []u64,
    op: enum {
        Add,
        Mul,
    },

    const ProblemError = error{
        ParseError,
    };

    fn init(
        alloc: std.mem.Allocator,
        op_line: []const u8,
        num_lines: [][]const u8,
        col_i: usize,
        col_j: usize,
    ) !Problem {
        const op_str = std.mem.trim(u8, op_line[col_i..col_j], " ")[0];
        const op: @TypeOf(@as(Problem, undefined).op) = switch (op_str) {
            '+' => .Add,
            '*' => .Mul,
            else => {
                return error.ParseError;
            },
        };

        var numbers = try alloc.alloc(u64, num_lines.len);
        for (0.., num_lines) |i, line| {
            const clamped = line[col_i..col_j];
            const trimmed = std.mem.trim(u8, clamped, " ");
            const number = try std.fmt.parseInt(u64, trimmed, 10);
            numbers[i] = number;
        }

        return Problem{
            .numbers = numbers,
            .op = op,
        };
    }

    fn deinit(self: *Problem, allocator: std.mem.Allocator) void {
        allocator.free(self.numbers);
    }

    fn solve(self: Problem) u64 {
        switch (self.op) {
            .Add => {
                var sum: u64 = 0;
                for (self.numbers) |num| {
                    sum += num;
                }
                return sum;
            },
            .Mul => {
                var product: u64 = 1;
                for (self.numbers) |num| {
                    product *= num;
                }
                return product;
            },
        }
    }
};

fn sum_problems(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var lines = std.ArrayList([]const u8).empty;
    defer lines.deinit(alloc);

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        try lines.append(alloc, line);
    }

    const op_line = lines.items[lines.items.len - 1];
    const num_lines = lines.items[0 .. lines.items.len - 1];

    var grand_total: u64 = 0;
    var problem_start_idx: usize = 0;
    // iterate through bottom (op) line to split into problems & solve
    for (1.., op_line[1..]) |i, possible_op| {
        if (possible_op == ' ') continue;
        defer problem_start_idx = i;

        var problem = try Problem.init(
            alloc,
            op_line,
            num_lines,
            problem_start_idx,
            i - 1,
        );
        defer problem.deinit(alloc);

        grand_total += problem.solve();
    } else {
        // add last problem
        var problem = try Problem.init(
            alloc,
            op_line,
            num_lines,
            problem_start_idx,
            op_line.len,
        );
        defer problem.deinit(alloc);

        grand_total += problem.solve();
    }

    return grand_total;
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

    const soln = try sum_problems(alloc, homework);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\123 328  51 64 
        \\ 45 64  387 23 
        \\  6 98  215 314
        \\*   +   *   +  
    ;
    try std.testing.expectEqual(4277556, sum_problems(alloc, input));
}
