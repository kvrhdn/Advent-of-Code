package particle

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestParse(t *testing.T) {
	input := `p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>`

	got := Parse(input)

	assert.Len(t, got, 2)
	assert.Equal(t, got[0], Particle{
		ID: 0,
		A:  Vec3{-1, 0, 0},
		V:  Vec3{2, 0, 0},
		P:  Vec3{3, 0, 0},
	})
	assert.Equal(t, got[1], Particle{
		ID: 1,
		A:  Vec3{-2, 0, 0},
		V:  Vec3{0, 0, 0},
		P:  Vec3{4, 0, 0},
	})
}

func TestWorld_Tick(t *testing.T) {
	input := `p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>`

	world := Parse(input)

	assert.Len(t, world, 2)
	assert.Equal(t, world[0], Particle{
		ID: 0,
		A:  Vec3{-1, 0, 0},
		V:  Vec3{2, 0, 0},
		P:  Vec3{3, 0, 0},
	})
	assert.Equal(t, world[1], Particle{
		ID: 1,
		A:  Vec3{-2, 0, 0},
		V:  Vec3{0, 0, 0},
		P:  Vec3{4, 0, 0},
	})

	world.Tick()

	assert.Len(t, world, 2)
	assert.Equal(t, world[0], Particle{
		ID: 0,
		A:  Vec3{-1, 0, 0},
		V:  Vec3{1, 0, 0},
		P:  Vec3{4, 0, 0},
	})
	assert.Equal(t, world[1], Particle{
		ID: 1,
		A:  Vec3{-2, 0, 0},
		V:  Vec3{-2, 0, 0},
		P:  Vec3{2, 0, 0},
	})
}

func TestWorld_RemoveCollisions(t *testing.T) {
	input := `p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>`

	world := Parse(input)
	assert.Len(t, world, 4)

	world.Tick()
	world.RemoveCollisions()
	assert.Len(t, world, 4)

	world.Tick()
	world.RemoveCollisions()
	assert.Len(t, world, 1)
}
