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
    pub fn fmt(self: RGB565, allocator: std.mem.Allocator) void {
        return std.fmt.allocPrint(allocator, ".r {d} .g {d} .b {d}\n", .{ self.r, self.g, self.b });
    }
    pub fn toR8G8B8A8(self: RGB565) R8G8B8A8 {
        return R8G8B8A8{
            .r = @as(u8, self.r) * 8,
            .g = @as(u8, self.g) * 4,
            .b = @as(u8, self.b) * 8,
            .a = 0xFF,
        };
    }
};
pub const A1R5G5B5 = packed struct {
    r: u5,
    g: u5,
    b: u5,
    a: u1,
    pub fn parse(reader: anytype) !A1R5G5B5 {
        const raw = try reader.readInt(u16, .little);
        return A1R5G5B5{
            .a = @as(u1, raw >> 15 & 0b1),
            .r = @as(u5, raw >> 10 & 0b0001_1111),
            .g = @as(u5, raw >> 5 & 0b0001_1111),
            .b = @as(u5, raw >> 0 & 0b0001_1111),
        };
    }
    pub fn fmt(self: A1R5G5B5, allocator: std.mem.Allocator) void {
        return std.fmt.allocPrint(allocator, ".a {d} .r {d} .g {d} .b {d}\n", .{ self.a, self.r, self.g, self.b });
    }
    pub fn toR8G8B8A8(self: A1R5G5B5) R8G8B8A8 {
        return R8G8B8A8{
            .r = @as(u8, self.r) * 8,
            .g = @as(u8, self.g) * 8,
            .b = @as(u8, self.b) * 8,
            .a = if (self.a > 0) 0xff else 0,
        };
    }
};
pub const A8R8G8B8 = struct {
    pub fn parse(reader: anytype) !R8G8B8A8 {
        return R8G8B8A8{
            .a = try reader.readInt(u8, .little),
            .r = try reader.readInt(u8, .little),
            .g = try reader.readInt(u8, .little),
            .b = try reader.readInt(u8, .little),
        };
    }
};
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

        var r: u32 = undefined;
        var g: u32 = undefined;
        var b: u32 = undefined;
        if (c1.r >= c0.r and c1.g >= c0.g and c1.b >= c0.b) {
            r = c0.r;
            g = c0.g;
            b = c0.b;
            r = r + c1.r;
            g = g + c1.g;
            b = b + c1.b;
            r = r / 2;
            g = g / 2;
            b = b / 2;
            c2 = RGB565{
                .r = @intCast(r),
                .g = @intCast(g),
                .b = @intCast(b),
            };
            c3 = RGB565{ .r = 0, .g = 0, .b = 0 };
        } else {
            r = c0.r;
            g = c0.g;
            b = c0.b;
            r = r * 2;
            g = g * 2;
            b = b * 2;
            r = r + c1.r;
            g = g + c1.g;
            b = b + c1.b;
            r = r / 3;
            g = g / 3;
            b = b / 3;
            c2 = RGB565{
                .r = @intCast(r),
                .g = @intCast(g),
                .b = @intCast(b),
            };

            r = c1.r;
            g = c1.g;
            b = c1.b;
            r = r * 2;
            g = g * 2;
            b = b * 2;
            r = r + c0.r;
            g = g + c0.g;
            b = b + c0.b;
            r = r / 3;
            g = g / 3;
            b = b / 3;
            c3 = RGB565{
                .r = @intCast(r),
                .g = @intCast(g),
                .b = @intCast(b),
            };
        }

        var lookupTable: [16]u2 = undefined;
        for (0..16) |i| {
            lookupTable[i] = @intCast((lookupTableU32 >> @intCast(2 * i)) & 0x3);
        }

        return BC1{ .c0 = c0, .c1 = c1, .c2 = c2, .c3 = c3, .lookupTable = lookupTable };
    }
    pub fn toPixel4x4(self: BC1) ![4][4]R8G8B8A8 {
        const colorPalette4x4 = [4]RGB565{ self.c0, self.c1, self.c2, self.c3 };

        var result: [4][4]R8G8B8A8 = undefined;
        for (0..4 * 4) |i| {
            const y = i / 4;
            const x = i % 4;
            result[y][x] = colorPalette4x4[self.lookupTable[y * 4 + x]].toR8G8B8A8();
        }
        return result;
    }
};
pub const BC2 = struct {
    // DONT BE LAZY
    alpha: [16]u4,
    bc1: BC1,
    pub fn toPixel4x4(self: BC2) ![4][4]R8G8B8A8 {
        var pixels = try self.bc1.toPixel4x4();
        for (0..4 * 4) |i| {
            const y = i / 4;
            const x = i % 4;
            pixels[y][x].a = self.alpha[i];
        }
        return pixels;
    }
    pub fn parse(reader: anytype) !BC2 {
        const alpha_data = try reader.readInt(u64, .little);
        var alpha: [4 * 4]u4 = undefined;

        for (0..4 * 4) |i| {
            alpha[i] = @as(u4, (alpha_data >> @intCast(4 * i)) & 0xF);
        }

        const bc1 = try BC1.parse(reader);
        return BC2{ .alpha = alpha, .bc1 = bc1 };
    }
};
pub const R8G8B8A8 = packed struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    pub fn fmt(self: R8G8B8A8, allocator: std.mem.Allocator) ![]u8 {
        return try std.fmt.allocPrint(allocator, "{d} {d} {d} {d}", .{ self.r, self.g, self.b, self.a });
    }
    pub fn packU32(self: R8G8B8A8) u32 {
        return (@as(u32, self.r) << 0) |
            (@as(u32, self.g) << 8) |
            (@as(u32, self.b) << 16) |
            (@as(u32, self.a) << 24);
    }
    pub fn packU8(self: R8G8B8A8) [4]u8 {
        return [4]u8{ self.r, self.g, self.b, self.a };
    }
    pub fn unpack(value: u32) R8G8B8A8 {
        return R8G8B8A8{
            .r = @intCast((value >> 0) & 0xFF),
            .g = @intCast((value >> 8) & 0xFF),
            .b = @intCast((value >> 16) & 0xFF),
            .a = @intCast((value >> 24) & 0xFF),
        };
    }
    pub fn parse(reader: anytype) !R8G8B8A8 {
        const raw = try reader.readInt(u32, .little);
        return unpack(raw);
    }
    pub fn parseA8R8G8B8(reader: anytype) !R8G8B8A8 {
        return R8G8B8A8{
            .a = try reader.readInt(u8, .little),
            .r = try reader.readInt(u8, .little),
            .g = try reader.readInt(u8, .little),
            .b = try reader.readInt(u8, .little),
        };
    }
};
pub fn parseUntilEOF(allocator: std.mem.Allocator, comptime T: type, reader: anytype) !std.ArrayList(T) {
    var blocks = std.ArrayList(T).init(allocator);
    errdefer blocks.deinit();

    while (true) {
        const block = T.parse(reader) catch |err| {
            if (err == error.EndOfStream) break;
            return err;
        };
        try blocks.append(block);
    }

    return blocks;
}

