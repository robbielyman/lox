const Token = @This();
const std = @import("std");

token_type: Type,
lexeme: []const u8,
line: usize,

pub const Type = enum {
    left_paren,
    right_paren,
    left_brace,
    right_brace,
    comma,
    dot,
    minus,
    plus,
    semicolon,
    slash,
    star,
    bang,
    bang_equal,
    equal,
    equal_equal,
    greater,
    greater_equal,
    less,
    less_equal,
    identifier,
    string,
    number,
    @"and",
    class,
    @"else",
    false,
    fun,
    @"for",
    @"if",
    nil,
    @"or",
    print,
    @"return",
    super,
    this,
    true,
    @"var",
    @"while",
    eof,
};

pub fn format(self: Token, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
    try writer.print("{s} {s} {d}", .{ @tagName(self.token_type), self.lexeme, self.line });
}

pub const Scanner = struct {
    source: []const u8,
    start: usize,
    line: usize,

    const keywords = std.StaticStringMap(Type).initComptime(.{
        .{ "and", .@"and" },
        .{ "class", .class },
        .{ "else", .@"else" },
        .{ "false", .false },
        .{ "for", .@"for" },
        .{ "fun", .fun },
        .{ "if", .@"if" },
        .{ "nil", .nil },
        .{ "or", .@"or" },
        .{ "print", .print },
        .{ "return", .@"return" },
        .{ "super", .super },
        .{ "this", .this },
        .{ "true", .true },
        .{ "var", .@"var" },
        .{ "while", .@"while" },
    });

    pub const Result = union(enum) {
        ok: Token,
        err: union(enum) {
            unexpected_character: usize,
            unterminated_string: usize,
        },
    };

    pub fn init(source: []const u8) Scanner {
        return .{
            .source = source,
            .start = 0,
            .line = 1,
        };
    }

    pub fn next(scanner: *Scanner) ?Result {
        var current = scanner.start;
        var t: ?Type = null;
        while (t == null) {
            scanner.start = current;
            if (scanner.start >= scanner.source.len) return null;
            const byte = scanner.source[current];
            current += 1;
            switch (byte) {
                '(' => t = .left_paren,
                ')' => t = .right_paren,
                '{' => t = .left_brace,
                '}' => t = .right_brace,
                ',' => t = .comma,
                '.' => t = .dot,
                '-' => t = .minus,
                '+' => t = .plus,
                ';' => t = .semicolon,
                '*' => t = .star,
                '!' => t = if (matches(scanner.source, '=', &current)) .bang_equal else .bang,
                '=' => t = if (matches(scanner.source, '=', &current)) .equal_equal else .equal,
                '<' => t = if (matches(scanner.source, '=', &current)) .less_equal else .less,
                '>' => t = if (matches(scanner.source, '=', &current)) .greater_equal else .greater,
                '/' => if (matches(scanner.source, '/', &current)) {
                    while (current < scanner.source.len and scanner.source[current] != '\n')
                        current += 1;
                } else {
                    t = .slash;
                },
                ' ', '\r', '\t' => {},
                '\n' => scanner.line += 1,
                '"' => {
                    while (current < scanner.source.len and scanner.source[current] != '"') {
                        if (scanner.source[current] == '\n')
                            scanner.line += 1;
                        current += 1;
                    }
                    if (current >= scanner.source.len) {
                        scanner.start = current;
                        return .{ .err = .{ .unterminated_string = scanner.line } };
                    }
                    current += 1;
                    t = .string;
                },
                '0'...'9' => {
                    while (current < scanner.source.len and isDigit(scanner.source[current]))
                        current += 1;
                    if (current + 1 < scanner.source.len and scanner.source[current] == '.' and isDigit(scanner.source[current]))
                        current += 1;
                    while (current < scanner.source.len and isDigit(scanner.source[current]))
                        current += 1;
                    t = .number;
                },
                else => {
                    if (isAlpha(byte)) {
                        while (current < scanner.source.len and (isAlpha(scanner.source[current]) or isDigit(scanner.source[current])))
                            current += 1;
                        const keyword = scanner.source[scanner.start..current];
                        t = if (keywords.get(keyword)) |k| k else .identifier;
                    } else {
                        scanner.start = current;
                        return .{ .err = .{ .unexpected_character = scanner.line } };
                    }
                },
            }
        }
        const lexeme = scanner.source[scanner.start..current];
        scanner.start = current;
        return .{
            .ok = .{
                .token_type = t.?,
                .lexeme = lexeme,
                .line = scanner.line,
            },
        };
    }
};

fn isAlpha(byte: u8) bool {
    return (byte >= 'a' and byte <= 'z') or (byte >= 'A' and byte <= 'Z') or byte == '_';
}

fn isDigit(byte: u8) bool {
    return byte >= '0' and byte <= '9';
}

fn matches(source: []const u8, against: u8, current: *usize) bool {
    if (current.* >= source.len) return false;
    if (source[current.*] != against) return false;
    current.* += 1;
    return true;
}
