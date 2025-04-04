const bc1Path = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/splash_eng.xpr";
const bc1Path2 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/m_puffer_generic_10.xpr";
const bc1Path3 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/ampedLogo.xpr";

pub fn main() !void {
    const input = bc1Path3;
    const file = try std.fs.cwd().openFile(input, .{});
    defer file.close();

    var stream = std.io.bufferedReader(file.reader());
    var reader = stream.reader();

    const fourcc = try reader.readInt(u32, .little);
    _ = fourcc;
    const fileSize = try reader.readInt(u32, .little);
    const headerSize = try reader.readInt(u32, .little);
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

    const width = 256;
    //const height = 128;
    const height = (fileSize - headerSize) / 8 / width;
    const y_start = 0; //280 / 4

    var ppm = try ppm3.Ppm3.init(allocator, width * 4, height * 4, 255);
    defer ppm.deinit();

    var table = [4][4]bc1.RGBA8888{ undefined, undefined, undefined, undefined };
    var row = [4]bc1.RGBA8888{ undefined, undefined, undefined, undefined };

    const blocks = try bc1.BC1.parseUntilEOF(allocator, reader);
    defer blocks.deinit();

    for (y_start..height) |y| {
        for (0..4) |y2| {
            for (0..width) |x| {
                table = try blocks.items[y * width + x].toRGBATable();
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

    // इनपुट फाइल का नाम प्राप्त करें और एक्सटेंशन हटाएं
    const input_path = std.fs.path.basename(input);
    const output_filename = std.fmt.allocPrint(allocator, "{s}.ppm", .{input_path[0..std.mem.lastIndexOf(u8, input_path, ".").?]}) catch return error.OutOfMemory;
    defer allocator.free(output_filename);

    const output_file = try std.fs.cwd().createFile(output_filename, .{});
    defer output_file.close();

    try output_file.writeAll(ppm_str);
    std.debug.print("PPM फ़ाइल सफलतापूर्वक लिखी गई: {s}\n", .{output_filename});
    std.debug.print("PPM पिक्सेल्स की संख्या: {d}\n", .{ppm.pixels.items.len});
}

const std = @import("std");
const bc1 = @import("bc.zig");
const ppm3 = @import("ppm.zig");
