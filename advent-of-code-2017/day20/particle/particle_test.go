package particle

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestParticle_Tick(t *testing.T) {
	p := Particle{
		ID: 0,
		A:  Vec3{-1, 0, 0},
		V:  Vec3{2, 0, 0},
		P:  Vec3{3, 0, 0},
	}

	p.Tick()

	assert.Equal(t, p.A, Vec3{-1, 0, 0})
	assert.Equal(t, p.V, Vec3{1, 0, 0})
	assert.Equal(t, p.P, Vec3{4, 0, 0})

	p.Tick()

	assert.Equal(t, p.A, Vec3{-1, 0, 0})
	assert.Equal(t, p.V, Vec3{0, 0, 0})
	assert.Equal(t, p.P, Vec3{4, 0, 0})
	p.Tick()

	assert.Equal(t, p.A, Vec3{-1, 0, 0})
	assert.Equal(t, p.V, Vec3{-1, 0, 0})
	assert.Equal(t, p.P, Vec3{3, 0, 0})
}
