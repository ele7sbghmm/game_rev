pub fn main() !void {
    const allocator = std.heap.page_allocator;

    const file = try std.fs.cwd().openFile("/Users/im/Desktop/NZ1.pgf", .{});
    const texture = try bc.parsePgfXpr(allocator, file);
    _ = texture;
}

const std = @import("std");
const bc = @import("bc.zig");
const xpr = @import("xpr.zig");
const ppm = @import("ppm.zig");
