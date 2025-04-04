const bc1Path = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/splash_eng.xpr";
const bc1Path2 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/m_puffer_generic_10.xpr";

pub fn main() !void {
    const file = try std.fs.cwd().openFile(bc1Path, .{});
    defer file.close();

    var stream = std.io.bufferedReader(file.reader());
    var reader = stream.reader();

    const fourcc = try reader.readInt(u32, .little);
    _ = fourcc;
    const fileSize = try reader.readInt(u32, .little);
    _ = fileSize;
    const headerSize = try reader.readInt(u32, .little);
    _ = headerSize;
    const res0 = try reader.readInt(u32, .little);
    _ = res0;
    const res1 = try reader.readInt(u32, .little);
    _ = res1;
    const res2 = try reader.readInt(u32, .little);
    _ = res2;
    const res3 = try reader.readInt(u32, .little);
    _ = res3;
    const res4 = try reader.readInt(u32, .little);
    _ = res4;

    const allocator = std.heap.page_allocator;

    const y_start = 0; //280 / 4
    var ppm = try ppm3.Ppm3.init(allocator, 0x80 * 4, (0x80 - y_start) * 4 - 1, 255);
    defer ppm.deinit();
    var table = [4][4]bc1.RGBA8888{ undefined, undefined, undefined, undefined };
    var row = [4]bc1.RGBA8888{ undefined, undefined, undefined, undefined };

    var blocks: [0x80 * 0x80]bc1.BC1 = undefined;
    for (0..0x80 * 0x80) |i| {
        blocks[i] = try bc1.BC1.parse(reader);
    }
    for (y_start..0x80) |y| {
        for (0..4) |y2| {
            for (0..0x80) |x| {
                table = try blocks[y * 0x80 + x].toRGBATable();
                row = table[y2];
                for (0..4) |x2| {
                    try ppm.pixels.append(row[x2].r * 8);
                    try ppm.pixels.append(row[x2].g * 4);
                    try ppm.pixels.append(row[x2].b * 8);
                }
            }
        }
    }
    const ppm_str = try ppm.format();
    defer allocator.free(ppm_str);

    const output_path = "output.ppm";
    const output_file = try std.fs.cwd().createFile(output_path, .{});
    defer output_file.close();

    try output_file.writeAll(ppm_str);
    std.debug.print("PPM फ़ाइल सफलतापूर्वक लिखी गई: {s}\n", .{output_path});
}

const std = @import("std");
const bc1 = @import("bc.zig");
const ppm3 = @import("ppm.zig");
