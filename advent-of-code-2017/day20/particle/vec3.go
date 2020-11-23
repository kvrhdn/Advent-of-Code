package particle

type Vec3 struct {
	x, y, z int
}

func (v *Vec3) add(other Vec3) {
	v.x += other.x
	v.y += other.y
	v.z += other.z
}
