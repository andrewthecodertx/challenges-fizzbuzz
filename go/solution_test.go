package main

import (
	"reflect"
	"testing"
)

func TestFizzBuzz(t *testing.T) {
	result := FizzBuzz()

	expected := []string{
		"1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
		"11", "Fizz", "13", "14", "FizzBuzz",
	}

	if len(result) != 100 {
		t.Errorf("Expected 100 results, got %d", len(result))
	}

	if !reflect.DeepEqual(result[:15], expected) {
		t.Errorf("First 15 elements don't match. Got %v", result[:15])
	}
}
