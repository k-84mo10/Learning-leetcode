package main
import "math"

func checkPerfectNumber(num int) bool {
    if num == 1 {
        return false
    }

    root := int(math.Sqrt(float64(num)))
    divisors := []int{1}
    for i := 2; i <= root; i++ {
        if num % i == 0 {
            divisors = append(divisors, i, num/i)
        }
    }

    sum := 0
    for _, divisor := range divisors {
        sum += divisor
    }

    return sum == num
}