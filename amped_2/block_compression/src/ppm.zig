pub const Ppm3 = struct {
    allocator: std.mem.Allocator,
    width: u32,
    height: u32,
    max_val: u32,
    pixels: std.ArrayList(u8),
    pub fn init(allocator: std.mem.Allocator, width: u32, height: u32, max_val: u32) !Ppm3 {
        // const pixels = try allocator.alloc(u8, width * height * 3);
        const pixels = std.ArrayList(u8).init(allocator);
        return Ppm3{ .allocator = allocator, .width = width, .height = height, .max_val = max_val, .pixels = pixels };
    }
    pub fn deinit(self: Ppm3) void {
        self.pixels.deinit();
    }
    pub fn print(self: Ppm3) void {
        std.debug.print("P3\n{d} {d}\n{d}\n", .{ self.width, self.height, self.max_val });
        std.debug.print("{any}\n", .{self.pixels.items});
    }
    pub fn format(self: Ppm3) ![]u8 {
        var result = std.ArrayList(u8).init(self.allocator);
        defer result.deinit();

        try result.appendSlice("P3\n");
        try result.appendSlice(std.fmt.allocPrint(self.allocator, "{d} {d}\n{d}\n", .{ self.width, self.height, self.max_val }) catch return error.OutOfMemory);

        for (self.pixels.items, 0..) |item, i| {
            if (i > 0) try result.append(' ');
            try result.appendSlice(std.fmt.allocPrint(self.allocator, "{d}", .{item}) catch return error.OutOfMemory);
        }

        try result.append(' ');

        return result.toOwnedSlice();
    }
    pub fn fillPixels(self: Ppm3, rgbaArray: []bc1.RGBA8888) void {
        self.pixels = try self.allocator.alloc(u8, rgbaArray.len * 3);
        for (0..rgbaArray.len) |i| {
            self.pixels[i * 3 + 0] = rgbaArray[i].r * 8;
            self.pixels[i * 3 + 1] = rgbaArray[i].g * 4;
            self.pixels[i * 3 + 2] = rgbaArray[i].b * 8;
        }
    }
};

const std = @import("std");
const bc1 = @import("bc.zig");
