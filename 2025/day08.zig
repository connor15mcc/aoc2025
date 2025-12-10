const std = @import("std");

const junctionBoxes = @embedFile("day08.txt");

const JunctionBox = struct {
    x: i64,
    y: i64,
    z: i64,

    fn parse(str: []const u8) !JunctionBox {
        var it = std.mem.tokenizeScalar(u8, str, ',');
        const x_str = it.next().?;
        const y_str = it.next().?;
        const z_str = it.next().?;
        return .{
            .x = try std.fmt.parseInt(i64, x_str, 10),
            .y = try std.fmt.parseInt(i64, y_str, 10),
            .z = try std.fmt.parseInt(i64, z_str, 10),
        };
    }

    fn distance_squared(self: *JunctionBox, other: JunctionBox) u64 {
        const dx = self.x - other.x;
        const dy = self.y - other.y;
        const dz = self.z - other.z;
        return @intCast(dx * dx + dy * dy + dz * dz);
    }
};

const StringLight = struct {
    i: usize,
    j: usize,
    distance_squared: u64,

    fn init(i: usize, j: usize, distance_squared: usize) StringLight {
        return .{
            .i = i,
            .j = j,
            .distance_squared = distance_squared,
        };
    }

    fn cmp(_: void, a: StringLight, b: StringLight) bool {
        return a.distance_squared < b.distance_squared;
    }
};

const Circuit = struct {
    parent: []usize,
    size: []usize,

    fn init(alloc: std.mem.Allocator, n: usize) !Circuit {
        const parent = try alloc.alloc(usize, n);
        const size = try alloc.alloc(usize, n);
        for (0..n) |i| {
            parent[i] = i;
            size[i] = 1;
        }

        return .{
            .parent = parent,
            .size = size,
        };
    }

    fn deinit(self: *Circuit, alloc: std.mem.Allocator) void {
        alloc.free(self.parent);
        alloc.free(self.size);
    }

    fn find(self: *Circuit, i: usize) usize {
        if (self.parent[i] != i) {
            self.parent[i] = self.find(self.parent[i]);
        }
        return self.parent[i];
    }

    fn unite(self: *Circuit, i: usize, j: usize) void {
        var root_i = self.find(i);
        var root_j = self.find(j);
        if (root_i == root_j) return;

        if (self.size[root_i] < self.size[root_j]) {
            const tmp = root_i;
            root_i = root_j;
            root_j = tmp;
        }

        self.parent[root_j] = root_i;
        self.size[root_i] += self.size[root_j];
    }
};

fn sumCircuits(alloc: std.mem.Allocator, input: []const u8, limit: u16) !u64 {
    var boxes = std.ArrayList(JunctionBox).empty;
    defer boxes.deinit(alloc);

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        try boxes.append(alloc, try JunctionBox.parse(line));
    }
    const n = boxes.items.len;

    var lights = std.ArrayList(StringLight).empty;
    defer lights.deinit(alloc);

    for (0..n) |i| {
        for (i + 1..n) |j| {
            const d2 = boxes.items[i].distance_squared(boxes.items[j]);
            try lights.append(alloc, StringLight.init(i, j, d2));
        }
    }
    std.mem.sortUnstable(StringLight, lights.items, {}, StringLight.cmp);
    try lights.resize(alloc, limit);

    var circuits = try Circuit.init(alloc, boxes.items.len);
    defer circuits.deinit(alloc);

    for (lights.items) |light| {
        circuits.unite(light.i, light.j);
    }

    var sizes = std.ArrayList(usize).empty;
    defer sizes.deinit(alloc);
    for (0.., circuits.parent) |i, parent| {
        if (i == parent) try sizes.append(alloc, circuits.size[i]);
    }
    std.mem.sortUnstable(usize, sizes.items, {}, std.sort.desc(usize));

    return sizes.items[0] * sizes.items[1] * sizes.items[2];
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

    const soln = try sumCircuits(alloc, junctionBoxes, 1000);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\162,817,812
        \\57,618,57
        \\906,360,560
        \\592,479,940
        \\352,342,300
        \\466,668,158
        \\542,29,236
        \\431,825,988
        \\739,650,466
        \\52,470,668
        \\216,146,977
        \\819,987,18
        \\117,168,530
        \\805,96,715
        \\346,949,466
        \\970,615,88
        \\941,993,340
        \\862,61,35
        \\984,92,344
        \\425,690,689
    ;
    try std.testing.expectEqual(40, sumCircuits(alloc, input, 10));
}
