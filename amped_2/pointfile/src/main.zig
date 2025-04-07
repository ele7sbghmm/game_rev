pub fn main() !void {
    const path = "./data/Pointfile.dat";
    const out_path = "./data/Pointfile.obj";

    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    var bufreader = std.io.bufferedReader(file.reader());
    var reader = bufreader.reader();

    const numFloats = try reader.readInt(u32, .little);

    const allocator = std.heap.page_allocator;

    var floats = std.ArrayList(f32).init(allocator);
    defer floats.deinit();
    var obj_buffer = std.ArrayList(u8).init(allocator);
    defer obj_buffer.deinit();

    var tmp: f32 = undefined;
    for (0..numFloats) |_| {
        tmp = @bitCast(try reader.readInt(u32, .little));
        try floats.append(tmp);
    }

    var i: u32 = 0;
    for (0..floats.items.len / 3 - 2) |_| {
        try obj_buffer.appendSlice(try std.fmt.allocPrint(allocator, "v {d:.2} {d:.2} {d:.2}\n", .{ floats.items[i + 0], floats.items[i + 1], floats.items[i + 2] }));
        i += 3;
    }

    const out_file = try std.fs.cwd().createFile(out_path, .{});
    defer out_file.close();

    try out_file.writeAll(obj_buffer.items);
}

const std = @import("std");
