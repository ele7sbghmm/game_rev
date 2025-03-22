//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.

pub fn main() !void {
    const path = "data/amped2_prototype_sep12/Levels/NZ1/NZ1_gfx/NZ1.pgf";
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    var bufreader = std.io.bufferedReader(file.reader());
    var reader = bufreader.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

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
    try reader.skipBytes(4, .{});

    try reader.skipBytes(pgfHeader.VBDataSize, .{});
    // const vertexBuffer = try allocator.alloc(u8, pgfHeader.VBDataSize);
    // defer allocator.free(vertexBuffer);
    //
    // _ = try reader.read(vertexBuffer);
    // std.debug.print("{any}\n", .{vertexBuffer[0..32]});
    // try reader.skipBytes(pgfHeader.NumVertexBuffers * 0xc, .{});

    var vbHeaders = std.ArrayList(u32).init(allocator);
    defer vbHeaders.deinit();
    _ = try parseHeaders(reader, &vbHeaders, pgfHeader.NumVertexBuffers, @intCast(pgfHeader.VBDataSize));

    try reader.skipBytes(pgfHeader.IBDataSize, .{});
    var ibHeaders = std.ArrayList(u32).init(allocator);
    defer ibHeaders.deinit();
    _ = try parseHeaders(reader, &ibHeaders, pgfHeader.NumIndexBuffers, @intCast(pgfHeader.IBDataSize));
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
    // a little more data past this point but don't know what it is

    std.debug.print("{}\n", .{pgfHeader});
    // std.debug.print("{}", .{vbHeaders});
    for (vbHeaders.items) |item| {
        std.debug.print("{}\n", .{item});
    }
    for (ibHeaders.items) |item| {
        std.debug.print("{}\n", .{item});
    }
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
fn parseHeaders(reader: anytype, array: *std.ArrayListAligned(u32, null), size: u32, end: u32) !void {
    for (0..size) |_| {
        try reader.skipBytes(4, .{});
        try array.append(try reader.readInt(u32, .little));
        try reader.skipBytes(4, .{});
    }
    try array.append(end);
}
const Color = struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    fn read(reader: anytype) !Color {
        return Color{
            .r = reader.readByte(u8),
            .g = reader.readByte(u8),
            .b = reader.readByte(u8),
            .a = reader.readByte(u8),
        };
    }
};
const Vertex = struct {
    x: f32,
    y: f32,
    z: f32,
    fn read(reader: anytype) !Vertex {
        return Vertex{
            .x = @bitCast(reader.readInt(u32, .little)),
            .y = @bitCast(reader.readInt(u32, .little)),
            .z = @bitCast(reader.readInt(u32, .little)),
        };
    }
};
const Vertex_1c = struct {
    pos: Vertex,
    color: Color,
    unk: Vertex,
    fn read(reader: anytype) !Vertex_1c {
        return Vertex_1c{
            .pos = Vertex.read(reader),
            .color = Vertex.read(reader),
            .unk = Vertex.read(reader),
        };
    }
};
const std = @import("std");
