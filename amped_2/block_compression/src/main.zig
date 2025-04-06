const bc1Path1 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/splash_eng.xpr";
const bc1Path2 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/m_puffer_generic_10.xpr";
const bc1Path3 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/TravisP_goggles.xpr";
const bc1Path4 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc1/boards_burton_01.xpr";
const bc2Path1 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc2/ampedLogo.xpr";
const bc2Path2 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc2/NZ1_Globe.xpr";
const bc2Path3 = "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/bc2/NZ1_LevelPict.xpr";

pub fn main() !void {
    const input = bc2Path3;
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

    const width = 128;
    // const height = 32;
    const height = (fileSize - headerSize) / 16 / width;
    const y_start = 0; //280 / 4

    var ppm_file = try ppm.Ppm7.init(allocator, width * 4, height * 4);
    defer ppm_file.deinit();

    var table = [4][4]bc.RGBA8888{ undefined, undefined, undefined, undefined };
    var row = [4]bc.RGBA8888{ undefined, undefined, undefined, undefined };

    const blocks = try bc.BC2.parseUntilEOF(allocator, reader);
    defer blocks.deinit();

    for (y_start..height) |y| {
        for (0..4) |y2| {
            for (0..width) |x| {
                table = try blocks.items[y * width + x].toPixels();
                row = table[y2];
                for (0..4) |x2| {
                    const alpha: u8 = @intCast(blocks.items[y * width + x].alpha[y2 * 4 + x2]);
                    const r: u32 = row[x2].r * 8;
                    const g: u32 = row[x2].g * 4;
                    const b: u32 = row[x2].b * 8;
                    const a: u32 = alpha * 16;
                    const rgba: u32 = (r << 0) + (g << 8) + (b << 16) + (a << 24);
                    try ppm_file.pixels.append(rgba);
                }
            }
        }
    }

    const header = try std.fmt.allocPrint(allocator, "P7\nWIDTH {d}\nHEIGHT {d}\nDEPTH {d}\nMAXVAL {d}\nTUPLTYPE {s}\nENDHDR\n", .{ ppm_file.width, ppm_file.height, ppm_file.depth, ppm_file.max_val, ppm_file.typltype.toString() });
    defer allocator.free(header);
    var result = std.ArrayList(u8).init(allocator);
    defer result.deinit();
    try result.appendSlice(header);

    for (ppm_file.pixels.items) |pixel| {
        try result.append(@intCast((pixel >> 0) & 0xFF));
        try result.append(@intCast((pixel >> 8) & 0xFF));
        try result.append(@intCast((pixel >> 16) & 0xFF));
        try result.append(@intCast((pixel >> 24) & 0xFF));
    }

    const input_path = std.fs.path.basename(input);
    const output_filename = std.fmt.allocPrint(allocator, "/Users/im/Public/kod/mujhe/game_rev/amped_2/block_compression/data/ppm/{s}.ppm", .{input_path[0..std.mem.lastIndexOf(u8, input_path, ".").?]}) catch return error.OutOfMemory;
    defer allocator.free(output_filename);

    const output_file = try std.fs.cwd().createFile(output_filename, .{});
    defer output_file.close();

    try output_file.writeAll(result.items);

    std.debug.print("filename: {s}\n", .{output_filename});
    // std.debug.print("pixels: {any}\n", .{ppm_file.pixels});
    std.debug.print("Number of pixels: {d}\n", .{ppm_file.pixels.items.len});
    if (ppm_file.pixels.items.len > 0) {
        std.debug.print("First pixel: {d}\n", .{ppm_file.pixels.items[0]});
    }
}

const std = @import("std");
const bc = @import("bc.zig");
const ppm = @import("ppm.zig");
