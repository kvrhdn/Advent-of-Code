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

func rotateLeft(v Vec2) Vec2 {
	return Vec2{
		x: -v.y,
		y: v.x,
	}
}

func rotateRight(v Vec2) Vec2 {
	return Vec2{
		x: v.y,
		y: -v.x,
	}
}
