//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.

pub fn main() !void {
    const path = "data/amped2_prototype_sep12/Levels/NZ1/NZ1_gfx/NZ1.pgf";
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    var bufreader = std.io.bufferedReader(file.reader());
    var reader = bufreader.reader();

    const version: f32 = @bitCast(try reader.readInt(u32, .little));
    _ = version;
    try reader.skipBytes(4, .{});

    const pgfSizes = try PgfSizes.read(reader);
    try reader.skipBytes(4, .{});

    try reader.skipBytes(pgfSizes.NumTextures * 0x14, .{});
    try reader.skipBytes(pgfSizes.TextureDataSize, .{});
    try reader.skipBytes(4, .{});

    try reader.skipBytes(pgfSizes.ShaderDataSize, .{});
    try reader.skipBytes(4, .{});

    const pgfHeader = try PgfHeader.read(reader);
    try reader.skipBytes(pgfHeader.VBDataSize, .{});
    try reader.skipBytes(pgfHeader.NumVertexBuffers * 0xc, .{});

    try reader.skipBytes(pgfHeader.IBDataSize, .{});
    try reader.skipBytes(pgfHeader.NumIndexBuffers * 0xc, .{});
    try reader.skipBytes(4, .{});

    try reader.skipBytes(pgfHeader.PGDataSize, .{});
    try reader.skipBytes(pgfHeader.BVDataSize, .{});
    try reader.skipBytes(pgfHeader.MiscDataSize, .{});
    try reader.skipBytes(pgfHeader.InfluenceDataSize, .{});
    try reader.skipBytes(pgfHeader.LIMDataSize, .{});
    try reader.skipBytes(pgfHeader.CollisionDataSize, .{});
    try reader.skipBytes(pgfHeader.StringTableSize, .{});
    try reader.skipBytes(pgfHeader.NumPrimLists * 48, .{});
    try reader.skipBytes(pgfHeader.NumVBGeomData * 48, .{});
    // more data past this point but don't know what it is

    std.debug.print("{}", .{pgfHeader});
}
const PgfSizes = struct {
    TotalFileSize: u32,
    TotalDataSize: u32,
    TextureDataSize: u32,
    NumTextures: u32,
    ShaderDataSize: u32,

    fn read(reader: anytype) !PgfSizes {
        return PgfSizes{
            .TotalFileSize = try reader.readInt(u32, .little),
            .TotalDataSize = try reader.readInt(u32, .little),
            .TextureDataSize = try reader.readInt(u32, .little),
            .NumTextures = try reader.readInt(u32, .little),
            .ShaderDataSize = try reader.readInt(u32, .little),
        };
    }
};
const PgfHeader = struct {
    VBDataSize: u32,
    IBDataSize: u32,
    PGDataSize: u32,
    BVDataSize: u32,
    MiscDataSize: u32,
    InfluenceDataSize: u32,
    LIMDataSize: u32,
    CollisionDataSize: u32,
    StringTableSize: u32,
    NumShaders: u32,
    NumVertexBuffers: u32,
    NumIndexBuffers: u32,
    NumPushBuffers: u32,
    NumPrimLists: u32,
    NumVBGeomData: u32,
    NumJoints: u32,

    fn read(reader: anytype) !PgfHeader {
        return PgfHeader{
            .VBDataSize = try reader.readInt(u32, .little),
            .IBDataSize = try reader.readInt(u32, .little),
            .PGDataSize = try reader.readInt(u32, .little),
            .BVDataSize = try reader.readInt(u32, .little),
            .MiscDataSize = try reader.readInt(u32, .little),
            .InfluenceDataSize = try reader.readInt(u32, .little),
            .LIMDataSize = try reader.readInt(u32, .little),
            .CollisionDataSize = try reader.readInt(u32, .little),
            .StringTableSize = try reader.readInt(u32, .little),
            .NumShaders = try reader.readInt(u32, .little),
            .NumVertexBuffers = try reader.readInt(u32, .little),
            .NumIndexBuffers = try reader.readInt(u32, .little),
            .NumPushBuffers = try reader.readInt(u32, .little),
            .NumPrimLists = try reader.readInt(u32, .little),
            .NumVBGeomData = try reader.readInt(u32, .little),
            .NumJoints = try reader.readInt(u32, .little),
        };
    }
};
const std = @import("std");
