pub struct Glyph {
	pub id: usize,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub x_offset: i32,
	pub y_offset: i32,
	pub x_advance: i32,
}

pub const LINE_HEIGHT: i32 = 26;
pub const BASE: i32 = 21;
pub static GLYPH: [Glyph; 336] = [
	Glyph { id: 32, x: 12, y: 214, width: 1, height: 1, x_offset: 0, y_offset: -5, x_advance: 5 },
	Glyph { id: 33, x: 68, y: 48, width: 3, height: 14, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 34, x: 243, y: 48, width: 5, height: 5, x_offset: 1, y_offset: 9, x_advance: 7 },
	Glyph { id: 35, x: 0, y: 60, width: 12, height: 14, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 36, x: 21, y: 139, width: 9, height: 16, x_offset: 1, y_offset: -1, x_advance: 11 },
	Glyph { id: 37, x: 154, y: 144, width: 14, height: 15, x_offset: 1, y_offset: 0, x_advance: 15 },
	Glyph { id: 38, x: 184, y: 144, width: 13, height: 15, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 39, x: 4, y: 27, width: 2, height: 5, x_offset: 1, y_offset: 9, x_advance: 4 },
	Glyph { id: 40, x: 201, y: 160, width: 5, height: 17, x_offset: 0, y_offset: -3, x_advance: 5 },
	Glyph { id: 41, x: 207, y: 160, width: 5, height: 17, x_offset: 0, y_offset: -3, x_advance: 5 },
	Glyph { id: 42, x: 150, y: 44, width: 9, height: 9, x_offset: 0, y_offset: 6, x_advance: 10 },
	Glyph { id: 43, x: 92, y: 41, width: 10, height: 10, x_offset: 0, y_offset: 2, x_advance: 11 },
	Glyph { id: 44, x: 0, y: 27, width: 3, height: 5, x_offset: 0, y_offset: -2, x_advance: 4 },
	Glyph { id: 45, x: 24, y: 29, width: 5, height: 3, x_offset: 0, y_offset: 4, x_advance: 6 },
	Glyph { id: 46, x: 50, y: 29, width: 3, height: 3, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 47, x: 46, y: 45, width: 7, height: 14, x_offset: 0, y_offset: 0, x_advance: 7 },
	Glyph { id: 48, x: 68, y: 127, width: 10, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 49, x: 62, y: 48, width: 5, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 50, x: 80, y: 112, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 51, x: 100, y: 112, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 52, x: 37, y: 60, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 53, x: 209, y: 66, width: 9, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 54, x: 70, y: 95, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 55, x: 144, y: 66, width: 10, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 56, x: 50, y: 94, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 57, x: 30, y: 91, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 58, x: 74, y: 36, width: 3, height: 11, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 59, x: 131, y: 52, width: 4, height: 13, x_offset: 0, y_offset: -2, x_advance: 5 },
	Glyph { id: 60, x: 113, y: 41, width: 9, height: 10, x_offset: 0, y_offset: 2, x_advance: 11 },
	Glyph { id: 61, x: 223, y: 47, width: 9, height: 6, x_offset: 1, y_offset: 4, x_advance: 11 },
	Glyph { id: 62, x: 123, y: 41, width: 9, height: 10, x_offset: 0, y_offset: 2, x_advance: 11 },
	Glyph { id: 63, x: 188, y: 96, width: 8, height: 15, x_offset: 0, y_offset: 0, x_advance: 8 },
	Glyph { id: 64, x: 219, y: 162, width: 15, height: 16, x_offset: 1, y_offset: -2, x_advance: 17 },
	Glyph { id: 65, x: 219, y: 81, width: 12, height: 14, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 66, x: 166, y: 66, width: 10, height: 14, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 67, x: 12, y: 123, width: 11, height: 15, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 68, x: 97, y: 66, width: 11, height: 14, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 69, x: 10, y: 45, width: 8, height: 14, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 70, x: 28, y: 45, width: 8, height: 14, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 71, x: 226, y: 146, width: 12, height: 15, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 72, x: 85, y: 66, width: 11, height: 14, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 73, x: 76, y: 48, width: 2, height: 14, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 74, x: 189, y: 160, width: 5, height: 17, x_offset: -1, y_offset: -3, x_advance: 5 },
	Glyph { id: 75, x: 188, y: 66, width: 10, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 76, x: 37, y: 45, width: 8, height: 14, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 77, x: 163, y: 81, width: 14, height: 14, x_offset: 1, y_offset: 0, x_advance: 17 },
	Glyph { id: 78, x: 61, y: 63, width: 11, height: 14, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 79, x: 212, y: 144, width: 13, height: 15, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 80, x: 239, y: 66, width: 9, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 81, x: 220, y: 198, width: 13, height: 18, x_offset: 1, y_offset: -3, x_advance: 14 },
	Glyph { id: 82, x: 245, y: 83, width: 10, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 83, x: 110, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 84, x: 25, y: 60, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 85, x: 13, y: 60, width: 11, height: 14, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 86, x: 206, y: 81, width: 12, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 87, x: 83, y: 81, width: 17, height: 14, x_offset: 0, y_offset: 0, x_advance: 17 },
	Glyph { id: 88, x: 109, y: 66, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 89, x: 121, y: 66, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 90, x: 155, y: 66, width: 10, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 91, x: 195, y: 160, width: 5, height: 17, x_offset: 1, y_offset: -3, x_advance: 6 },
	Glyph { id: 92, x: 54, y: 47, width: 7, height: 14, x_offset: 0, y_offset: 0, x_advance: 7 },
	Glyph { id: 93, x: 213, y: 161, width: 5, height: 17, x_offset: 0, y_offset: -3, x_advance: 6 },
	Glyph { id: 94, x: 81, y: 38, width: 10, height: 10, x_offset: 0, y_offset: 5, x_advance: 10 },
	Glyph { id: 95, x: 65, y: 32, width: 9, height: 2, x_offset: 0, y_offset: -3, x_advance: 8 },
	Glyph { id: 96, x: 41, y: 29, width: 4, height: 3, x_offset: 3, y_offset: 12, x_advance: 11 },
	Glyph { id: 97, x: 10, y: 33, width: 9, height: 11, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 98, x: 112, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 99, x: 58, y: 35, width: 8, height: 11, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 100, x: 145, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 101, x: 20, y: 33, width: 9, height: 11, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 102, x: 24, y: 75, width: 7, height: 15, x_offset: 0, y_offset: 0, x_advance: 6 },
	Glyph { id: 103, x: 156, y: 128, width: 10, height: 15, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 104, x: 30, y: 107, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 105, x: 80, y: 80, width: 2, height: 15, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 106, x: 177, y: 197, width: 5, height: 19, x_offset: -1, y_offset: -4, x_advance: 4 },
	Glyph { id: 107, x: 40, y: 93, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 108, x: 251, y: 98, width: 2, height: 15, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 109, x: 196, y: 54, width: 15, height: 11, x_offset: 1, y_offset: 0, x_advance: 17 },
	Glyph { id: 110, x: 30, y: 33, width: 9, height: 11, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 111, x: 212, y: 54, width: 10, height: 11, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 112, x: 24, y: 123, width: 10, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 113, x: 123, y: 128, width: 10, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 114, x: 67, y: 36, width: 6, height: 11, x_offset: 1, y_offset: 0, x_advance: 7 },
	Glyph { id: 115, x: 40, y: 33, width: 8, height: 11, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 116, x: 115, y: 52, width: 7, height: 13, x_offset: 0, y_offset: 0, x_advance: 6 },
	Glyph { id: 117, x: 0, y: 33, width: 9, height: 11, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 118, x: 223, y: 54, width: 10, height: 11, x_offset: 0, y_offset: 0, x_advance: 9 },
	Glyph { id: 119, x: 164, y: 54, width: 15, height: 11, x_offset: 0, y_offset: 0, x_advance: 14 },
	Glyph { id: 120, x: 234, y: 54, width: 10, height: 11, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 121, x: 101, y: 128, width: 10, height: 15, x_offset: 0, y_offset: -4, x_advance: 9 },
	Glyph { id: 122, x: 49, y: 33, width: 8, height: 11, x_offset: 0, y_offset: 0, x_advance: 9 },
	Glyph { id: 123, x: 158, y: 160, width: 7, height: 17, x_offset: 0, y_offset: -3, x_advance: 7 },
	Glyph { id: 124, x: 203, y: 197, width: 2, height: 19, x_offset: 4, y_offset: -4, x_advance: 10 },
	Glyph { id: 125, x: 150, y: 160, width: 7, height: 17, x_offset: 0, y_offset: -3, x_advance: 7 },
	Glyph { id: 126, x: 7, y: 29, width: 9, height: 3, x_offset: 0, y_offset: 6, x_advance: 11 },
	Glyph { id: 160, x: 9, y: 234, width: 1, height: 1, x_offset: 0, y_offset: -5, x_advance: 5 },
	Glyph { id: 161, x: 72, y: 48, width: 3, height: 14, x_offset: 1, y_offset: -3, x_advance: 5 },
	Glyph { id: 162, x: 215, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 163, x: 134, y: 128, width: 10, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 164, x: 160, y: 44, width: 9, height: 9, x_offset: 1, y_offset: 3, x_advance: 11 },
	Glyph { id: 165, x: 49, y: 62, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 166, x: 253, y: 217, width: 2, height: 19, x_offset: 4, y_offset: -4, x_advance: 10 },
	Glyph { id: 167, x: 116, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 168, x: 17, y: 29, width: 6, height: 3, x_offset: 2, y_offset: 12, x_advance: 11 },
	Glyph { id: 169, x: 74, y: 144, width: 15, height: 15, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 170, x: 203, y: 45, width: 6, height: 8, x_offset: 0, y_offset: 7, x_advance: 6 },
	Glyph { id: 171, x: 170, y: 44, width: 8, height: 9, x_offset: 0, y_offset: 1, x_advance: 9 },
	Glyph { id: 172, x: 233, y: 48, width: 9, height: 5, x_offset: 0, y_offset: 3, x_advance: 11 },
	Glyph { id: 173, x: 30, y: 29, width: 5, height: 3, x_offset: 0, y_offset: 4, x_advance: 6 },
	Glyph { id: 174, x: 90, y: 144, width: 15, height: 15, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 175, x: 54, y: 30, width: 10, height: 2, x_offset: 0, y_offset: 14, x_advance: 9 },
	Glyph { id: 176, x: 215, y: 46, width: 7, height: 7, x_offset: 1, y_offset: 8, x_advance: 8 },
	Glyph { id: 177, x: 136, y: 53, width: 10, height: 12, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 178, x: 188, y: 44, width: 6, height: 9, x_offset: 0, y_offset: 6, x_advance: 6 },
	Glyph { id: 179, x: 133, y: 41, width: 6, height: 10, x_offset: 0, y_offset: 5, x_advance: 6 },
	Glyph { id: 180, x: 36, y: 29, width: 4, height: 3, x_offset: 3, y_offset: 12, x_advance: 11 },
	Glyph { id: 181, x: 160, y: 112, width: 9, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 182, x: 122, y: 160, width: 10, height: 17, x_offset: 1, y_offset: -2, x_advance: 12 },
	Glyph { id: 183, x: 46, y: 29, width: 3, height: 3, x_offset: 1, y_offset: 6, x_advance: 5 },
	Glyph { id: 184, x: 249, y: 48, width: 4, height: 5, x_offset: 0, y_offset: -4, x_advance: 4 },
	Glyph { id: 185, x: 210, y: 45, width: 4, height: 8, x_offset: 0, y_offset: 6, x_advance: 6 },
	Glyph { id: 186, x: 195, y: 45, width: 7, height: 8, x_offset: 0, y_offset: 7, x_advance: 7 },
	Glyph { id: 187, x: 179, y: 44, width: 8, height: 9, x_offset: 0, y_offset: 1, x_advance: 9 },
	Glyph { id: 188, x: 148, y: 81, width: 14, height: 14, x_offset: 0, y_offset: 0, x_advance: 15 },
	Glyph { id: 189, x: 133, y: 81, width: 14, height: 14, x_offset: 0, y_offset: 0, x_advance: 15 },
	Glyph { id: 190, x: 58, y: 143, width: 15, height: 15, x_offset: 0, y_offset: 0, x_advance: 15 },
	Glyph { id: 191, x: 134, y: 96, width: 8, height: 15, x_offset: 0, y_offset: -4, x_advance: 8 },
	Glyph { id: 192, x: 126, y: 237, width: 12, height: 19, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 193, x: 178, y: 237, width: 12, height: 19, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 194, x: 191, y: 237, width: 12, height: 19, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 195, x: 66, y: 178, width: 12, height: 18, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 196, x: 79, y: 178, width: 12, height: 18, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 197, x: 14, y: 175, width: 12, height: 18, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 198, x: 101, y: 81, width: 16, height: 14, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 199, x: 12, y: 216, width: 11, height: 19, x_offset: 1, y_offset: -4, x_advance: 12 },
	Glyph { id: 200, x: 91, y: 197, width: 8, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 201, x: 109, y: 197, width: 8, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 202, x: 100, y: 197, width: 8, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 203, x: 29, y: 158, width: 8, height: 18, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 204, x: 183, y: 197, width: 4, height: 19, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 205, x: 198, y: 197, width: 4, height: 19, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 206, x: 169, y: 197, width: 7, height: 19, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 207, x: 248, y: 198, width: 6, height: 18, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 208, x: 192, y: 81, width: 13, height: 14, x_offset: 0, y_offset: 0, x_advance: 13 },
	Glyph { id: 209, x: 213, y: 179, width: 11, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 210, x: 84, y: 237, width: 13, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 211, x: 98, y: 237, width: 13, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 212, x: 112, y: 237, width: 13, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 213, x: 0, y: 175, width: 13, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 214, x: 234, y: 198, width: 13, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 215, x: 140, y: 43, width: 9, height: 9, x_offset: 1, y_offset: 3, x_advance: 11 },
	Glyph { id: 216, x: 235, y: 162, width: 13, height: 16, x_offset: 1, y_offset: -1, x_advance: 14 },
	Glyph { id: 217, x: 228, y: 237, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 218, x: 108, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 219, x: 120, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 220, x: 141, y: 178, width: 11, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 221, x: 216, y: 237, width: 11, height: 19, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 222, x: 199, y: 66, width: 9, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 223, x: 79, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 224, x: 221, y: 128, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 225, x: 231, y: 130, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 226, x: 0, y: 91, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 227, x: 0, y: 107, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 228, x: 10, y: 107, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 229, x: 31, y: 141, width: 9, height: 16, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 230, x: 180, y: 54, width: 15, height: 11, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 231, x: 80, y: 96, width: 8, height: 15, x_offset: 1, y_offset: -4, x_advance: 9 },
	Glyph { id: 232, x: 150, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 233, x: 200, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 234, x: 20, y: 107, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 235, x: 211, y: 128, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 236, x: 251, y: 147, width: 4, height: 15, x_offset: 0, y_offset: 0, x_advance: 4 },
	Glyph { id: 237, x: 251, y: 131, width: 4, height: 15, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 238, x: 0, y: 75, width: 7, height: 15, x_offset: 0, y_offset: 0, x_advance: 4 },
	Glyph { id: 239, x: 40, y: 77, width: 6, height: 15, x_offset: 0, y_offset: 0, x_advance: 4 },
	Glyph { id: 240, x: 90, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 241, x: 190, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 242, x: 167, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 243, x: 200, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 244, x: 35, y: 125, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 245, x: 46, y: 126, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 246, x: 57, y: 126, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 247, x: 103, y: 41, width: 9, height: 10, x_offset: 0, y_offset: 2, x_advance: 11 },
	Glyph { id: 248, x: 104, y: 52, width: 10, height: 13, x_offset: 1, y_offset: -1, x_advance: 11 },
	Glyph { id: 249, x: 180, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 250, x: 140, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 251, x: 130, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 252, x: 240, y: 114, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 253, x: 132, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 9 },
	Glyph { id: 254, x: 0, y: 194, width: 10, height: 19, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 255, x: 242, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 9 },
	Glyph { id: 256, x: 85, y: 160, width: 12, height: 17, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 257, x: 0, y: 45, width: 9, height: 14, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 258, x: 92, y: 178, width: 12, height: 18, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 259, x: 220, y: 112, width: 9, height: 15, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 260, x: 53, y: 178, width: 12, height: 18, x_offset: 0, y_offset: -4, x_advance: 12 },
	Glyph { id: 261, x: 230, y: 112, width: 9, height: 15, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 262, x: 204, y: 237, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 263, x: 98, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 264, x: 240, y: 237, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 265, x: 179, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 266, x: 105, y: 178, width: 11, height: 18, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 267, x: 143, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 268, x: 84, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 269, x: 170, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 270, x: 96, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 271, x: 198, y: 144, width: 13, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 272, x: 178, y: 81, width: 13, height: 14, x_offset: 0, y_offset: 0, x_advance: 13 },
	Glyph { id: 273, x: 239, y: 146, width: 11, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 274, x: 133, y: 160, width: 8, height: 17, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 275, x: 219, y: 66, width: 9, height: 14, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 276, x: 11, y: 156, width: 8, height: 18, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 277, x: 10, y: 91, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 278, x: 247, y: 179, width: 8, height: 18, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 279, x: 20, y: 91, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 280, x: 47, y: 158, width: 8, height: 18, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 281, x: 60, y: 94, width: 9, height: 15, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 282, x: 118, y: 197, width: 8, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 283, x: 40, y: 109, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 284, x: 152, y: 237, width: 12, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 285, x: 209, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 286, x: 27, y: 177, width: 12, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 287, x: 198, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 288, x: 40, y: 177, width: 12, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 289, x: 187, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 290, x: 165, y: 237, width: 12, height: 19, x_offset: 1, y_offset: -4, x_advance: 14 },
	Glyph { id: 291, x: 176, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 292, x: 24, y: 216, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 293, x: 81, y: 197, width: 9, height: 19, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 294, x: 118, y: 81, width: 14, height: 14, x_offset: 0, y_offset: 0, x_advance: 14 },
	Glyph { id: 295, x: 0, y: 123, width: 11, height: 15, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 296, x: 38, y: 158, width: 8, height: 18, x_offset: -1, y_offset: 0, x_advance: 5 },
	Glyph { id: 297, x: 206, y: 96, width: 8, height: 15, x_offset: -1, y_offset: 0, x_advance: 4 },
	Glyph { id: 298, x: 182, y: 160, width: 6, height: 17, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 299, x: 249, y: 68, width: 6, height: 14, x_offset: 0, y_offset: 0, x_advance: 4 },
	Glyph { id: 300, x: 56, y: 159, width: 6, height: 18, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 301, x: 249, y: 163, width: 6, height: 15, x_offset: 0, y_offset: 0, x_advance: 4 },
	Glyph { id: 302, x: 63, y: 159, width: 4, height: 18, x_offset: 0, y_offset: -4, x_advance: 5 },
	Glyph { id: 303, x: 193, y: 197, width: 4, height: 19, x_offset: 0, y_offset: -4, x_advance: 4 },
	Glyph { id: 304, x: 68, y: 159, width: 2, height: 18, x_offset: 1, y_offset: 0, x_advance: 5 },
	Glyph { id: 305, x: 78, y: 36, width: 2, height: 11, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 306, x: 174, y: 160, width: 7, height: 17, x_offset: 1, y_offset: -3, x_advance: 10 },
	Glyph { id: 307, x: 153, y: 197, width: 7, height: 19, x_offset: 1, y_offset: -4, x_advance: 9 },
	Glyph { id: 308, x: 0, y: 234, width: 8, height: 22, x_offset: -1, y_offset: -3, x_advance: 5 },
	Glyph { id: 309, x: 161, y: 197, width: 7, height: 19, x_offset: -1, y_offset: -4, x_advance: 4 },
	Glyph { id: 310, x: 236, y: 179, width: 10, height: 18, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 311, x: 41, y: 197, width: 9, height: 19, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 312, x: 245, y: 54, width: 9, height: 11, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 313, x: 136, y: 197, width: 8, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 314, x: 188, y: 197, width: 4, height: 19, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 315, x: 20, y: 156, width: 8, height: 18, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 316, x: 252, y: 237, width: 3, height: 19, x_offset: 0, y_offset: -4, x_advance: 4 },
	Glyph { id: 317, x: 197, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 318, x: 74, y: 79, width: 5, height: 15, x_offset: 1, y_offset: 0, x_advance: 4 },
	Glyph { id: 319, x: 19, y: 45, width: 8, height: 14, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 320, x: 250, y: 114, width: 5, height: 15, x_offset: 1, y_offset: 0, x_advance: 6 },
	Glyph { id: 321, x: 133, y: 66, width: 10, height: 14, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 322, x: 61, y: 78, width: 6, height: 15, x_offset: 0, y_offset: 0, x_advance: 5 },
	Glyph { id: 323, x: 72, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 324, x: 210, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 325, x: 189, y: 178, width: 11, height: 18, x_offset: 1, y_offset: -4, x_advance: 14 },
	Glyph { id: 326, x: 70, y: 111, width: 9, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 327, x: 36, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 328, x: 60, y: 110, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 329, x: 232, y: 81, width: 12, height: 14, x_offset: 0, y_offset: 0, x_advance: 13 },
	Glyph { id: 330, x: 110, y: 160, width: 11, height: 17, x_offset: 1, y_offset: -3, x_advance: 14 },
	Glyph { id: 331, x: 50, y: 110, width: 9, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 332, x: 71, y: 160, width: 13, height: 17, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 333, x: 177, y: 66, width: 10, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 334, x: 206, y: 198, width: 13, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 335, x: 189, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 336, x: 70, y: 237, width: 13, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 337, x: 178, y: 128, width: 10, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 338, x: 41, y: 142, width: 16, height: 15, x_offset: 1, y_offset: 0, x_advance: 17 },
	Glyph { id: 339, x: 147, y: 54, width: 16, height: 11, x_offset: 1, y_offset: 0, x_advance: 18 },
	Glyph { id: 340, x: 231, y: 217, width: 10, height: 19, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 341, x: 54, y: 78, width: 6, height: 15, x_offset: 1, y_offset: 0, x_advance: 7 },
	Glyph { id: 342, x: 225, y: 179, width: 10, height: 18, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 343, x: 16, y: 75, width: 7, height: 15, x_offset: 0, y_offset: -4, x_advance: 7 },
	Glyph { id: 344, x: 220, y: 217, width: 10, height: 19, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 345, x: 32, y: 75, width: 7, height: 15, x_offset: 1, y_offset: 0, x_advance: 7 },
	Glyph { id: 346, x: 51, y: 197, width: 9, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 347, x: 233, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 348, x: 31, y: 196, width: 9, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 349, x: 89, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 350, x: 21, y: 196, width: 9, height: 19, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 351, x: 107, y: 96, width: 8, height: 15, x_offset: 1, y_offset: -4, x_advance: 9 },
	Glyph { id: 352, x: 11, y: 194, width: 9, height: 19, x_offset: 1, y_offset: 0, x_advance: 10 },
	Glyph { id: 353, x: 125, y: 96, width: 8, height: 15, x_offset: 1, y_offset: 0, x_advance: 9 },
	Glyph { id: 354, x: 153, y: 178, width: 11, height: 18, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 355, x: 142, y: 160, width: 7, height: 17, x_offset: 0, y_offset: -4, x_advance: 6 },
	Glyph { id: 356, x: 0, y: 214, width: 11, height: 19, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 357, x: 8, y: 75, width: 7, height: 15, x_offset: 0, y_offset: 0, x_advance: 6 },
	Glyph { id: 358, x: 73, y: 63, width: 11, height: 14, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 359, x: 123, y: 52, width: 7, height: 13, x_offset: 0, y_offset: 0, x_advance: 6 },
	Glyph { id: 360, x: 177, y: 178, width: 11, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 361, x: 90, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 362, x: 98, y: 160, width: 11, height: 17, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 363, x: 229, y: 66, width: 9, height: 14, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 364, x: 129, y: 178, width: 11, height: 18, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 365, x: 120, y: 112, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 366, x: 23, y: 236, width: 11, height: 20, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 367, x: 11, y: 139, width: 9, height: 16, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 368, x: 48, y: 217, width: 11, height: 19, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 369, x: 241, y: 130, width: 9, height: 15, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 370, x: 165, y: 178, width: 11, height: 18, x_offset: 1, y_offset: -4, x_advance: 14 },
	Glyph { id: 371, x: 170, y: 112, width: 9, height: 15, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 372, x: 35, y: 237, width: 17, height: 19, x_offset: 0, y_offset: 0, x_advance: 17 },
	Glyph { id: 373, x: 106, y: 144, width: 15, height: 15, x_offset: 0, y_offset: 0, x_advance: 14 },
	Glyph { id: 374, x: 60, y: 217, width: 11, height: 19, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 375, x: 165, y: 217, width: 10, height: 19, x_offset: 0, y_offset: -4, x_advance: 9 },
	Glyph { id: 376, x: 117, y: 178, width: 11, height: 18, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 377, x: 154, y: 217, width: 10, height: 19, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 378, x: 152, y: 96, width: 8, height: 15, x_offset: 0, y_offset: 0, x_advance: 9 },
	Glyph { id: 379, x: 0, y: 156, width: 10, height: 18, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 380, x: 161, y: 96, width: 8, height: 15, x_offset: 0, y_offset: 0, x_advance: 9 },
	Glyph { id: 381, x: 143, y: 217, width: 10, height: 19, x_offset: 0, y_offset: 0, x_advance: 11 },
	Glyph { id: 382, x: 242, y: 98, width: 8, height: 15, x_offset: 0, y_offset: 0, x_advance: 9 },
	Glyph { id: 383, x: 47, y: 77, width: 6, height: 15, x_offset: 1, y_offset: 0, x_advance: 6 },
	Glyph { id: 402, x: 127, y: 197, width: 8, height: 19, x_offset: 1, y_offset: -4, x_advance: 11 },
	Glyph { id: 416, x: 122, y: 144, width: 15, height: 15, x_offset: 1, y_offset: 0, x_advance: 15 },
	Glyph { id: 417, x: 92, y: 52, width: 11, height: 13, x_offset: 1, y_offset: 0, x_advance: 11 },
	Glyph { id: 431, x: 169, y: 144, width: 14, height: 15, x_offset: 1, y_offset: 0, x_advance: 14 },
	Glyph { id: 432, x: 79, y: 49, width: 12, height: 13, x_offset: 1, y_offset: 0, x_advance: 12 },
	Glyph { id: 496, x: 145, y: 197, width: 7, height: 19, x_offset: -1, y_offset: -4, x_advance: 4 },
	Glyph { id: 506, x: 139, y: 237, width: 12, height: 19, x_offset: 0, y_offset: 0, x_advance: 12 },
	Glyph { id: 507, x: 71, y: 197, width: 9, height: 19, x_offset: 0, y_offset: 0, x_advance: 10 },
	Glyph { id: 508, x: 53, y: 237, width: 16, height: 19, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 509, x: 138, y: 144, width: 15, height: 15, x_offset: 0, y_offset: 0, x_advance: 16 },
	Glyph { id: 510, x: 9, y: 236, width: 13, height: 20, x_offset: 1, y_offset: -1, x_advance: 14 },
	Glyph { id: 511, x: 0, y: 139, width: 10, height: 16, x_offset: 1, y_offset: -1, x_advance: 11 },
	Glyph { id: 536, x: 61, y: 197, width: 9, height: 19, x_offset: 1, y_offset: -4, x_advance: 10 },
	Glyph { id: 537, x: 224, y: 96, width: 8, height: 15, x_offset: 1, y_offset: -4, x_advance: 9 },
	Glyph { id: 538, x: 201, y: 178, width: 11, height: 18, x_offset: 0, y_offset: -4, x_advance: 10 },
	Glyph { id: 539, x: 166, y: 160, width: 7, height: 17, x_offset: 0, y_offset: -4, x_advance: 6 },
	Glyph { id: 567, x: 68, y: 78, width: 5, height: 15, x_offset: -1, y_offset: -4, x_advance: 4 },
];

