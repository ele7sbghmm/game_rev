pub const Ppm3 = struct {
    allocator: std.mem.Allocator,
    width: u32,
    height: u32,
    max_val: u32,
    pixels: std.ArrayList(u8),
    pub fn init(allocator: std.mem.Allocator, width: u32, height: u32, max_val: u32) !Ppm3 {
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
    pub fn fmt(self: Ppm3) ![]u8 {
        var result = std.ArrayList(u8).init(self.allocator);
        defer result.deinit();

        try result.appendSlice("P3\n");
        try result.appendSlice(try std.fmt.allocPrint(self.allocator, "{d} {d}\n{d}\n", .{ self.width, self.height, self.max_val }) catch return error.OutOfMemory);

        for (self.pixels.items, 0..) |item, i| {
            if (i > 0) try result.append(' ');
            try result.appendSlice(try std.fmt.allocPrint(self.allocator, "{d}", .{item}) catch return error.OutOfMemory);
        }

        try result.append(' ');

        return result.toOwnedSlice();
    }
    pub fn fillPixels(self: Ppm3, rgbaArray: []bc.R8G8B8A8) void {
        self.pixels = try self.allocator.alloc(u8, rgbaArray.len * 3);
        for (0..rgbaArray.len) |i| {
            self.pixels[i * 3 + 0] = rgbaArray[i].r * 8;
            self.pixels[i * 3 + 1] = rgbaArray[i].g * 4;
            self.pixels[i * 3 + 2] = rgbaArray[i].b * 8;
        }
    }
};
const TUPLTYPE = enum {
    RGB_ALPHA,
    RGB,
    GRAYSCALE,
    pub fn toString(self: TUPLTYPE) []const u8 {
        return switch (self) {
            .RGB_ALPHA => "RGB_ALPHA",
            .RGB => "RGB",
            .GRAYSCALE => "GRAYSCALE",
        };
    }
};
pub const Ppm7 = struct {
    allocator: std.mem.Allocator,
    width: u32,
    height: u32,
    depth: u32 = 4,
    max_val: u32 = 255,
    typltype: TUPLTYPE = .RGB_ALPHA,
    pixels: std.ArrayList(u32),
    pub fn init(allocator: std.mem.Allocator, width: u32, height: u32) !Ppm7 {
        // const pixels = try allocator.alloc(u8, width * height * 3);
        const pixels = std.ArrayList(u32).init(allocator);
        return Ppm7{ .allocator = allocator, .width = width, .height = height, .pixels = pixels };
    }
    pub fn deinit(self: Ppm7) void {
        self.pixels.deinit();
    }
    pub fn fmtHeader(self: Ppm7) ![]u8 {
        var result = std.ArrayList(u8).init(self.allocator);
        defer result.deinit();

        try result.appendSlice(try std.fmt.allocPrint(self.allocator, "P7\nWIDTH {d}\nHEIGHT {d}\nDEPTH {d}\nMAXVAL {d}\nTUPLTYPE {s}\nENDHDR\n", .{ self.width, self.height, self.depth, self.max_val, self.typltype.toString() }));

        return result.toOwnedSlice();
    }
    pub fn write(self: Ppm7, path: []const u8, pixels: std.ArrayList(u8)) !void {
        const output_filename = std.fmt.allocPrint(self.allocator, "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/ppm/{s}.ppm", .{path}) catch return error.OutOfMemory;
        defer self.allocator.free(output_filename);

        const output_file = try std.fs.cwd().createFile(output_filename, .{});
        defer output_file.close();

        try output_file.writeAll(try self.fmtHeader());
        try output_file.writeAll(pixels.items);
    }
};

const std = @import("std");
const bc = @import("bc.zig");
