const std = @import("std");
const bc = @import("bc.zig");

// https://xboxdevwiki.net/XPR
pub const XprHeader = struct {
    magic: u32,
    fileSize: u32,
    headerSize: u32,
    pub fn parse(reader: anytype) !XprHeader {
        return XprHeader{
            .magic = try reader.readInt(u32, .little),
            .fileSize = try reader.readInt(u32, .little),
            .headerSize = try reader.readInt(u32, .little),
        };
    }
};
pub const XprTexture = struct {
    common: u32,
    data: u32,
    lock: u32,
    format: Format,
    size: u32,
    pub fn parse(reader: anytype) !XprTexture {
        return XprTexture{
            .common = try reader.readInt(u32, .little),
            .data = try reader.readInt(u32, .little),
            .lock = try reader.readInt(u32, .little),
            .format = Format.unpack(try reader.readInt(u32, .little)),
            .size = try reader.readInt(u32, .little),
        };
    }
};
const Format = struct {
    dma: u32,
    dimensions: u32,
    format: u32,
    levels: u32,
    width: u32,
    height: u32,
    depth: u32,
    pub fn unpack(raw: u32) Format {
        return Format{
            .dma = 0b1111 & (raw >> 0),
            .dimensions = 0b1111 & (raw >> 4),
            .format = 0b1111_1111 & (raw >> 8),
            .levels = 0b1111 & (raw >> 16),
            .width = @as(u32, 1) << @intCast(0b1111 & (raw >> 20)),
            .height = @as(u32, 1) << @intCast(0b1111 & (raw >> 20)),
            .depth = 0b1111 & (raw >> 28),
        };
    }
};

pub const CompressionType = union(enum) {
    A8R8G8B8: bc.A8R8G8B8,
    BC1: bc.BC1,
    BC2: bc.BC2,
    Unsupported,
};

pub fn getBlockCompressionType(format: u32) CompressionType {
    return switch (format) {
        0x6 => .{CompressionType.A8R8G8B8},
        0xc => .{CompressionType.BC1},
        0xe => .{CompressionType.BC2},
        else => .Unsupported,
    };
}

pub const PgfSizes = struct {
    totalFileSize: u32,
    totalDataSize: u32,
    textureDataSize: u32,
    numTextures: u32,
    shaderDataSize: u32,
    pub fn parse(reader: anytype) !PgfSizes {
        return PgfSizes{ .totalFileSize = try reader.readInt(u32, .little), .totalDataSize = try reader.readInt(u32, .little), .textureDataSize = try reader.readInt(u32, .little), .numTextures = try reader.readInt(u32, .little), .shaderDataSize = try reader.readInt(u32, .little) };
    }
};
