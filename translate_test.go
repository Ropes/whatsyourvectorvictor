package whatsyourvectorvictor

import "testing"

func TestAlpha(t *testing.T) {
	x := TranslateRune('a')
	if x != "alpha" {
		t.Errorf("expecting 'alpha', got: '%s'", x)
	}
	y := TranslateRune('A')
	if y != "alpha" {
		t.Errorf("expecting 'alpha', got: '%s'", y)
	}
	t.Logf("x: %s", x)
}

func TestFoxtrot(t *testing.T) {
	f := TranslateRune('f')
	if f != "foxtrot" {
		t.Errorf("expecting foxtrot got '%s'", f)
	}
}
