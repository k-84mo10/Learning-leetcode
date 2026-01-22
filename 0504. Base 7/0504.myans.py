class Solution:
    def convertToBase7(self, num: int) -> str:
        if num == 0:
            return "0"
        
        sign = "-" if num < 0 else ""
        num = abs(num)

        digits = []
        while num > 0:
            num, rem = divmod(num, 7)
            digits.append(str(rem))

        return sign + "".join(reversed(digits))
