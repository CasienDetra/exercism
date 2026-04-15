package raindrops

import "strconv"

var divisors = []struct {
	divisor int
	sound   string
}{
	{3, "Pling"}, {5, "Plang"}, {7, "Plong"},
}

func Convert(number int) string {
	var sound string

	for _, div := range divisors {
		if number%div.divisor == 0 {
			sound += div.sound
		}
	}

	if sound == "" {
		return strconv.Itoa(number)
	}
	return sound
}
