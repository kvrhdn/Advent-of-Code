package particle

import "fmt"

type Vec3 struct {
	X, Y, Z int
}

func (v Vec3) String() string {
	return fmt.Sprintf("(%v, %v, %v)", v.X, v.Y, v.Z)
}

func (v *Vec3) Add(other Vec3) {
	v.X += other.X
	v.Y += other.Y
	v.Z += other.Z
}
