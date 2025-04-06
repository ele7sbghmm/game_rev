pub const RGB565 = packed struct {
    r: u5,
    g: u6,
    b: u5,
    pub fn parse(reader: anytype) !RGB565 {
        const raw = try reader.readInt(u16, .little);

        return RGB565{
            .r = @intCast(raw >> 11 & 0b0001_1111),
            .g = @intCast(raw >> 5 & 0b0011_1111),
            .b = @intCast(raw >> 0 & 0b0001_1111),
        };
    }
    pub fn print(self: RGB565) void {
        std.debug.print(".r {d} .g {d} .b {d}\n", .{ self.r, self.g, self.b });
    }
    pub fn toRGBA8888(self: RGB565) RGBA8888 {
        return RGBA8888{
            .r = @intCast(self.r),
            .g = @intCast(self.g),
            .b = @intCast(self.b),
            .a = 0xFF,
        };
    }
};
pub const RGBA8888 = packed struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    pub fn format(self: RGBA8888, allocator: std.mem.Allocator) ![]u8 {
        return try std.fmt.allocPrint(allocator, "{d} {d} {d} {d}", .{ self.r, self.g, self.b, self.a });
    }
    pub fn pack(self: RGBA8888) u32 {
        return @as(u32, self.r) |
            (@as(u32, self.g) << 8) |
            (@as(u32, self.b) << 16) |
            (@as(u32, self.a) << 24);
    }
    pub fn unpack(value: u32) RGBA8888 {
        return RGBA8888{
            .r = @intCast(value & 0xFF),
            .g = @intCast((value >> 8) & 0xFF),
            .b = @intCast((value >> 16) & 0xFF),
            .a = @intCast((value >> 24) & 0xFF),
        };
    }
};

fn unpack2Bits(n: u32) [16]u2 {
    var result: [16]u2 = undefined;
    for (0..16) |i| {
        result[i] = @intCast((n >> @intCast(2 * i)) & 0x3);
    }
    return result;
}

pub const BC1 = struct {
    c0: RGB565,
    c1: RGB565,
    c2: RGB565,
    c3: RGB565,
    lookupTable: [16]u2,

    pub fn parse(reader: anytype) !BC1 {
        const c0 = try RGB565.parse(reader);
        const c1 = try RGB565.parse(reader);
        var c2: RGB565 = undefined;
        var c3: RGB565 = undefined;

        const lookupTableU32 = try reader.readInt(u32, .little);
        const lookupTable = unpack2Bits(lookupTableU32);

        if (c1.r >= c0.r and c1.g >= c0.g and c1.b >= c0.b) {
            c2 = RGB565{ .r = c0.r / 2 + c1.r / 2, .g = c0.g / 2 + c1.g / 2, .b = c0.b / 2 + c1.b / 2 };
            c3 = RGB565{ .r = 0, .g = 0, .b = 0 };
        } else {
            c2 = RGB565{ .r = c0.r / 3 * 2 + c1.r / 3, .g = c0.g / 3 * 2 + c1.g / 3, .b = c0.b / 3 * 2 + c1.b / 3 };
            c3 = RGB565{ .r = c0.r / 3 + c1.r / 3 * 2, .g = c0.g / 3 + c1.g / 3 * 2, .b = c0.b / 3 + c1.b / 3 * 2 };
        }
        return BC1{ .c0 = c0, .c1 = c1, .c2 = c2, .c3 = c3, .lookupTable = lookupTable };
    }
    pub fn print(self: BC1) void {
        std.debug.print("c0: {d} {d} {d}\n", .{ self.c0.r, self.c0.g, self.c0.b });
        std.debug.print("c1: {d} {d} {d}\n", .{ self.c1.r, self.c1.g, self.c1.b });
        std.debug.print("c2: {d} {d} {d}\n", .{ self.c2.r, self.c2.g, self.c2.b });
        std.debug.print("c3: {d} {d} {d}\n", .{ self.c3.r, self.c3.g, self.c3.b });
        std.debug.print("lookupTable: {any}\n", .{self.lookupTable});
    }
    pub fn toPixels(self: BC1) ![4][4]RGBA8888 {
        var result: [4][4]RGBA8888 = undefined;
        var table = [4]RGB565{ self.c0, self.c1, self.c2, self.c3 };
        for (0..4) |y| {
            for (0..4) |x| {
                result[y][x] = table[self.lookupTable[y * 4 + x]].toRGBA8888();
            }
        }
        return result;
    }
    pub fn formatRGBATableRow(allocator: std.mem.Allocator, row: [4]RGBA8888) ![]u8 {
        const c0 = try std.fmt.allocPrint(allocator, "{d} {d} {d}", .{ row[0].r, row[0].g, row[0].b });
        const c1 = try std.fmt.allocPrint(allocator, "{d} {d} {d}", .{ row[1].r, row[1].g, row[1].b });
        const c2 = try std.fmt.allocPrint(allocator, "{d} {d} {d}", .{ row[2].r, row[2].g, row[2].b });
        const c3 = try std.fmt.allocPrint(allocator, "{d} {d} {d}", .{ row[3].r, row[3].g, row[3].b });
        return try std.fmt.allocPrint(allocator, "{s} {s} {s} {s} ", .{ c0, c1, c2, c3 });
    }
    pub fn parseUntilEOF(allocator: std.mem.Allocator, reader: anytype) !std.ArrayList(BC1) {
        var blocks = std.ArrayList(BC1).init(allocator);
        errdefer blocks.deinit();

        while (true) {
            const block = BC1.parse(reader) catch |err| {
                if (err == error.EndOfStream) break;
                return err;
            };
            try blocks.append(block);
        }

        return blocks;
    }
};
pub const BC2 = struct {
    // DONT BE LAZY
    alpha: [16]u4,
    bc1: BC1,
    pub fn toPixels(self: BC2) ![4][4]RGBA8888 {
        var pixels = try self.bc1.toPixels();
        for (0..16) |i| {
            pixels[i / 4][i % 4].a = self.alpha[i];
        }
        return pixels;
    }
    pub fn parse(reader: anytype) !BC2 {
        // Read 64 bits (8 bytes) of alpha data as a single u64
        const alpha_data = try reader.readInt(u64, .little);
        var alpha: [16]u4 = undefined;

        // Unpack 4 bits at a time from the u64
        for (0..16) |i| {
            alpha[i] = @intCast((alpha_data >> @intCast(4 * i)) & 0xF);
        }

        const bc1 = try BC1.parse(reader);
        return BC2{ .alpha = alpha, .bc1 = bc1 };
    }
    pub fn parseUntilEOF(allocator: std.mem.Allocator, reader: anytype) !std.ArrayList(BC2) {
        var blocks = std.ArrayList(BC2).init(allocator);
        errdefer blocks.deinit();

        while (true) {
            const block = BC2.parse(reader) catch |err| {
                if (err == error.EndOfStream) break;
                return err;
            };
            try blocks.append(block);
        }

        return blocks;
    }
};

const std = @import("std");
