func intToRoman(num int) string {
    digit := []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
    roman := []string{"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"}

    s := ""
    for i:=0; i<len(digit); i++{
        if num <= 0 {
            break
        }

        for num >= digit[i] {
            num -= digit[i]
            s += roman[i]
        }
    }
    return s
}