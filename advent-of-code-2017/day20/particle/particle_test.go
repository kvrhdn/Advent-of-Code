package particle

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParticle_Tick(t *testing.T) {
	p := Particle{
		ID:  0,
		acc: Vec3{-1, 0, 0},
		vel: Vec3{2, 0, 0},
		pos: Vec3{3, 0, 0},
	}

	p.tick()

	assert.Equal(t, Vec3{-1, 0, 0}, p.acc)
	assert.Equal(t, Vec3{1, 0, 0}, p.vel)
	assert.Equal(t, Vec3{4, 0, 0}, p.pos)

	p.tick()

	assert.Equal(t, Vec3{-1, 0, 0}, p.acc)
	assert.Equal(t, Vec3{0, 0, 0}, p.vel)
	assert.Equal(t, Vec3{4, 0, 0}, p.pos)
	p.tick()

	assert.Equal(t, Vec3{-1, 0, 0}, p.acc)
	assert.Equal(t, Vec3{-1, 0, 0}, p.vel)
	assert.Equal(t, Vec3{3, 0, 0}, p.pos)
}
