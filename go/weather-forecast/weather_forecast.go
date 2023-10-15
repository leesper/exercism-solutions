// Package weather forecasts the current weather of Goblinocus.
package weather

// CurrentCondition represents the current weather condition.
var CurrentCondition string

// CurrentLocation represents the location in Goblinocus.
var CurrentLocation string

// Forecast takes city and condition parameters and
// returns the corresponding weather condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
