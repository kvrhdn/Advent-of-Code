package day19

type Vec2 struct {
	x, y int
}

func sum(v1, v2 Vec2) Vec2 {
	return Vec2{
		x: v1.x + v2.x,
		y: v1.y + v2.y,
	}
}

func invert(v Vec2) Vec2 {
	return Vec2{
		x: -1 * v.x,
		y: -1 * v.y,
	}
}

func rotate(v Vec2) Vec2 {
	return Vec2{
		x: -1 * v.y,
		y: 1 * v.x,
	}
}
