const Lox = @This();
const Token = @import("token.zig");
const std = @import("std");

had_error: bool = false,

pub fn run(lox: *Lox, src: []const u8, allocator: std.mem.Allocator) !void {
    var tokens = std.ArrayList(Token).init(allocator);
    defer tokens.deinit();
    var scanner = Token.Scanner.init(src);
    while (scanner.next()) |res| {
        switch (res) {
            .ok => |token| try tokens.append(token),
            .err => |scan_err| {
                switch (scan_err) {
                    .unexpected_character => |line| lox.err(line, "Unexpected character."),
                    .unterminated_string => |line| lox.err(line, "Unterminated string."),
                }
            },
        }
    }
    try tokens.append(.{
        .token_type = .eof,
        .lexeme = "\x00",
        .line = scanner.line + 1,
    });
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    for (tokens.items) |value| {
        try stdout.print("{}\n", .{value});
    }
    try bw.flush();
}

pub fn err(lox: *Lox, line: usize, msg: []const u8) void {
    lox.report(line, "", msg);
}

fn report(lox: *Lox, line: usize, where: []const u8, msg: []const u8) void {
    std.debug.print("[line {d}] Error{s}: {s}\n", .{ line, where, msg });
    lox.had_error = true;
}
