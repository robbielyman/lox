const std = @import("std");
const Lox = @import("lox.zig");

pub fn main() !void {
    var gpa: std.heap.GeneralPurposeAllocator(.{}) = .{};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    switch (args.len) {
        1 => try runPrompt(allocator),
        2 => try runFile(allocator, args[1]),
        else => {
            try stdout.print("Usage: zlox [script]\n", .{});
            try bw.flush();
        },
    }
}

fn runPrompt(allocator: std.mem.Allocator) !void {
    var lox: Lox = .{};
    const stdin = std.io.getStdIn().reader();
    var src = std.ArrayList(u8).init(allocator);
    defer src.deinit();

    while (true) {
        defer lox.had_error = false;
        stdin.streamUntilDelimiter(src.writer(), '\n', null) catch |err| switch (err) {
            error.EndOfStream => return,
            else => return err,
        };
        defer src.clearRetainingCapacity();
        if (src.items.len == 0) break;
        try lox.run(src.items, allocator);
    }
}

fn runFile(allocator: std.mem.Allocator, file_name: []const u8) !void {
    var lox: Lox = .{};
    const file = try std.fs.cwd().openFile(file_name, .{});
    const src = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    defer allocator.free(src);
    try lox.run(src, allocator);
    if (lox.had_error) std.process.exit(65);
}