pub fn parsePgfXpr(allocator: std.mem.Allocator, file: std.fs.File) !void {
    var bufreader = std.io.bufferedReader(file.reader());
    const reader = bufreader.reader();
    const seeker = file.seekableStream();

    _ = try reader.readInt(u32, .little); // version
    _ = try reader.readInt(u32, .little); // -
    const sizes = try xpr.PgfSizes.parse(reader);
    _ = try reader.readInt(u32, .little);

    var textureResources = std.ArrayList(xpr.XprTexture).init(allocator);
    defer textureResources.deinit();
    const textureDataOffset = sizes.numTextures * 0x14 + 0x20;

    // for (0..sizes.numTextures) |_| {
    //     try textureResources.append(try xpr.XprTexture.parse(reader));
    // }
    // for (textureResources.items) |res| {
    const res = try xpr.XprTexture.parse(reader);
    std.debug.print("{any}\n", .{res});
    const h: u32 = @intCast(res.format.height);
    const w: u32 = @intCast(res.format.width);

    const offset = textureDataOffset + res.data;
    _ = try seeker.seekTo(offset);
    std.debug.print("offset 0x{x}\n", .{offset});

    var bc1array = std.ArrayList(BC1).init(allocator);
    defer bc1array.deinit();
    std.debug.print("h {x}\nw {x}\n", .{ h, w });
    std.debug.print("h * w / 16 = {d} {x}\n", .{ h * w / 16, h * w / 16 });
    for (0..0x3f5) |_| {
        _ = try reader.readInt(u32, .big);
    }
    for (0..h * w / 2 / 8 - 0x400) |_| {
        const btex = try BC1.parse(reader);
        try bc1array.append(btex);
    }

    var pixels = std.ArrayList(u8).init(allocator);
    var r: u8 = undefined;
    var g: u8 = undefined;
    var b: u8 = undefined;
    var a: u8 = undefined;
    defer pixels.deinit();

    for (0..h / 4 - 8) |Y| {
        for (0..4) |y| {
            for (0..w / 4) |X| {
                const table = try bc1array.items[(Y * w / 4 + X)].toPixel4x4();
                for (0..4) |x| {
                    r = @intCast(table[y][x].r);
                    g = @intCast(table[y][x].g);
                    b = @intCast(table[y][x].b);
                    a = table[y][x].a;
                    try pixels.append(r);
                    try pixels.append(g);
                    try pixels.append(b);
                    try pixels.append(a);
                }
            }
        }
    }
    const img = try ppm.Ppm7.init(allocator, w, h);
    const name = try std.fmt.allocPrint(allocator, "nz1_xpr0", .{});
    try img.write(name, pixels);
}

const std = @import("std");
const xpr = @import("xpr.zig");
const ppm = @import("ppm.zig");
