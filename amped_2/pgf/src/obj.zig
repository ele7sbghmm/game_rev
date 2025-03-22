const std = @import("std");

const Obj = struct {
    allocator: std.mem.Allocator,
    vertexCount: u32,
    vertexBuffer: *std.ArrayList(u8),
    faceBuffer: *std.ArrayList(u8),

    fn init() Obj {
        const allocator = std.heap.page_allocator;

        var vertexBuffer = std.ArrayList(u8).init(allocator);
        defer vertexBuffer.deinit();
        var faceBuffer = std.ArrayList(u8).init(allocator);
        defer faceBuffer.deinit();

        return Obj{
            .allocator = allocator,
            .vertexBuffer = vertexBuffer,
            .faceBuffer = faceBuffer,
        };
    }
    fn addTri(self: *Obj, v1: Vec3, v2: Vec3, v3: Vec3) !void {
        try self.vertexBuffer.appendSlice(try v1.format(self.allocator));
        try self.vertexBuffer.appendSlice(try v2.format(self.allocator));
        try self.vertexBuffer.appendSlice(try v3.format(self.allocator));

        try self.faceBuffer.appendSlice(try std.fmt.allocPrint(self.allocator, "f {d} {d} {d}\n", .{ self.vertexCount, self.vertexCount + 1, self.vertexCount + 2 }));
        self.vertexCount += 3;
    }
    fn print(self: Obj) void {
        std.debug.print("{s}\n", .{self.vertexBuffer.items});
        std.debug.print("{s}\n", .{self.faceBuffer.items});
    }
};
const Vec3 = struct {
    x: f32,
    y: f32,
    z: f32,
    fn format(self: Vec3, allocator: std.mem.Allocator) ![]u8 {
        return try std.fmt.allocPrint(allocator, "v {d:.2} {d:.2} {d:.2}\n", .{ self.x, self.y, self.z });
    }
};
pub fn main() !void {
    var obj = Obj{};
    try obj.addTri(Vec3{ .x = 1, .y = 2, .z = 3 }, Vec3{ .x = 4, .y = 5, .z = 6 }, Vec3{ .x = 7, .y = 8, .z = 9 });
    obj.print();
}
