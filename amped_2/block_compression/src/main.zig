pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // const file = try std.fs.cwd().openFile("data/nz1/thirdxpr0.pgf", .{});
    const file = try std.fs.cwd().openFile("data/nz1/splash_eng.pgf", .{});
    const texture = try bc.parsePgfXpr(allocator, file);
    _ = texture;
}

const std = @import("std");
const bc = @import("bc.zig");
const xpr = @import("xpr.zig");
const ppm = @import("ppm.zig");
